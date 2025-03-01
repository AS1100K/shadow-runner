use bevy::prelude::*;
use player::{Player, PlayerPlugin};

pub mod camera;
#[cfg(feature = "debug")]
pub mod editor;
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

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default())
            .add_systems(Update, base_game_system)
            .add_plugins(EntitySpawnerPlugin);
    }
}

#[derive(Resource, Debug, Default)]
pub struct GameState {
    pub is_paused: bool,
}

fn base_game_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
    mut game_state: ResMut<GameState>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if game_state.is_paused {
            time.unpause();
        } else {
            time.pause();
        }
        game_state.is_paused = !game_state.is_paused;
    }
}
