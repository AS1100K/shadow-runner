use bevy::prelude::*;
use shadown_runner::camera::MainCameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainCameraPlugin)
        .run();
}
