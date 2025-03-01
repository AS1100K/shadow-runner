use bevy::prelude::*;
use player::{Player, PlayerPlugin};

pub mod camera;
pub mod levels;
pub mod physics;
pub mod player;

pub struct EntitySpawnerPlugin;

impl Plugin for EntitySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_basic)
            .add_systems(Startup, levels::one::spawn)
            .add_plugins(PlayerPlugin);
    }
}

fn spawn_basic(mut commands: Commands, windows: Query<&Window>) {
    for window in &windows {
        let width = window.width();
        let height = window.height();

        // Create a Player
        commands.spawn(Player::new(width, height));
    }
}
