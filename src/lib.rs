use assets::AssetsManagerPlugin;
use bevy::prelude::*;
use bevy::utils::{Duration, Instant};
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use bevy::window::WindowMode;
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
            .insert_resource(Time::<Fixed>::from_seconds(1.))
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
            .add_systems(
                Update,
                (
                    auto_despawn_system,
                    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
                    full_screen,
                )
                    .run_if(in_state(assets::AssetsLoadingState::Loaded)),
            )
            .add_systems(
                OnEnter(assets::AssetsLoadingState::Loaded),
                play_game_background_music,
            );
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
    CreditScreen,
}

fn base_game_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match *game_state.get() {
            GameState::PauseScreen => {
                next_game_state.set(GameState::PlayingScreen);
            }
            GameState::PlayingScreen => {
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

fn play_game_background_music(mut commands: Commands, audio_assets: Res<assets::AudioAssets>) {
    commands.spawn((
        AudioPlayer(audio_assets.smooth_lovin.clone()),
        PlaybackSettings::LOOP,
    ));
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn full_screen(keyboard: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if keyboard.just_pressed(KeyCode::F11) {
        for mut window in &mut windows {
            match window.mode {
                WindowMode::Fullscreen(_) => {
                    window.mode = WindowMode::Windowed;
                }
                _ => {
                    window.mode = WindowMode::Fullscreen(MonitorSelection::Current);
                }
            }
        }
    }
}
