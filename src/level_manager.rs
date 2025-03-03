use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;

use crate::GameState;

pub struct LevelManager;

impl Plugin for LevelManager {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                exited: GameState::MainMenuScreen,
                entered: GameState::PlayingScreen,
            },
            load_level,
        );
    }
}

// TODO: Make it trigger from a event to change level as well as integrate
// with Start Button, Level Menu Button, Next Level, etc
fn load_level(mut commands: Commands, level: Option<Res<LevelSelection>>) {
    log::info!("Inserting level 0");
    if level.is_none() {
        commands.insert_resource(LevelSelection::index(0));
    }
}
