use super::despawn_screen;
use crate::GameState;
use bevy::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PauseScreen), spawn_screen)
            .add_systems(
                OnExit(GameState::PauseScreen),
                despawn_screen::<OnPauseScreen>,
            );
    }
}

#[derive(Component)]
pub struct OnPauseScreen;

fn spawn_screen(mut commands: Commands) {
    commands
        .spawn((
            OnPauseScreen,
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
            parent.spawn((Text::new("Game Paused"), TextColor(Color::hsl(0., 1., 0.5))));
        });
}
