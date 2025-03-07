use crate::{utils::Maybe, GameState};
use bevy::prelude::*;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animation_sprite.run_if(in_state(GameState::PlayingScreen)),
        )
        .add_systems(Update, animate_image_node);
    }
}

#[derive(States, Debug, Hash, Clone, Eq, PartialEq)]
pub struct EmptyState;

#[derive(Component, Default)]
pub struct AnimationIndice {
    pub indices: Vec<usize>,
    pub current_index: usize,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
struct AnimationTypeSprite;

#[derive(Component)]
struct AnimationTypeImageNode;

#[derive(Bundle)]
pub struct Animation {
    pub animation_indice: AnimationIndice,
    pub animation_timer: AnimationTimer,
    type_sprite: Maybe<AnimationTypeSprite>,
    type_image_node: Maybe<AnimationTypeImageNode>,
}

impl Animation {
    pub fn new(start_indice: usize, last_indice: usize, timer: Timer) -> Self {
        let mut indices = Vec::new();

        for i in start_indice..=last_indice {
            indices.push(i);
        }

        Self {
            animation_indice: AnimationIndice {
                indices,
                current_index: 0,
            },
            animation_timer: AnimationTimer(timer),
            type_sprite: Maybe::new(AnimationTypeSprite),
            type_image_node: Maybe::NONE,
        }
    }

    pub fn new_with_custom_tiles(tiles: Vec<usize>, timer: Timer) -> Self {
        Self {
            animation_indice: AnimationIndice {
                indices: tiles,
                current_index: 0,
            },
            animation_timer: AnimationTimer(timer),
            type_sprite: Maybe::new(AnimationTypeSprite),
            type_image_node: Maybe::NONE,
        }
    }

    pub fn new_image_node(start_indice: usize, last_indice: usize, timer: Timer) -> Self {
        let mut indices = Vec::new();

        for i in start_indice..=last_indice {
            indices.push(i);
        }

        Self {
            animation_indice: AnimationIndice {
                indices,
                current_index: 0,
            },
            animation_timer: AnimationTimer(timer),
            type_sprite: Maybe::NONE,
            type_image_node: Maybe::new(AnimationTypeImageNode),
        }
    }

    pub fn new_image_node_with_custom_tiles(tiles: Vec<usize>, timer: Timer) -> Self {
        Self {
            animation_indice: AnimationIndice {
                indices: tiles,
                current_index: 0,
            },
            animation_timer: AnimationTimer(timer),
            type_sprite: Maybe::NONE,
            type_image_node: Maybe::new(AnimationTypeImageNode),
        }
    }
}

// See copied system animate_image_node
fn animation_sprite(
    time: Res<Time>,
    mut query: Query<
        (&mut AnimationIndice, &mut AnimationTimer, &mut Sprite),
        With<AnimationTypeSprite>,
    >,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.indices[indices.indices.len() - 1] {
                    indices.current_index = 0;
                    indices.indices[indices.current_index]
                } else {
                    indices.current_index += 1;
                    if indices.current_index >= indices.indices.len() {
                        indices.current_index = 0;
                    }
                    indices.indices[indices.current_index]
                }
            }
        }
    }
}

// This system is just a copy of animation_sprite system and the only change
// here is it changes the NodeImage instead of Sprite
fn animate_image_node(
    time: Res<Time>,
    mut query: Query<
        (&mut AnimationIndice, &mut AnimationTimer, &mut ImageNode),
        With<AnimationTypeImageNode>,
    >,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.indices[indices.indices.len() - 1] {
                    indices.current_index = 0;
                    indices.indices[indices.current_index]
                } else {
                    indices.current_index += 1;
                    if indices.current_index >= indices.indices.len() {
                        indices.current_index = 0;
                    }
                    indices.indices[indices.current_index]
                }
            }
        }
    }
}

// fn update_animation(mut query: Query<(&mut Sprite, &AnimationIndice), Changed<AnimationIndice>>) {
//     for (mut sprite, indices) in &mut query {
//         if let Some(atlas) = &mut sprite.texture_atlas {
//             atlas.index = indices.indices[0];
//         }
//     }
// }
