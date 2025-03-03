use super::despawn_screen;
use crate::{assets::FontAssets, GameState};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenuScreen), spawn_screen)
            .add_systems(
                OnExit(GameState::MainMenuScreen),
                despawn_screen::<OnMainMenuScreen>,
            )
            .add_systems(
                Update,
                start_game.run_if(in_state(GameState::MainMenuScreen)),
            );
    }
}

#[derive(Component)]
pub struct OnMainMenuScreen;

fn spawn_screen(mut commands: Commands, font_assets: Res<FontAssets>) {
    let font = &font_assets.default_font;

    commands
        .spawn((
            OnMainMenuScreen,
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
                Text::new("Press Enter to Start"),
                TextColor(Color::hsl(0., 1., 0.5)),
                TextFont {
                    font: font.clone(),
                    font_size: 70.,
                    ..default()
                },
            ));
        });
}

// This doesn't belong in loading screen
fn start_game(
    inputs: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if inputs.pressed(KeyCode::Enter) {
        next_game_state.set(GameState::PlayingScreen);
    }
}
