use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct Floor {
    pub sprite: Sprite,
    pub collider: Collider,
    pub transform: Transform,
    pub friction: Friction,
}

impl Floor {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Self {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(window_width, window_height / 3.)),
                ..Default::default()
            },
            collider: Collider::cuboid(window_width / 2., window_height / 6.),
            transform: Transform::from_xyz(0., -window_height / 3., 0.),
            friction: Friction {
                coefficient: 0.,
                combine_rule: CoefficientCombineRule::Min,
            },
        }
    }
}

#[derive(Bundle)]
pub struct Box {
    pub sprite: Sprite,
    pub collider: Collider,
    pub transform: Transform,
    pub friction: Friction,
}

impl Box {
    // TODO: Allow to customize the Box Position, relative to floor
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Self {
            sprite: Sprite {
                // Green
                color: Color::hsl(150., 0.77, 0.5),
                custom_size: Some(Vec2::new(window_height * 0.1, window_height * 0.1)),
                ..Default::default()
            },
            collider: Collider::cuboid(window_height * 0.05, window_height * 0.05),
            transform: Transform::from_xyz(window_width * 0.5, -window_height * 0.1, 0.),
            friction: Friction {
                coefficient: 0.,
                combine_rule: CoefficientCombineRule::Min,
            },
        }
    }
}
