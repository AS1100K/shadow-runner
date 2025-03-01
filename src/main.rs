use bevy::prelude::*;
use shadow_runner::{camera::MainCameraPlugin, physics::PhysicsPlugin, EntitySpawnerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(EntitySpawnerPlugin)
        .add_plugins(MainCameraPlugin)
        .run();
}
