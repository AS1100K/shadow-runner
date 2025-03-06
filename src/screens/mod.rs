use bevy::prelude::*;
use game_over_screen::GameOverPlugin;
use levels_menu_screen::LevelsMenuPlugin;
use main_menu_screen::MainMenuPlugin;
use pause_screen::PausePlugin;

use crate::assets::{AssetsLoadingState, AudioAssets};

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
            .add_systems(
                Update,
                button_hover.run_if(in_state(AssetsLoadingState::Loaded)),
            );
    }
}

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
