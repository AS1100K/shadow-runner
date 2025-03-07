use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::LdtkPlugin;
use shadow_runner::{camera::MainCameraPlugin, physics::PhysicsPlugin, BasePlugin};

fn main() {
    let mut app = App::new();

    #[allow(unused_mut)]
    let mut default_plugins = DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                resolution: WindowResolution::new(1280., 720.),
                title: String::from("Shadow Runner"),
                ..default()
            }),
            ..default()
        })
        .set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        });

    #[cfg(target_os = "windows")]
    {
        use bevy::render::{
            settings::{Backends, RenderCreation, WgpuSettings},
            RenderPlugin,
        };

        default_plugins = default_plugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::DX12),
                ..default()
            }),
            synchronous_pipeline_compilation: false,
        });
    }

    app.add_plugins(default_plugins);
    app.add_plugins(LdtkPlugin);
    app.add_plugins(BasePlugin);
    app.add_plugins(PhysicsPlugin);
    app.add_plugins(MainCameraPlugin);

    app.run();
}
