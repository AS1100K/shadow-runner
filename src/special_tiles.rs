use crate::{
    colliders::ColliderBundle, hostile_entity::DamageCount, player::PlayerEntity,
    sprite_animation::Animation, walls::GlobalWallEntity, GameState,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, LdtkIntCell};
use bevy_rapier2d::prelude::*;

pub struct SpecialTilesPlugin;

impl Plugin for SpecialTilesPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<JumpBooster>("Jump_Booster", 1)
            .register_ldtk_int_cell_for_layer::<Spike>("Spike", 1)
            .add_systems(
                Update,
                jump_booster_collision_event.run_if(in_state(GameState::PlayingScreen)),
            );
    }
}

#[derive(better_default::Default, Bundle, LdtkIntCell)]
#[default(active_events: ActiveEvents::COLLISION_EVENTS)]
pub struct JumpBooster {
    #[from_int_grid_cell]
    pub collider: ColliderBundle,
    pub entity: JumpBoosterEntity,
    pub active_events: ActiveEvents,
}

#[derive(better_default::Default, Component)]
#[default(boost_velocty: 500., boost_cap: 650.)]
pub struct JumpBoosterEntity {
    pub boost_velocty: f32,
    pub boost_cap: f32,
}

#[derive(better_default::Default, Bundle, LdtkIntCell)]
#[default(
    damage_count: DamageCount(1),
    animation: Animation::new(0, 5, Timer::from_seconds(0.25, TimerMode::Repeating)),
    active_events: ActiveEvents::COLLISION_EVENTS
)]
pub struct Spike {
    pub sprite: Sprite,
    pub damage_count: DamageCount,
    pub active_events: ActiveEvents,
    pub animation: Animation,
    // So, that colliders will be automatically setup by merging
    pub global_wall: GlobalWallEntity,
    pub spike_entity: SpikeEntity,
}

#[derive(Default, Component)]
pub struct SpikeEntity;

fn jump_booster_collision_event(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Velocity), With<PlayerEntity>>,
    jump_booster_query: Query<(Entity, &JumpBoosterEntity)>,
) {
    for collision in collision_events.read() {
        if let CollisionEvent::Started(entity_one, entity_two, ..) = *collision {
            let (player_entity, mut player_velocty) = player_query.single_mut();

            if entity_one == player_entity || entity_two == player_entity {
                for (jump_booster_entity, jump_booster) in &jump_booster_query {
                    if entity_two == jump_booster_entity || entity_one == jump_booster_entity {
                        let mut new_velocity = jump_booster.boost_velocty - player_velocty.linvel.y;

                        if new_velocity > jump_booster.boost_cap {
                            new_velocity = jump_booster.boost_cap;
                        }

                        player_velocty.linvel.y = new_velocity;
                        return;
                    }
                }
            }
        }
    }
}
