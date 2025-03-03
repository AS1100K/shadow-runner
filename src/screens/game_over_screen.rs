use super::despawn_screen;
use crate::GameState;
use bevy::prelude::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOverScreen), spawn_screen)
            .add_systems(
                OnExit(GameState::GameOverScreen),
                despawn_screen::<OnGameOverScreen>,
            );
    }
}

#[derive(Component)]
pub struct OnGameOverScreen;

fn spawn_screen(mut commands: Commands) {
    commands
        .spawn((
            OnGameOverScreen,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(0.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((Text::new("Game Over"), TextColor(Color::hsl(0., 1., 0.5))));
        });
}
