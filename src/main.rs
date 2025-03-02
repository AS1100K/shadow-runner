use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkPlugin;
use shadow_runner::{camera::MainCameraPlugin, physics::PhysicsPlugin, BasePlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins(LdtkPlugin);
    app.add_plugins(BasePlugin);
    app.add_plugins(PhysicsPlugin);
    app.add_plugins(MainCameraPlugin);

    #[cfg(feature = "debug")]
    app.add_plugins(shadow_runner::editor::Editor);

    app.run();
}
