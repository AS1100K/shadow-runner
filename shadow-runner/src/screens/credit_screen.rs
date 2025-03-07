use super::{despawn_screen, LevelsMenuButton, MainMenuButton};
use crate::{
    assets::{self, FontAssets},
    GameState,
};
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

fn spawn_screen(mut commands: Commands, font_assets: Res<FontAssets>, world: Res<assets::World>) {
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
            OnCreditScreen,
        ))
        .with_child(ImageNode {
            image: world.background.clone(),
            ..default()
        });

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
        ))
        .with_children(|parent| {
            // Thanks for Playing!
            parent.spawn((
                Text::new("Thanks for Playing!"),
                TextColor(Color::hsl(327., 0.24, 0.16)),
                TextFont {
                    font: font_assets.default_font.clone(),
                    font_size: 100.,
                    ..default()
                },
            ));

            // Author References
            parent.spawn((
                Text::new("Game built by AS1100K for `Code for Cause` Game Jam!"),
                TextColor(Color::hsl(327., 0.24, 0.16)),
                TextFont {
                    font: font_assets.default_font.clone(),
                    font_size: 33.,
                    ..default()
                },
            ));

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
                    BackgroundColor(Color::hsl(327., 0.24, 0.16)),
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
