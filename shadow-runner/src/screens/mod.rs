use crate::{
    assets::{AssetsLoadingState, AudioAssets},
    GameState,
};
use bevy::prelude::*;
use credit_screen::CreditScreenPlugin;
use game_over_screen::GameOverPlugin;
use levels_menu_screen::LevelsMenuPlugin;
use main_menu_screen::MainMenuPlugin;
use pause_screen::PausePlugin;

pub mod credit_screen;
pub mod game_over_screen;
pub mod levels_menu_screen;
pub mod loading_screen;
pub mod main_menu_screen;
pub mod pause_screen;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PausePlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(GameOverPlugin)
            .add_plugins(LevelsMenuPlugin)
            .add_plugins(CreditScreenPlugin)
            .add_systems(
                Update,
                (button_hover, back_to_main_menu, back_to_levels_menu)
                    .run_if(in_state(AssetsLoadingState::Loaded)),
            )
            .add_systems(OnExit(GameState::PlayingScreen), pause_game)
            .add_systems(OnEnter(GameState::PlayingScreen), unpause_game);
    }
}

#[derive(Component)]
pub struct MainMenuButton;

#[derive(Component)]
pub struct LevelsMenuButton;

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        info!("Despawning Screen Recursively");
        commands.entity(entity).despawn_recursive();
    }
}

#[allow(clippy::type_complexity)]
pub fn button_hover(
    query: Query<
        (Entity, &Interaction),
        (With<Button>, Changed<Interaction>, Without<AudioPlayer>),
    >,
    audio_assets: Res<AudioAssets>,
    mut commands: Commands,
) {
    for (entity, interaction) in &query {
        if Interaction::Hovered == *interaction {
            commands.entity(entity).insert((
                AudioPlayer(audio_assets.button.clone()),
                PlaybackSettings::REMOVE,
            ));
        }
    }
}

pub fn back_to_main_menu(
    query: Query<&Interaction, (With<MainMenuButton>, Changed<Interaction>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if Interaction::Pressed == *interaction {
            next_game_state.set(GameState::MainMenuScreen);
        }
    }
}

pub fn back_to_levels_menu(
    query: Query<&Interaction, (With<LevelsMenuButton>, Changed<Interaction>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if Interaction::Pressed == *interaction {
            next_game_state.set(GameState::LevelsMenuScreen);
        }
    }
}

pub fn pause_game(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

pub fn unpause_game(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}
