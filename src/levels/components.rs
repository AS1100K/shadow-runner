use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct Floor {
    pub sprite: Sprite,
    pub collider: Collider,
    pub transform: Transform,
    pub friction: Friction,
    #[cfg(feature = "debug")]
    pub clickable: crate::editor::Clickable,
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
            #[cfg(feature = "debug")]
            clickable: crate::editor::Clickable("Floor"),
        }
    }
}

#[derive(Bundle)]
pub struct Box {
    pub sprite: Sprite,
    pub collider: Collider,
    pub transform: Transform,
    pub friction: Friction,
    #[cfg(feature = "debug")]
    pub clickable: crate::editor::Clickable,
}

impl Box {
    /// Creates a basic cuboid at `0, 0, 0` with 10% size of window's width and height
    pub fn new_default(window_width: f32, window_height: f32) -> Self {
        Self::new(window_height * 0.1, window_width * 0.1, 0., 0., 0.)
    }

    pub fn new(width: f32, height: f32, x: f32, y: f32, z: f32) -> Self {
        Self {
            sprite: Sprite {
                // Green
                color: Color::hsl(150., 0.77, 0.5),
                custom_size: Some(Vec2::new(width, height)),
                ..Default::default()
            },
            collider: Collider::cuboid(width / 2., height / 2.),
            transform: Transform::from_xyz(x, y, z),
            friction: Friction {
                coefficient: 0.,
                combine_rule: CoefficientCombineRule::Min,
            },
            #[cfg(feature = "debug")]
            clickable: crate::editor::Clickable("Box"),
        }
    }
}
