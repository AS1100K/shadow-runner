use crate::{player::PlayerEntity, ASPECT_RATIO};
use bevy::{
    prelude::*,
    render::camera::{ScalingMode, Viewport},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (sync_camera, update_camera_viewport));
    }
}

#[derive(Component, Default)]
#[require(Camera2d, Transform, Velocity)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    let mut orthographic_project = OrthographicProjection::default_2d();
    orthographic_project.scaling_mode = ScalingMode::AutoMin {
        min_width: 1280.,
        min_height: 720.,
    };

    commands.spawn(MainCamera).insert(OrthographicProjection {
        scaling_mode: ScalingMode::AutoMin {
            min_width: 1280.,
            min_height: 720.,
        },
        ..OrthographicProjection::default_2d()
    });
}

fn update_camera_viewport(
    window_query: Query<&Window, Changed<Window>>,
    mut camera_query: Query<&mut Camera, With<MainCamera>>,
) {
    for window in &window_query {
        let size = window.size();

        for mut camera in &mut camera_query {
            let aspect_ratio = size.x / size.y;
            let viewport_width = if aspect_ratio > ASPECT_RATIO {
                size.y * ASPECT_RATIO
            } else {
                size.x
            };
            let viewport_height = if aspect_ratio > ASPECT_RATIO {
                size.y
            } else {
                size.x / ASPECT_RATIO
            };

            camera.viewport = Some(Viewport {
                physical_position: UVec2::new(
                    ((size.x - viewport_width) / 2.0) as u32,
                    ((size.y - viewport_height) / 2.0) as u32,
                ),
                physical_size: UVec2::new(viewport_width as u32, viewport_height as u32),
                ..Default::default()
            });
        }
    }
}

#[allow(clippy::type_complexity)]
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
    level_selection: Option<Res<LevelSelection>>,
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

            let Some(level_selection) = level_selection.as_ref() else {
                return;
            };

            if level_selection.is_match(&LevelIndices::default(), level) {
                let level_ratio = level.px_wid as f32 / level.px_hei as f32;
                orthographic_projection.viewport_origin = Vec2::ZERO;

                if level_ratio > ASPECT_RATIO {
                    // level is wider than the screen
                    let height = (level.px_hei as f32 / 9.).round() * 9.;
                    let width = height * ASPECT_RATIO;
                    camera_transform.translation.x =
                        (player_translation.x - level_transform.translation.x - width / 2.)
                            .clamp(0., level.px_wid as f32 - width);
                    camera_transform.translation.y = 0.;
                } else {
                    // level is taller than the screen
                    let width = (level.px_wid as f32 / 16.).round() * 16.;
                    let height = width / ASPECT_RATIO;
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
