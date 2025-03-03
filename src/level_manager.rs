use better_default::Default;
use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::prelude::*;

use crate::{assets, GameState};

pub struct LevelManager;

impl Plugin for LevelManager {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevelInfo::default())
            .add_systems(
                OnEnter(assets::AssetsLoadingState::Loaded),
                initialize_all_levels_res,
            )
            .add_systems(
                Update,
                sync_level_changes.run_if(in_state(GameState::PlayingScreen)),
            );
    }
}

#[derive(Resource, Default)]
#[default(current_level_uid: 0)]
pub struct CurrentLevelInfo {
    current_level_uid: usize,
}

#[derive(Resource, Debug)]
pub struct AllLevels {
    /// i32 -> UID
    /// String -> iid
    ///
    /// ## Example
    ///
    /// ```ron
    /// {
    ///     0: "e052a7f2-e920-11ef-9cc5-79226502923b",
    /// }
    /// ```
    pub all_levels: HashMap<i32, String>,
}

fn initialize_all_levels_res(
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    world: Res<assets::World>,
    mut commands: Commands,
) {
    let ldtk = &world.ldtk;

    let ldtk_project = ldtk_project_assets
        .get(LdtkProjectHandle {
            handle: ldtk.clone(),
        })
        .expect("Assets must be loaded by now");

    let root_levels: Vec<(i32, String)> = ldtk_project
        .root_levels()
        .iter()
        .map(|level| (level.uid, level.iid.clone()))
        .collect();

    let mut all_levels = HashMap::new();
    for level in root_levels {
        all_levels.insert(level.0, level.1);
    }

    log::debug!("All Levels: {:#?}", all_levels);

    commands.insert_resource(AllLevels { all_levels });
}

fn sync_level_changes(mut commands: Commands, current_level_info: Res<CurrentLevelInfo>) {
    if current_level_info.is_changed() {
        log::info!("Inserting level {}", current_level_info.current_level_uid);
        commands.insert_resource(LevelSelection::index(0));
    }
}
