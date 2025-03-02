use crate::colliders::ColliderBundle;
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
    #[cfg(feature = "debug")]
    pub clickable: crate::editor::Clickable,
}

#[derive(Default, Component)]
pub struct PlayerEntity;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<Player>("Player")
            .add_systems(Update, player_movement);
    }
}

// TODO: Fix Infinite Jump
fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<PlayerEntity>>,
) {
    for mut velocity in &mut query {
        let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 200.;

        if input.just_pressed(KeyCode::Space) {
            velocity.linvel.y = 400.;
        }
    }
}
