use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;

pub struct AssetsManagerPlugin;

impl Plugin for AssetsManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(AssetsLoadingState::Loading)
            .add_loading_state(
                LoadingState::new(AssetsLoadingState::Loading)
                    .continue_to_state(AssetsLoadingState::Loaded)
                    .load_collection::<World>()
                    .load_collection::<IconsAssets>()
                    .load_collection::<FontAssets>(),
            )
            .add_systems(
                OnTransition {
                    exited: AssetsLoadingState::Loading,
                    entered: AssetsLoadingState::Loaded,
                },
                update_game_state,
            );
    }
}

// This State is managed by screens/loading_screen.rs
// and is used to load ldtk map into the world
#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AssetsLoadingState {
    Loading,
    Loaded,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct World {
    #[asset(path = "shadow_runner.ldtk")]
    pub ldtk: Handle<LdtkProject>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct IconsAssets {
    #[asset(path = "icons/icon_reset.png")]
    pub reset_icon: Handle<Image>,
    #[asset(path = "icons/icon_heart.png")]
    pub heart_icon: Handle<Image>,
    #[asset(path = "keyboard/keyboard_W.png")]
    pub keyboard_w: Handle<Image>,
    #[asset(path = "keyboard/keyboard_A.png")]
    pub keyboard_a: Handle<Image>,
    #[asset(path = "keyboard/keyboard_S.png")]
    pub keyboard_s: Handle<Image>,
    #[asset(path = "keyboard/keyboard_D.png")]
    pub keyboard_d: Handle<Image>,
    #[asset(path = "keyboard/keyboard_SpaceBar_1.png")]
    pub keyboard_spacebar_1: Handle<Image>,
    #[asset(path = "keyboard/keyboard_SpaceBar_2.png")]
    pub keyboard_spacebar_2: Handle<Image>,
    #[asset(path = "keyboard/keyboard_SpaceBar_3.png")]
    pub keyboard_spacebar_3: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct FontAssets {
    #[asset(path = "fonts/RasterForge.ttf")]
    pub default_font: Handle<Font>,
}

fn update_game_state(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::MainMenuScreen);
}
