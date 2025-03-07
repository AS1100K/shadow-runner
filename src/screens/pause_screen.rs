use super::{despawn_screen, game_over_screen::RestartGameButton, MainMenuButton};
use crate::{
    assets::{FontAssets, IconsAssets},
    GameState,
};
use bevy::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PauseScreen), spawn_screen)
            .add_systems(
                OnExit(GameState::PauseScreen),
                despawn_screen::<OnPauseScreen>,
            )
            .add_systems(
                Update,
                resume_game_button.run_if(in_state(GameState::PauseScreen)),
            );
    }
}

#[derive(Component)]
pub struct OnPauseScreen;

#[derive(Component)]
pub struct ResumeGameButton;

fn spawn_screen(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    icon_assets: Res<IconsAssets>,
) {
    let font = &font_assets.default_font;
    let reset_icon = &icon_assets.reset_icon;

    commands
        .spawn((
            OnPauseScreen,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                column_gap: Val::Px(20.),
                left: Val::Px(0.),
                top: Val::Px(0.),
                ..default()
            },
            // White With 20% Opacity
            BackgroundColor(Color::hsla(0., 1., 1., 0.2)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Game Paused"),
                TextColor(Color::hsl(327., 0.24, 0.16)),
                TextFont {
                    font: font.clone(),
                    font_size: 100.,
                    ..default()
                },
            ));

            // Spawn Buttons
            parent
                .spawn(Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(50.),
                    ..default()
                })
                .with_children(|parent| {
                    // Spawn Resume Button
                    parent
                        .spawn((
                            ResumeGameButton,
                            Button,
                            Node {
                                width: Val::Px(300.),
                                height: Val::Px(100.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(25.),
                                ..default()
                            },
                            BackgroundColor(Color::hsl(327., 0.24, 0.16)),
                        ))
                        .with_child((
                            Text::new("Resume"),
                            // hsl(0, 0%, 88%)
                            TextColor(Color::hsl(0., 0., 0.88)),
                            TextFont {
                                font: font.clone(),
                                font_size: 33.,
                                ..default()
                            },
                        ));

                    // Spawn Restart Button
                    parent
                        .spawn((
                            RestartGameButton,
                            Button,
                            Node {
                                width: Val::Px(300.),
                                height: Val::Px(100.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(25.),
                                ..default()
                            },
                            BackgroundColor(Color::hsl(327., 0.24, 0.16)),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(Node {
                                    width: Val::Px(50.),
                                    height: Val::Px(50.),
                                    ..default()
                                })
                                .with_child(ImageNode {
                                    image: reset_icon.clone(),
                                    ..default()
                                });

                            parent.spawn((
                                Text::new("Restart"),
                                // hsl(0, 0%, 88%)
                                TextColor(Color::hsl(0., 0., 0.88)),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 33.,
                                    ..default()
                                },
                            ));
                        });

                    // Spawn Main Menu Button
                    parent
                        .spawn((
                            MainMenuButton,
                            Button,
                            Node {
                                width: Val::Px(300.),
                                height: Val::Px(100.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(25.),
                                ..default()
                            },
                            BackgroundColor(Color::hsl(327., 0.24, 0.16)),
                        ))
                        .with_child((
                            Text::new("Main Menu"),
                            // hsl(0, 0%, 88%)
                            TextColor(Color::hsl(0., 0., 0.88)),
                            TextFont {
                                font: font.clone(),
                                font_size: 33.,
                                ..default()
                            },
                        ));
                });
        });
}

fn resume_game_button(
    query: Query<&Interaction, (With<ResumeGameButton>, Changed<Interaction>)>,
    mut input: ResMut<ButtonInput<KeyCode>>,
) {
    for interaction in &query {
        if Interaction::Pressed == *interaction {
            input.press(KeyCode::Escape);
        }
    }
}
