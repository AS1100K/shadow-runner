use super::{despawn_screen, LevelsMenuButton, MainMenuButton};
use crate::{assets::FontAssets, GameState};
use bevy::prelude::*;

pub struct CreditScreenPlugin;

impl Plugin for CreditScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CreditScreen), spawn_screen)
            .add_systems(
                OnExit(GameState::CreditScreen),
                despawn_screen::<OnCreditScreen>,
            );
    }
}

#[derive(Component)]
pub struct OnCreditScreen;

fn spawn_screen(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            OnCreditScreen,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Px(0.),
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|parent| {
            // Thanks for Playing!
            parent.spawn((
                Text::new("Thanks for Playing!"),
                TextColor::WHITE,
                TextFont {
                    font: font_assets.default_font.clone(),
                    font_size: 100.,
                    ..default()
                },
            ));

            // Social References
            parent.spawn((
                Text::new("Game built by AS1100K for `Code for Cause` Game Jam!"),
                TextColor::WHITE,
                TextFont {
                    font: font_assets.default_font.clone(),
                    font_size: 28.,
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("Source Code: https://github.com/as1100k/shadow-runner"),
                TextColor::WHITE,
                TextFont {
                    font: font_assets.default_font.clone(),
                    font_size: 28.,
                    ..default()
                },
            ));

            // Spawn Buttons
            // parent
            //     .spawn(Node {
            //         justify_content: JustifyContent::Center,
            //         align_items: AlignItems::Center,
            //         ..default()
            //     })
            //     .with_children(|parent| {

            //     });

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
                    BackgroundColor(Color::hsl(31., 0.72, 0.46)),
                ))
                .with_child((
                    Text::new("Main Menu"),
                    // hsl(0, 0%, 88%)
                    TextColor(Color::hsl(0., 0., 0.88)),
                    TextFont {
                        font: font_assets.default_font.clone(),
                        font_size: 33.,
                        ..default()
                    },
                ));

            // Spawn Levels Menu Button
            parent
                .spawn((
                    LevelsMenuButton,
                    Button,
                    Node {
                        width: Val::Px(300.),
                        height: Val::Px(100.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(25.),
                        ..default()
                    },
                    BackgroundColor(Color::hsl(31., 0.72, 0.46)),
                ))
                .with_child((
                    Text::new("Levels Menu"),
                    // hsl(0, 0%, 88%)
                    TextColor(Color::hsl(0., 0., 0.88)),
                    TextFont {
                        font: font_assets.default_font.clone(),
                        font_size: 33.,
                        ..default()
                    },
                ));
        });
}
