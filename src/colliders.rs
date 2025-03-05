use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, LdtkIntCell};
use bevy_rapier2d::prelude::*;

use crate::{
    player::{PLAYER_HEIGHT, PLAYER_WIDTH},
    GRID_SIZE,
};

#[derive(Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(value: &EntityInstance) -> Self {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match value.identifier.as_ref() {
            "Player" => ColliderBundle {
                rigid_body: RigidBody::Dynamic,
                collider: Collider::cuboid(PLAYER_WIDTH / 2., PLAYER_HEIGHT / 2.),
                friction: Friction {
                    coefficient: 0.,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ..default()
            },
            "Sand_Ghoul" | "Grave_Revenant" | "Mutilated_Stumbler" => ColliderBundle {
                rigid_body: RigidBody::KinematicVelocityBased,
                collider: Collider::cuboid(GRID_SIZE as f32 / 2., GRID_SIZE as f32 / 2.),
                rotation_constraints,
                ..default()
            },
            _ => ColliderBundle::default(),
        }
    }
}
