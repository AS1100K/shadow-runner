use bevy::prelude::*;
use game_over_screen::GameOverPlugin;
use loading_screen::LoadingPlugin;
use pause_screen::PausePlugin;

pub mod game_over_screen;
pub mod loading_screen;
pub mod pause_screen;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PausePlugin)
            .add_plugins(LoadingPlugin)
            .add_plugins(GameOverPlugin);
    }
}

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
