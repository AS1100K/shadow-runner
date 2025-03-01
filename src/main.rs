use bevy::prelude::*;
use shadow_runner::{camera::MainCameraPlugin, physics::PhysicsPlugin, EntitySpawnerPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(PhysicsPlugin);
    app.add_plugins(EntitySpawnerPlugin);
    app.add_plugins(MainCameraPlugin);

    #[cfg(feature = "debug")]
    app.add_plugins(shadow_runner::editor::Editor);

    app.run();
}
