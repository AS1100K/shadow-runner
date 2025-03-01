use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::PlayerEntity;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, sync_camera);

        #[cfg(feature = "debug")]
        app.add_systems(Update, camera_movement);
    }
}

#[derive(Component)]
#[require(Camera2d, Transform, Velocity)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

fn sync_camera(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<PlayerEntity>)>,
    player_query: Query<&Transform, (With<PlayerEntity>, Without<MainCamera>)>,
) {
    for mut camera_transform in &mut camera_query {
        for player_transform in &player_query {
            camera_transform.translation.x = player_transform.translation.x;
        }
    }
}

#[cfg(feature = "debug")]
fn camera_movement(
    mut query: Query<&mut Transform, With<MainCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut camera in &mut query {
        if keys.pressed(KeyCode::Numpad6) {
            camera.translation.x += 500. * time.delta_secs();
        }

        if keys.pressed(KeyCode::Numpad8) {
            camera.translation.y += 500. * time.delta_secs();
        }

        if keys.pressed(KeyCode::Numpad4) {
            camera.translation.x -= 500. * time.delta_secs();
        }

        if keys.pressed(KeyCode::Numpad2) {
            camera.translation.y -= 500. * time.delta_secs();
        }

        if keys.just_pressed(KeyCode::Numpad5) {
            camera.translation.x = 0.;
            camera.translation.y = 0.;
        }
    }
}
