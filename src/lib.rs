use assets::AssetsManagerPlugin;
use bevy::prelude::*;
use bevy::utils::{Duration, Instant};
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_light_2d::plugin::Light2dPlugin;
use hostile_entity::HostilePlugin;
use level_manager::LevelManager;
use player::PlayerPlugin;
use screens::ScreensPlugin;
use special_tiles::SpecialTilesPlugin;
use sprite_animation::SpriteAnimationPlugin;
use time::TimeTakenPlugin;
use tutorial::GameTutorialPlugin;
use walls::WallPlugin;

pub mod assets;
pub mod camera;
pub mod colliders;
pub mod ground_detection;
pub mod hostile_entity;
pub mod level_manager;
pub mod physics;
pub mod player;
pub mod screens;
pub mod special_tiles;
pub mod sprite_animation;
pub mod time;
pub mod tutorial;
pub mod utils;
pub mod walls;

pub const GRID_SIZE: i32 = 16;
pub const ASPECT_RATIO: f32 = 16. / 9.;

pub struct EntitySpawnerPlugin;

impl Plugin for EntitySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(assets::AssetsLoadingState::Loaded), spawn_basic)
            .add_plugins(WallPlugin)
            .add_plugins(SpecialTilesPlugin)
            .add_plugins(Light2dPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(HostilePlugin)
            .add_plugins(SpriteAnimationPlugin);
    }
}

fn spawn_basic(mut commands: Commands, world: Res<assets::World>) {
    log::info!("Loading LDTK Bundle");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: world.ldtk.clone().into(),
        ..default()
    });
}

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::default())
            .insert_resource(Time::<Fixed>::from_seconds(0.5))
            .add_plugins(AssetsManagerPlugin)
            .add_plugins(ScreensPlugin)
            .add_plugins(LevelManager)
            .add_plugins(GameTutorialPlugin)
            .add_plugins(TimeTakenPlugin)
            .add_plugins(EntitySpawnerPlugin)
            .add_systems(
                Update,
                base_game_system.run_if(in_state(assets::AssetsLoadingState::Loaded)),
            )
            .add_systems(Update, auto_despawn_system);
    }
}

#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    PauseScreen,
    GameOverScreen,
    #[default]
    LoadingScreen,
    MainMenuScreen,
    LevelsMenuScreen,
    PlayingScreen,
}

fn base_game_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut time_taken_res: ResMut<time::TimeTakenRes>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match *game_state.get() {
            GameState::PauseScreen => {
                time_taken_res.stopwatch.unpause();
                time.unpause();
                next_game_state.set(GameState::PlayingScreen);
            }
            GameState::PlayingScreen => {
                time_taken_res.stopwatch.pause();
                time.pause();
                next_game_state.set(GameState::PauseScreen);
            }
            _ => {}
        }
    }
}

#[derive(Component, better_default::Default)]
#[default(instant: Instant::now(), duration: Duration::from_secs(5), recursive_despawn: true)]
pub struct AutoDespawn {
    instant: Instant,
    duration: Duration,
    recursive_despawn: bool,
}

impl AutoDespawn {
    pub fn new(duration: Duration) -> Self {
        Self {
            instant: Instant::now(),
            duration,
            recursive_despawn: false,
        }
    }

    pub fn new_recursive_despawn(duration: Duration) -> Self {
        Self {
            instant: Instant::now(),
            duration,
            recursive_despawn: true,
        }
    }
}

pub fn auto_despawn_system(mut commands: Commands, query: Query<(Entity, &AutoDespawn)>) {
    for (entity, auto_despawn) in &query {
        if auto_despawn.instant.elapsed() > auto_despawn.duration {
            if auto_despawn.recursive_despawn {
                commands.entity(entity).despawn_recursive();
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}
