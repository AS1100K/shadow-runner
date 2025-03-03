use crate::{player::PlayerEntity, ASPECT_RATIO};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

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
    mut camera_query: Query<
        (&mut Transform, &mut OrthographicProjection),
        (With<MainCamera>, Without<PlayerEntity>),
    >,
    player_query: Query<&Transform, With<PlayerEntity>>,
    level_query: Query<
        (&Transform, &LevelIid),
        (Without<OrthographicProjection>, Without<PlayerEntity>),
    >,
    ldtk_projects: Query<&LdtkProjectHandle>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut camera_transform, mut orthographic_projection) = camera_query.single_mut();

        for (level_transform, level_iid) in &level_query {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_projects.single())
                .expect("Project should be loaded if level has spawned");

            let level = ldtk_project
                .get_raw_level_by_iid(&level_iid.to_string())
                .expect("Spawned level should exist in LDtk Project");

            if level_selection.is_match(&LevelIndices::default(), level) {
                let level_ratio = level.px_wid as f32 / level.px_hei as f32;
                orthographic_projection.viewport_origin = Vec2::ZERO;

                if level_ratio > ASPECT_RATIO {
                    // level is wider than the screen
                    let height = (level.px_hei as f32 / 9.).round() * 9.;
                    let width = height * ASPECT_RATIO;
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    camera_transform.translation.x =
                        (player_translation.x - level_transform.translation.x - width / 2.)
                            .clamp(0., level.px_wid as f32 - width);
                    camera_transform.translation.y = 0.;
                } else {
                    // level is taller than the screen
                    let width = (level.px_wid as f32 / 16.).round() * 16.;
                    let height = width / ASPECT_RATIO;
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    camera_transform.translation.y =
                        (player_translation.y - level_transform.translation.y - height / 2.)
                            .clamp(0., level.px_hei as f32 - height);
                    camera_transform.translation.x = 0.;
                }

                camera_transform.translation.x += level_transform.translation.x;
                camera_transform.translation.y += level_transform.translation.y;
            }
        }
    }
}

#[cfg(feature = "debug")]
fn camera_movement(
    mut query: Query<(&mut Transform, &mut MainCamera, &mut OrthographicProjection)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time<bevy::prelude::Real>>,
) {
    for (mut camera_transform, mut main_camera, mut orthographic_projection) in &mut query {
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

        if keys.just_pressed(KeyCode::NumpadAdd) {
            orthographic_projection.scale -= 0.1;
        }

        if keys.just_pressed(KeyCode::NumpadSubtract) {
            orthographic_projection.scale += 0.1;
        }
    }
}
