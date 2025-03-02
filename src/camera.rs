use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rapier2d::prelude::*;

use crate::{player::PlayerEntity, GRID_SIZE};

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, sync_camera);

        #[cfg(feature = "debug")]
        app.add_systems(Update, camera_movement);
    }
}

#[derive(Component, Default)]
#[require(Camera2d, Transform, Velocity)]
pub struct MainCamera {
    #[cfg(feature = "debug")]
    player_delta: f32,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(MainCamera::default());
}

fn sync_camera(
    #[cfg(feature = "debug")] mut camera_query: Query<
        (&mut Transform, &MainCamera),
        Without<PlayerEntity>,
    >,
    #[cfg(not(feature = "debug"))] mut camera_query: Query<
        &mut Transform,
        (With<MainCamera>, Without<PlayerEntity>),
    >,
    player_query: Query<Option<&GridCoords>, (With<PlayerEntity>, Without<MainCamera>)>,
) {
    #[cfg(feature = "debug")]
    let (mut camera_transform, camera) = camera_query.single_mut();
    #[cfg(not(feature = "debug"))]
    let mut camera_transform = camera_query.single_mut();

    for player_grid_coords in &player_query {
        let player_translation = if let Some(player_grid_coords) = player_grid_coords {
            bevy_ecs_ldtk::utils::grid_coords_to_translation(
                *player_grid_coords,
                IVec2::splat(GRID_SIZE),
            )
        } else {
            Vec2::new(0., 0.)
        };

        #[cfg(feature = "debug")]
        {
            camera_transform.translation.x = player_translation.x + camera.player_delta;
        }

        #[cfg(not(feature = "debug"))]
        {
            camera_transform.translation.x = player_translation.x;
        }
    }
}

#[cfg(feature = "debug")]
fn camera_movement(
    mut query: Query<(&mut Transform, &mut MainCamera)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time<bevy::prelude::Real>>,
) {
    for (mut camera_transform, mut main_camera) in &mut query {
        if keys.pressed(KeyCode::Numpad6) || keys.all_pressed([KeyCode::AltLeft, KeyCode::KeyD]) {
            main_camera.player_delta += 500. * time.delta_secs();
        }

        if keys.pressed(KeyCode::Numpad8) || keys.all_pressed([KeyCode::AltLeft, KeyCode::KeyW]) {
            camera_transform.translation.y += 500. * time.delta_secs();
        }

        if keys.pressed(KeyCode::Numpad4) || keys.all_pressed([KeyCode::AltLeft, KeyCode::KeyA]) {
            main_camera.player_delta -= 500. * time.delta_secs();
        }

        if keys.pressed(KeyCode::Numpad2) || keys.all_pressed([KeyCode::AltLeft, KeyCode::KeyS]) {
            camera_transform.translation.y -= 500. * time.delta_secs();
        }

        if keys.just_pressed(KeyCode::Numpad5)
            || keys.all_pressed([KeyCode::AltLeft, KeyCode::Space])
        {
            main_camera.player_delta = 0.;
            camera_transform.translation.y = 0.;
        }
    }
}
