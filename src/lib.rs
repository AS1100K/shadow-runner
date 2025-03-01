use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use player::{Player, PlayerPlugin, PLAYER_HEIGHT, PLAYER_WIDTH};

pub mod camera;
pub mod physics;
pub mod player;

pub struct EntitySpawnerPlugin;

impl Plugin for EntitySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_basic)
            .add_plugins(PlayerPlugin);
    }
}

fn spawn_basic(mut commands: Commands) {
    // Create a ground
    commands
        .spawn(Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(500., 500.)),
            ..Default::default()
        })
        .insert(Collider::cuboid(500. / 2., 500. / 2.))
        .insert(Transform::from_xyz(0., -200., 0.));

    // Create a Player
    commands
        .spawn(Sprite {
            color: Color::hsl(0., 100., 50.),
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            ..Default::default()
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(PLAYER_WIDTH / 2., PLAYER_HEIGHT / 2.))
        .insert(Transform::from_xyz(0., 400., 0.));
}
