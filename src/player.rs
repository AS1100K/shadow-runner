use crate::{
    assets::AssetsLoadingState,
    colliders::ColliderBundle,
    ground_detection::{GroundDetection, GroundDetectionPlugin},
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
}

#[derive(Default, Component)]
pub struct PlayerEntity;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<Player>("Player")
            .add_systems(
                Update,
                player_movement.run_if(in_state(AssetsLoadingState::Loaded)),
            )
            .add_plugins(GroundDetectionPlugin);
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
