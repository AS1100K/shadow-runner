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
                    .load_collection::<FontAssets>()
                    .load_collection::<AudioAssets>()
                    .load_collection::<EntitySpriteAssets>(),
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
    #[asset(path = "Legacy_Adventure_Pack/Background.png")]
    pub background: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct EntitySpriteAssets {
    // Hostile Entity
    #[asset(texture_atlas_layout(
        tile_size_x = 16,
        tile_size_y = 16,
        columns = 4,
        rows = 1,
        padding_x = 0,
        padding_y = 0,
        offset_x = 0,
        offset_y = 0
    ))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "hostile/Sand Ghoul/SandGhoul.png")]
    pub sand_ghoul: Handle<Image>,
    #[asset(path = "hostile/Grave Revenant/GraveRevenant.png")]
    pub grave_revenant: Handle<Image>,
    #[asset(path = "hostile/Mutilated Stumbler/MutilatedStumbler.png")]
    pub mutilated_stumbler: Handle<Image>,
    #[asset(path = "hostile/Adept Necromancer/AdeptNecromancer.png")]
    pub adept_necromancer: Handle<Image>,

    // Player
    #[asset(texture_atlas_layout(
        tile_size_x = 16,
        tile_size_y = 16,
        columns = 24,
        rows = 1,
        padding_x = 0,
        padding_y = 0,
        offset_x = 0,
        offset_y = 0
    ))]
    pub player_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "character/running.png")]
    pub player_running: Handle<Image>,
    #[asset(path = "character/idle.png")]
    pub player_idle: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct IconsAssets {
    #[asset(path = "Legacy_Adventure_Pack/jump_booster.png")]
    pub jump_booster_icon: Handle<Image>,
    #[asset(path = "Legacy_Adventure_Pack/spike.png")]
    pub spike: Handle<Image>,
    #[asset(texture_atlas_layout(
        tile_size_x = 16,
        tile_size_y = 16,
        columns = 6,
        rows = 1,
        padding_x = 0,
        padding_y = 0,
        offset_x = 0,
        offset_y = 0
    ))]
    pub spike_layout: Handle<TextureAtlasLayout>,
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

#[derive(AssetCollection, Resource, Clone)]
pub struct AudioAssets {
    #[asset(path = "audio/Smooth Lovin.ogg")]
    pub smooth_lovin: Handle<AudioSource>,
    #[asset(path = "audio/I can Feel it Coming.ogg")]
    pub i_can_feel_it_coming: Handle<AudioSource>,
    #[asset(path = "audio/damage.ogg")]
    pub damage: Handle<AudioSource>,
    #[asset(path = "audio/button.ogg")]
    pub button: Handle<AudioSource>,
    #[asset(path = "audio/jump_boost.ogg")]
    pub jump_boost: Handle<AudioSource>,
}

fn update_game_state(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::MainMenuScreen);
}
