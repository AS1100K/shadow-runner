use crate::{
    assets::{AudioAssets, EntitySpriteAssets, IconsAssets},
    camera::MainCamera,
    colliders::ColliderBundle,
    ground_detection::{GroundDetection, GroundDetectionPlugin},
    level_manager::CurrentLevelInfo,
    screens::despawn_screen,
    sprite_animation::Animation,
    time::RecordTimeEvent,
    GameState, GRID_SIZE,
};
use bevy::utils::Duration;
use bevy::{color::palettes::css::YELLOW, prelude::*, time::common_conditions::on_real_timer};
use bevy_ecs_ldtk::prelude::*;
use bevy_light_2d::prelude::{AmbientLight2d, PointLight2d};
use bevy_rapier2d::prelude::Velocity;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<Player>("Player")
            .add_plugins(GroundDetectionPlugin)
            .add_systems(OnEnter(GameState::PlayingScreen), spawn_healthbar)
            .add_systems(
                OnExit(GameState::PlayingScreen),
                despawn_screen::<HealthBarContext>,
            )
            .add_systems(
                Update,
                (
                    player_movement,
                    sync_healthbar,
                    handle_player_animation,
                    update_blindness,
                )
                    .run_if(in_state(GameState::PlayingScreen)),
            )
            .add_systems(
                Update,
                add_blindness.run_if(
                    in_state(GameState::PlayingScreen)
                        .and(on_real_timer(Duration::from_secs_f32(0.5))),
                ),
            )
            .add_systems(
                FixedUpdate,
                continue_taking_damage.run_if(in_state(GameState::PlayingScreen)),
            )
            .add_systems(
                OnTransition {
                    exited: GameState::PlayingScreen,
                    entered: GameState::CreditScreen,
                },
                despawn_blindness,
            )
            .add_systems(
                OnTransition {
                    exited: GameState::PlayingScreen,
                    entered: GameState::MainMenuScreen,
                },
                despawn_blindness,
            );
    }
}

pub const PLAYER_WIDTH: f32 = 16.;
pub const PLAYER_HEIGHT: f32 = 32.;

#[derive(better_default::Default, Bundle, LdtkEntity)]
#[default(animation: Animation::new_with_custom_tiles(vec![1, 4, 7, 10, 13, 16, 19], Timer::from_seconds(0.25, TimerMode::Repeating)))]
pub struct Player {
    #[sprite_sheet]
    pub sprite: Sprite,
    #[grid_coords]
    pub grid_coords: GridCoords,
    pub player_entity: PlayerEntity,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub ground_detection: GroundDetection,
    pub health_bar: HealthBar,
    pub player_state: PlayerState,
    pub animation: Animation,
}

#[derive(Default, Component)]
pub struct PlayerEntity;

#[derive(Default, Component, PartialEq, Eq)]
pub enum PlayerState {
    #[default]
    Idle,
    Running,
}

#[derive(better_default::Default, Component)]
#[default(health: 6)]
pub struct HealthBar {
    pub health: u8,
}

/// Represents that an Entity is blind
#[derive(Default, Component)]
pub struct Blinded(pub Timer);

// TODO: Add auto-snip to the diagonal tiles
fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Sprite, &GroundDetection), With<PlayerEntity>>,
) {
    for (mut velocity, mut player_sprite, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::KeyD) {
            player_sprite.flip_x = false;
            1.
        } else {
            0.
        };
        let left = if input.pressed(KeyCode::KeyA) {
            player_sprite.flip_x = true;
            1.
        } else {
            0.
        };

        velocity.linvel.x = (right - left) * 200.;

        if input.just_pressed(KeyCode::Space) && ground_detection.on_ground {
            velocity.linvel.y = 400.;
        }
    }
}

#[allow(clippy::type_complexity)]
fn handle_player_animation(
    mut query: Query<
        (&mut Sprite, &mut PlayerState, &Velocity),
        (With<PlayerEntity>, Changed<Velocity>),
    >,
    entity_sprite_assets: Res<EntitySpriteAssets>,
) {
    for (mut sprite, mut player_state, player_velocity) in &mut query {
        if player_velocity.linvel.x == 0. && (-1.0..=1.0).contains(&player_velocity.linvel.y) {
            // Idle State
            if PlayerState::Idle != *player_state {
                *player_state = PlayerState::Idle;
                sprite.image = entity_sprite_assets.player_idle.clone();
            }
        } else {
            // Running State
            if PlayerState::Running != *player_state {
                *player_state = PlayerState::Running;
                sprite.image = entity_sprite_assets.player_running.clone();
            }
        }
    }
}

#[derive(Component)]
pub struct HealthBarContext;

#[derive(Component)]
#[require(HealthBar)]
pub struct ContinueTakingDamage(pub u8);

