use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_wall_collisions, read_events).chain())
            .register_ldtk_int_cell::<Wall<WallEntity>>(1)
            .register_ldtk_int_cell::<Wall<OutOfWorldEntity>>(2);
    }
}

#[derive(Default, Component)]
pub struct OutOfWorldEntity;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct Wall<C: Component + Default> {
    wall_entity: C,
    global_wall: GlobalWall,
}

#[derive(Default, Component)]
pub struct WallEntity;

#[derive(Default, Component)]
pub struct GlobalWall;

// This system is taken from platformer example in `bevy_ecs_ldtk`
// Code: https://github.com/Trouv/bevy_ecs_ldtk/blob/main/examples/platformer/walls.rs#L32
pub fn spawn_wall_collisions(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent, Option<&OutOfWorldEntity>), Added<GlobalWall>>,
    parent_query: Query<&Parent, Without<GlobalWall>>,
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
        .for_each(|(&grid_coords, parent, out_of_world)| {
            // An intgrid tile's direct parent will be a layer entity, not the level entity
            // To get the level entity, you need the tile's grandparent.
            // This is where parent_query comes in.
            if let Ok(grandparent) = parent_query.get(parent.get()) {
                let int_cell_id = if out_of_world.is_some() { 2 } else { 1 };
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

                        if wall_rect.origin == 2 {
                            // entity.insert(Sensor);
                            entity.insert(ActiveEvents::COLLISION_EVENTS);
                        }
                    }
                });
            }
        });
    }
}

fn read_events(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.read() {
        log::info!("Received collision event: {:?}", collision_event);
    }
}
