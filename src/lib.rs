use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;
use level_manager::LevelManager;
use player::PlayerPlugin;
use screens::ScreensPlugin;
use walls::WallPlugin;

pub mod camera;
pub mod colliders;
pub mod level_manager;
pub mod physics;
pub mod player;
pub mod screens;
pub mod walls;

pub const GRID_SIZE: i32 = 16;
pub const ASPECT_RATIO: f32 = 16. / 9.;

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
        app.insert_state(GameState::default())
            .add_plugins(ScreensPlugin)
            .add_plugins(LevelManager)
            // Current Level Index
            // .insert_resource(LevelSelection::index(0))
            .add_systems(Update, base_game_system)
            .add_plugins(EntitySpawnerPlugin);
    }
}

#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    PauseScreen,
    GameOverScreen,
    #[default]
    LoadingScreen,
    PlayingScreen,
}

fn base_game_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match *game_state.get() {
            GameState::PauseScreen => {
                time.unpause();
                next_game_state.set(GameState::PlayingScreen);
            }
            GameState::PlayingScreen => {
                time.pause();
                next_game_state.set(GameState::PauseScreen);
            }
            _ => {}
        }
    }
}
