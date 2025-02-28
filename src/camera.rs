use bevy::prelude::*;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}
