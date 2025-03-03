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
                levels_screen.run_if(in_state(GameState::MainMenuScreen)),
            );
    }
}

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct StartGameButton;

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
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(50.),
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(0.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Shadow Runner"),
                TextColor(Color::hsl(0., 1., 0.5)),
                TextFont {
                    font: font.clone(),
                    font_size: 70.,
                    ..default()
                },
            ));

            // Spawn Start Button
            parent
                .spawn((
                    Button,
                    StartGameButton,
                    Node {
                        width: Val::Px(250.),
                        height: Val::Px(100.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::hsl(31., 0.72, 0.46)),
                ))
                .with_child((
                    Text::new("Start"),
                    // hsl(0, 0%, 88%)
                    TextColor(Color::hsl(0., 0., 0.88)),
                    TextFont {
                        font: font.clone(),
                        font_size: 33.,
                        ..default()
                    },
                ));
        });
}

fn levels_screen(
    query: Query<&Interaction, (With<StartGameButton>, Changed<Interaction>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if Interaction::Pressed == *interaction {
            next_game_state.set(GameState::LevelsMenuScreen);
        }
    }
}
