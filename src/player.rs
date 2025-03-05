use crate::{
    assets::{AssetsLoadingState, IconsAssets},
    colliders::ColliderBundle,
    ground_detection::{GroundDetection, GroundDetectionPlugin},
    level_manager::CurrentLevelInfo,
    screens::despawn_screen,
    sprite_animation::Animation,
    time::RecordTimeEvent,
    GameState,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<Player>("Player")
            .add_plugins(GroundDetectionPlugin)
            .add_systems(
                Update,
                player_movement.run_if(in_state(AssetsLoadingState::Loaded)),
            )
            .add_systems(OnEnter(GameState::PlayingScreen), spawn_healthbar)
            .add_systems(
                OnExit(GameState::PlayingScreen),
                despawn_screen::<HealthBarContext>,
            )
            .add_systems(
                Update,
                sync_healthbar.run_if(in_state(GameState::PlayingScreen)),
            )
            .add_systems(
                FixedUpdate,
                continue_taking_damage.run_if(in_state(GameState::PlayingScreen)),
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

#[derive(Default, Component)]
pub enum PlayerState {
    #[default]
    Idle,
    Moving,
    Falling,
    Dead,
    Hurt,
}

#[derive(better_default::Default, Component)]
#[default(health: 6)]
pub struct HealthBar {
    pub health: u8,
}

// TODO: Add auto-snip to the diagonal tiles
fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection, &mut Sprite), With<PlayerEntity>>,
) {
    for (mut velocity, ground_detection, mut player_sprite) in &mut query {
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

#[derive(Component)]
pub struct HealthBarContext;

#[derive(Component)]
#[require(HealthBar)]
pub struct ContinueTakingDamage(pub u8);

fn spawn_healthbar(mut commands: Commands) {
    commands.spawn((
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
    ));
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

fn continue_taking_damage(mut query: Query<(&mut HealthBar, &ContinueTakingDamage)>) {
    for (mut health_bar, damage_count) in &mut query {
        if health_bar.health > damage_count.0 {
            health_bar.health -= 1;
        } else {
            health_bar.health = 0;
        }
    }
}
