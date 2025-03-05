use crate::GameState;
use bevy::prelude::*;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animation_sprite.run_if(in_state(GameState::PlayingScreen)),
        );
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

#[derive(Bundle)]
pub struct Animation {
    pub animation_indice: AnimationIndice,
    pub animation_timer: AnimationTimer,
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
        }
    }

    pub fn new_with_custom_tiles(tiles: Vec<usize>, timer: Timer) -> Self {
        Self {
            animation_indice: AnimationIndice {
                indices: tiles,
                current_index: 0,
            },
            animation_timer: AnimationTimer(timer),
        }
    }
}

fn animation_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndice, &mut AnimationTimer, &mut Sprite)>,
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
