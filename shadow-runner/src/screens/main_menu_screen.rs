use super::{despawn_screen, LevelsMenuButton};
use crate::{
    assets::{self, FontAssets},
    GameState,
};
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
                exit_game.run_if(in_state(GameState::MainMenuScreen)),
            );
    }
}

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct ExitGameButton;

fn spawn_screen(mut commands: Commands, font_assets: Res<FontAssets>, world: Res<assets::World>) {
    let font = &font_assets.default_font;

    // Spawn Background
    commands
        .spawn((
            Node {
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                top: Val::Px(0.),
                left: Val::Px(0.),
                ..default()
            },
            // hsl(213, 71%, 35%)
            BackgroundColor(Color::hsl(213., 0.71, 0.35)),
            OnMainMenuScreen,
        ))
        .with_child(ImageNode {
            image: world.background.clone(),
            ..default()
        });

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
                // hsl(327, 24%, 16%)
                TextColor(Color::hsl(327., 0.24, 0.16)),
                TextFont {
                    font: font.clone(),
                    font_size: 100.,
                    ..default()
                },
            ));

            // Spawn Start Button
            parent
                .spawn((
                    Button,
                    LevelsMenuButton,
                    Node {
                        width: Val::Px(250.),
                        height: Val::Px(100.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::hsl(327., 0.24, 0.16)),
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

            // Spawn Exit Game Button
            parent
                .spawn((
                    Button,
                    ExitGameButton,
                    Node {
                        width: Val::Px(250.),
                        height: Val::Px(100.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::hsl(327., 0.24, 0.16)),
                ))
                .with_child((
                    Text::new("Exit"),
                    // hsl(0, 0%, 88%)
                    TextColor(Color::hsl(0., 0., 0.88)),
                    TextFont {
                        font: font.clone(),
                        font_size: 33.,
                        ..default()
                    },
                ));
        });

    // Spawn Author Text
    commands
        .spawn((
            OnMainMenuScreen,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.),
                left: Val::Px(10.),
                ..default()
            },
        ))
        .with_child((
            Text::new("Built by AS1100K for `Code for Cause` Game Jam"),
            TextColor::WHITE,
            TextFont {
                font: font.clone(),
                font_size: 28.,
                ..default()
            },
        ));
}

fn exit_game(
    query: Query<&Interaction, (With<ExitGameButton>, Changed<Interaction>)>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    for interaction in &query {
        if Interaction::Pressed == *interaction {
            app_exit_event.send(AppExit::Success);
        }
    }
}
