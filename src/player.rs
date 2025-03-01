use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PLAYER_WIDTH: f32 = 20.;
pub const PLAYER_HEIGHT: f32 = 100.;

#[derive(Component)]
#[require(Velocity, Transform)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

fn movement(
    mut query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        // Right Movement
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 100. * time.delta_secs();
        }

        // Left Movement
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 100. * time.delta_secs();
        }

        // Up/Jump Movement
        if keys.pressed(KeyCode::KeyW)
            || keys.pressed(KeyCode::ArrowUp)
            || keys.pressed(KeyCode::Space)
        {
            transform.translation.y += 100. * time.delta_secs();
        }
    }
}
