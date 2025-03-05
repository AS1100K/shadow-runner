use crate::{
    colliders::ColliderBundle,
    player::{ContinueTakingDamage, HealthBar, PlayerEntity},
    GameState,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};
use bevy_rapier2d::prelude::{ActiveEvents, CollisionEvent, Velocity};

pub struct HostilePlugin;

impl Plugin for HostilePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<Hostile>("Sand_Ghoul")
            .add_systems(
                Update,
                (patrol, damage_player).run_if(in_state(GameState::PlayingScreen)),
            );
    }
}

#[derive(better_default::Default, Bundle, LdtkEntity)]
#[default(active_events: ActiveEvents::COLLISION_EVENTS)]
pub struct Hostile {
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    #[ldtk_entity]
    pub patrol: Patrol,
    pub active_events: ActiveEvents,
    hostile_entity: HostileEntity,
}

#[derive(Default, Component)]
pub struct HostileEntity;

#[derive(Component, Debug, Default)]
pub struct Patrol {
    pub points: Vec<Vec2>,
    pub index: usize,
    pub forward: bool,
}

impl LdtkEntity for Patrol {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        let mut points = Vec::new();
        points.push(ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        ));

        let ldtk_patrol_points = entity_instance
            .iter_points_field("patrol")
            .expect("patrol field should be correctrly typed for hostile entities");

        for ldtk_point in ldtk_patrol_points {
            let pixel_coords = (ldtk_point.as_vec2() + Vec2::new(0.5, 0.5))
                * Vec2::splat(layer_instance.grid_size as f32);

            points.push(ldtk_pixel_coords_to_translation_pivoted(
                pixel_coords.as_ivec2(),
                layer_instance.c_hei * layer_instance.grid_size,
                IVec2::new(entity_instance.width, entity_instance.height),
                entity_instance.pivot,
            ));
        }

        Self {
            points,
            index: 1,
            forward: true,
        }
    }
}

pub fn patrol(mut query: Query<(&mut Transform, &mut Velocity, &mut Patrol)>) {
    for (mut transform, mut velocity, mut patrol) in &mut query {
        if patrol.points.len() <= 1 {
            continue;
        }

        let mut new_velocity =
            (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * 50.;

        if new_velocity.dot(velocity.linvel) < 0. {
            if patrol.index == 0 {
                patrol.forward = true;
            } else if patrol.index == patrol.points.len() - 1 {
                patrol.forward = false;
            }

            transform.translation.x = patrol.points[patrol.index].x;
            transform.translation.y = patrol.points[patrol.index].y;

            if patrol.forward {
                patrol.index += 1;
            } else {
                patrol.index -= 1;
            }

            new_velocity =
                (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * 75.;
        }

        velocity.linvel = new_velocity;
    }
}

fn damage_player(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut HealthBar), With<PlayerEntity>>,
    hostile_query: Query<Entity, With<HostileEntity>>,
    mut commands: Commands,
) {
    for collision in collision_events.read() {
        let (player_entity, mut player_healtbar) = player_query.single_mut();
        match collision {
            &CollisionEvent::Started(entity_one, entity_two, ..) => {
                if entity_one == player_entity || entity_two == player_entity {
                    for hostile_entity in &hostile_query {
                        if entity_two == hostile_entity || entity_one == hostile_entity {
                            log::info!("Got damage from hostile entity");
                            player_healtbar.health -= 1;
                            commands.entity(player_entity).insert(ContinueTakingDamage);
                            return;
                        }
                    }
                }
            }
            &CollisionEvent::Stopped(entity_one, entity_two, ..) => {
                if entity_one == player_entity || entity_two == player_entity {
                    for hostile_entity in &hostile_query {
                        if entity_two == hostile_entity || entity_one == hostile_entity {
                            log::info!("Removing Continuous Damage");
                            commands
                                .entity(player_entity)
                                .remove::<ContinueTakingDamage>();
                            return;
                        }
                    }
                }
            }
        }
    }
}
