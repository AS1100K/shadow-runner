use crate::{time::RestartTimeEvent, GameState};
use better_default::Default;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::BTreeMap;

pub struct LevelManager;

impl Plugin for LevelManager {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevelInfo::default())
            .insert_resource(AllLevels::default())
            .add_systems(
                Update,
                sync_level_changes.run_if(in_state(GameState::PlayingScreen)),
            );
    }
}

#[derive(Resource, Default)]
#[default(current_level_id: 0)]
pub struct CurrentLevelInfo {
    pub current_level_id: i32,
}

#[derive(Resource, Debug, Clone, Default)]
#[default(all_levels: BTreeMap::from([
    (0, String::from("e052a7f2-e920-11ef-9cc5-79226502923b")),
    (1, String::from("e6572c40-e920-11ef-a264-afdec05b9c0d")),
    // TODO: Update Levels laters once they are created
    (2, String::from("")),
    (3, String::from("")),
    (4, String::from("")),
    (5, String::from("")),
    (6, String::from("")),
    (7, String::from("")),
    (8, String::from("")),
    (9, String::from("")),
    (10, String::from("")),
    (11, String::from("")),
]))]
pub struct AllLevels {
    /// i32 -> Level Number
    /// String -> Level Iid
    ///
    /// ## Example
    ///
    /// ```ron
    /// {
    ///     0: "e052a7f2-e920-11ef-9cc5-79226502923b",
    ///     1: "e6572c40-e920-11ef-a264-afdec05b9c0d",
    /// }
    /// ```
    pub all_levels: BTreeMap<i32, String>,
}

fn sync_level_changes(
    mut commands: Commands,
    current_level_info: Res<CurrentLevelInfo>,
    all_levels: Res<AllLevels>,
    mut restart_time_event: EventWriter<RestartTimeEvent>,
) {
    if current_level_info.is_changed() {
        log::info!("Inserting level {}", current_level_info.current_level_id);
        if let Some(level_iid) = all_levels
            .all_levels
            .get(&current_level_info.current_level_id)
        {
            commands.insert_resource(LevelSelection::iid(level_iid));
            restart_time_event.send(RestartTimeEvent);
        } else {
            log::error!("Level didn't found, make sure the ldtk map is syned with the default implementation.");
        }
    }
}
