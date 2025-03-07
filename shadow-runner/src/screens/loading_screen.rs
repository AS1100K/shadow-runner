use super::despawn_screen;
use crate::GameState;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadingScreen), spawn_screen)
            .add_systems(
                OnExit(GameState::LoadingScreen),
                despawn_screen::<OnLoadingScreen>,
            );
    }
}

#[derive(Component)]
pub struct OnLoadingScreen;

fn spawn_screen(mut commands: Commands) {
    commands
        .spawn((
            OnLoadingScreen,
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
            parent.spawn((
                Text::new("Loading..."),
                TextColor(Color::hsl(0., 1., 0.5)),
                TextFont {
                    font_size: 100.,
                    ..default()
                },
            ));
        });
}
