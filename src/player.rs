use crate::{
    assets::{AssetsLoadingState, IconsAssets},
    colliders::ColliderBundle,
    ground_detection::{GroundDetection, GroundDetectionPlugin},
    screens::despawn_screen,
    GameState,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub const PLAYER_WIDTH: f32 = 16.;
pub const PLAYER_HEIGHT: f32 = 32.;

#[derive(Default, Bundle, LdtkEntity)]
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
}

#[derive(Default, Component)]
pub struct PlayerEntity;

#[derive(better_default::Default, Component)]
#[default(health: 6)]
pub struct HealthBar {
    pub health: u8,
}

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
            );
    }
}

// TODO: Add auto-snip to the diagonal tiles
fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection), With<PlayerEntity>>,
) {
    for (mut velocity, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 200.;

        if input.just_pressed(KeyCode::Space) && ground_detection.on_ground {
            velocity.linvel.y = 400.;
        }
    }
}

#[derive(Component)]
pub struct HealthBarContext;

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

fn sync_healthbar(
    health_bar_query: Query<&HealthBar, (With<PlayerEntity>, Changed<HealthBar>)>,
    health_bar_context_query: Query<Entity, With<HealthBarContext>>,
    icons_assets: Res<IconsAssets>,
    mut commands: Commands,
) {
    for health_bar in &health_bar_query {
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
