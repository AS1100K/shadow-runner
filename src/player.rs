use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PLAYER_WIDTH: f32 = 20.;
pub const PLAYER_HEIGHT: f32 = 100.;

#[derive(Bundle)]
pub struct Player {
    pub sprite: Sprite,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
    pub velocity: Velocity,
    pub transform: Transform,
    pub player_entity: PlayerEntity,
}

impl Player {
    pub fn new(window_width: f32, _window_height: f32) -> Self {
        Self {
            sprite: Sprite {
                // Red
                color: Color::hsl(0., 1., 0.5),
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..Default::default()
            },
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::cuboid(PLAYER_WIDTH / 2., PLAYER_HEIGHT / 2.),
            velocity: Velocity::linear(Vec2::new(100., 0.)),
            transform: Transform::from_xyz(-window_width / 2., 0., 0.),
            player_entity: PlayerEntity,
        }
    }
}

#[derive(Component)]
pub struct PlayerEntity;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

fn movement(
    mut query: Query<&mut Transform, With<PlayerEntity>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        // Up/Jump Movement
        if keys.pressed(KeyCode::KeyW)
            || keys.pressed(KeyCode::ArrowUp)
            || keys.pressed(KeyCode::Space)
        {
            transform.translation.y += 500. * time.delta_secs();
        }
    }
}
