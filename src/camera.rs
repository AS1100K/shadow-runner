use bevy::prelude::*;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);

        #[cfg(feature = "debug")]
        app.add_systems(Update, camera_movement);
    }
}

#[derive(Component)]
#[require(Camera2d, Transform)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

#[cfg(feature = "debug")]
fn camera_movement(
    mut query: Query<&mut Transform, With<MainCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut camera in &mut query {
        if keys.pressed(KeyCode::Numpad6) {
            camera.translation.x += 10.;
        }

        if keys.pressed(KeyCode::Numpad8) {
            camera.translation.y += 10.;
        }

        if keys.pressed(KeyCode::Numpad4) {
            camera.translation.x -= 10.;
        }

        if keys.pressed(KeyCode::Numpad2) {
            camera.translation.y -= 10.;
        }

        if keys.just_pressed(KeyCode::Numpad5) {
            camera.translation.x = 0.;
            camera.translation.y = 0.;
        }
    }
}
