use super::components::*;
use bevy::prelude::*;

pub fn spawn(mut commands: Commands, windows: Query<&Window>) {
    for window in &windows {
        let width = window.width();
        let height = window.height();

        // Create Floor
        commands.spawn(Floor::new(width, height));

        // Creata a Box
        commands.spawn(Box::new(width * 0.1, height * 0.1, -width / 4., 0., 0.));
    }
}
