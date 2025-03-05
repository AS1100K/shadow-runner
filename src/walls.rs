use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    assets::AssetsLoadingState, level_manager::CurrentLevelInfo, player::PlayerEntity,
    time::RecordTimeEvent, GameState,
};

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_wall_collisions, read_collisions)
                .chain()
                .run_if(in_state(AssetsLoadingState::Loaded)),
        )
        .register_ldtk_int_cell::<Wall<WallEntity>>(1)
        .register_ldtk_int_cell::<Wall<OutOfWorldEntity>>(2)
        .register_ldtk_int_cell::<Wall<NextLevelEntity>>(3);
    }
}

// TODO: Directly implement LdtkIntCell Trait instead of macro
// as it will simplify spawn_wall_collisions and also
// introduce ability to remove sprite for some walls
#[derive(Default, Bundle, LdtkIntCell)]
pub struct Wall<C: Component + Default> {
    wall_entity: C,
    global_wall: GlobalWallEntity,
}

#[derive(Default, Component)]
pub struct OutOfWorldEntity;

#[derive(Default, Component)]
pub struct WallEntity;

#[derive(Default, Component)]
pub struct NextLevelEntity;

#[derive(Default, Component)]
pub struct GlobalWallEntity;

#[derive(Default, Component)]
pub struct NextLevelTrigger;

#[derive(Default, Component)]
pub struct OutOfWorldTrigger;

// This system is inspired from platformer example in `bevy_ecs_ldtk` and is modified
// to add specific components based on which IntCell was that wall of.
// Code: https://github.com/Trouv/bevy_ecs_ldtk/blob/main/examples/platformer/walls.rs#L32
#[allow(clippy::type_complexity)]
pub fn spawn_wall_collisions(
    mut commands: Commands,
    wall_query: Query<
        (
            &GridCoords,
            &Parent,
            Option<&OutOfWorldEntity>,
            Option<&NextLevelEntity>,
        ),
        Added<GlobalWallEntity>,
    >,
    parent_query: Query<&Parent, Without<GlobalWallEntity>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
        /// Represents the ID of the IntCell
        origin: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
        origin: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashMap<GridCoords, i32>> = HashMap::new();

    wall_query
        .iter()
        .for_each(|(&grid_coords, parent, out_of_world, next_level_entity)| {
            // An intgrid tile's direct parent will be a layer entity, not the level entity
            // To get the level entity, you need the tile's grandparent.
            // This is where parent_query comes in.
            if let Ok(grandparent) = parent_query.get(parent.get()) {
                let int_cell_id = match (out_of_world, next_level_entity) {
                    (Some(_), None) => 2,
                    (None, Some(_)) => 3,
                    (None, None) => 1,
                    _ => 3,
                };
                level_to_wall_locations
                    .entry(grandparent.get())
                    .or_default()
                    .insert(grid_coords, int_cell_id);
            }
        });

    if !wall_query.is_empty() {
        level_query.iter().for_each(|(level_entity, level_iid)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        let grid_coords = GridCoords { x, y };
                        let int_cell_here = level_walls.get(&grid_coords).copied();

                        match (plate_start, int_cell_here) {
                            (Some((_s, current_id)), Some(id)) if id == current_id => {
                                // Do nothing, continuing current plate
                            }
                            (Some((s, current_id)), Some(id)) if id != current_id => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                    origin: current_id,
                                });
                                plate_start = Some((x, id));
                            }
                            (Some((s, current_id)), None) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                    origin: current_id,
                                });
                                plate_start = None;
                            }
                            (None, Some(id)) => {
                                plate_start = Some((x, id));
                            }
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<Rect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                                origin: plate.origin,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        let mut entity = level.spawn_empty();
                        entity
                            .insert(Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction::new(1.0))
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());

                        if wall_rect.origin == 2 || wall_rect.origin == 3 {
                            info!("Subscribing to collision for {} Wall", wall_rect.origin);
                            entity.insert(ActiveEvents::COLLISION_EVENTS);
                        }

                        if wall_rect.origin == 2 {
                            entity.insert(OutOfWorldTrigger);
                        }

                        if wall_rect.origin == 3 {
                            entity.insert(NextLevelTrigger);
                        }
                    }
                });
            }
        });
    }
}

fn read_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<PlayerEntity>>,
    next_level_trigger_query: Query<Entity, With<NextLevelTrigger>>,
    out_of_world_trigger_query: Query<Entity, With<OutOfWorldTrigger>>,
    mut current_level_info: ResMut<CurrentLevelInfo>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut time: ResMut<Time<Virtual>>,
    mut record_time_event: EventWriter<RecordTimeEvent>,
) {
    for collision_event in collision_events.read() {
        if let &CollisionEvent::Started(entity_one, entity_two, ..) = collision_event {
            let player_entity = player_query.single();
            let next_level_trigger_entity = next_level_trigger_query.single();

            if entity_one == player_entity || entity_two == player_entity {
                log::info!("Player Collision Detected");

                if entity_two == next_level_trigger_entity
                    || entity_one == next_level_trigger_entity
                {
                    // Next Level
                    current_level_info.current_level_id += 1;
                    record_time_event.send(RecordTimeEvent(current_level_info.current_level_id));
                } else {
                    for out_of_world_entity in &out_of_world_trigger_query {
                        if entity_two == out_of_world_entity || entity_one == out_of_world_entity {
                            // Game Over...
                            record_time_event
                                .send(RecordTimeEvent(current_level_info.current_level_id));
                            next_game_state.set(GameState::GameOverScreen);
                            time.pause();
                            return;
                        }
                    }
                }
            }
        };
    }
}
