use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};
use player::PlayerPlugin;
use walls::WallPlugin;

pub mod camera;
pub mod colliders;
#[cfg(feature = "debug")]
pub mod editor;
pub mod physics;
pub mod player;
pub mod walls;

pub const GRID_SIZE: i32 = 16;

pub struct EntitySpawnerPlugin;

impl Plugin for EntitySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_basic)
            .add_plugins(WallPlugin)
            .add_plugins(PlayerPlugin);
    }
}

fn spawn_basic(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn LDTK Bundle
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("shadow_runner.ldtk").into(),
        ..default()
    });
}

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default())
            // Current Level Index
            .insert_resource(LevelSelection::index(0))
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