fn spawn_healthbar(
    mut commands: Commands,
    health_bar_query: Query<&HealthBar, With<PlayerEntity>>,
    icons_assets: Res<IconsAssets>,
) {
    commands
        .spawn((
            Node {
                top: Val::Px(50.),
                right: Val::Px(10.),
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.),
                ..default()
            },
            HealthBarContext,
        ))
        .with_children(|parent| {
            for health_bar in &health_bar_query {
                for _ in 0..health_bar.health {
                    parent
                        .spawn(Node {
                            width: Val::Px(30.),
                            height: Val::Px(30.),
                            ..default()
                        })
                        .with_child(ImageNode {
                            image: icons_assets.heart_icon.clone(),
                            ..default()
                        });
                }
            }
        });
}

#[allow(clippy::too_many_arguments)]
fn sync_healthbar(
    health_bar_query: Query<&HealthBar, (With<PlayerEntity>, Changed<HealthBar>)>,
    health_bar_context_query: Query<Entity, With<HealthBarContext>>,
    icons_assets: Res<IconsAssets>,
    mut commands: Commands,
    current_level_info: Res<CurrentLevelInfo>,
    mut record_time_event: EventWriter<RecordTimeEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut time: ResMut<Time<Virtual>>,
) {
    for health_bar in &health_bar_query {
        if health_bar.health == 0 {
            // Game Over
            record_time_event.send(RecordTimeEvent(current_level_info.current_level_id));
            next_game_state.set(GameState::GameOverScreen);
            time.pause();
            return;
        }

        for health_bar_context in &health_bar_context_query {
            let mut health_bar_context_commands = commands.entity(health_bar_context);

            // Remove the old hearts
            health_bar_context_commands.despawn_descendants();

            // Generate Sprite Bundle with all the hearts
            health_bar_context_commands.with_children(|parent| {
                // Spawn Health Icons
                for _ in 0..health_bar.health {
                    parent
                        .spawn(Node {
                            width: Val::Px(30.),
                            height: Val::Px(30.),
                            ..default()
                        })
                        .with_child(ImageNode {
                            image: icons_assets.heart_icon.clone(),
                            ..default()
                        });
                }
            });
        }
    }
}

fn continue_taking_damage(
    mut query: Query<
        (
            Entity,
            &mut HealthBar,
            &ContinueTakingDamage,
            Option<&AudioPlayer>,
        ),
        With<PlayerEntity>,
    >,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
) {
    for (entity, mut health_bar, damage_count, audio_player) in &mut query {
        if health_bar.health > damage_count.0 {
            if audio_player.is_none() {
                commands.entity(entity).insert((
                    AudioPlayer(audio_assets.damage.clone()),
                    PlaybackSettings::REMOVE,
                ));
            }
            health_bar.health -= 1;
        } else {
            health_bar.health = 0;
        }
    }
}

fn add_blindness(
    main_camera_query: Query<Entity, With<MainCamera>>,
    player_query: Query<Entity, (With<PlayerEntity>, Added<Blinded>)>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    current_level_info: Res<CurrentLevelInfo>,
) {
    for player in &player_query {
        log::info!("Adding Blindness");
        let light_color = if current_level_info.current_level_id >= 7 {
            Color::WHITE
        } else {
            Color::Srgba(YELLOW)
        };

        commands.entity(player).insert(PointLight2d {
            radius: GRID_SIZE as f32 * 4.,
            color: light_color,
            intensity: 0.8,
            ..default()
        });

        // Spawn Background Music
        commands.spawn((
            AudioPlayer(audio_assets.i_can_feel_it_coming.clone()),
            PlaybackSettings::REMOVE,
        ));

        for main_camera in &main_camera_query {
            commands.entity(main_camera).insert(AmbientLight2d {
                brightness: 0.,
                ..default()
            });
        }
    }
}

fn update_blindness(
    mut player_query: Query<(Entity, &mut Blinded), With<PlayerEntity>>,
    main_camera_query: Query<Entity, With<MainCamera>>,
    time: Res<Time<Virtual>>,
    mut commands: Commands,
    current_level_info: Res<CurrentLevelInfo>,
) {
    for (player, mut blinded) in &mut player_query {
        if current_level_info.is_changed() {
            log::info!("Removing Blindness because level has changed");
            commands
                .entity(player)
                .remove::<Blinded>()
                .insert(PointLight2d {
                    intensity: 1.,
                    ..default()
                });

            for main_camera in &main_camera_query {
                commands.entity(main_camera).remove::<AmbientLight2d>();
            }

            return;
        }

        blinded.0.tick(time.delta());

        if blinded.0.finished() {
            log::info!("Removing Blindness");
            commands
                .entity(player)
                .remove::<Blinded>()
                .insert(PointLight2d {
                    intensity: 1.,
                    ..default()
                });

            for main_camera in &main_camera_query {
                commands.entity(main_camera).remove::<AmbientLight2d>();
            }
        }
    }
}

// Similar to update_blindness system
fn despawn_blindness(
    player_query: Query<Entity, With<PlayerEntity>>,
    main_camera_query: Query<Entity, With<MainCamera>>,
    mut commands: Commands,
) {
    for player in &player_query {
        log::info!("Removing Blindness");
        commands
            .entity(player)
            .remove::<Blinded>()
            .insert(PointLight2d {
                intensity: 1.,
                ..default()
            });

        for main_camera in &main_camera_query {
            commands.entity(main_camera).remove::<AmbientLight2d>();
        }
    }
}
