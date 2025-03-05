use crate::{
    assets::{FontAssets, IconsAssets},
    level_manager::CurrentLevelInfo,
    screens::despawn_screen,
    AutoDespawn, GameState,
};
use bevy::prelude::*;

pub struct GameTutorialPlugin;

impl Plugin for GameTutorialPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(TutorialState::Finished)
            .insert_resource(TutorialInfo::default())
            .add_systems(
                OnTransition {
                    exited: GameState::LevelsMenuScreen,
                    entered: GameState::PlayingScreen,
                },
                set_tutorial_state,
            )
            .add_systems(
                OnEnter(TutorialState::OnGoing),
                spawn_basic_keycodes_overlay,
            )
            .add_systems(
                OnExit(TutorialState::OnGoing),
                despawn_screen::<TutorialParent>,
            )
            .add_systems(
                Update,
                (tutorial_progress, update_tutorial_context)
                    .chain()
                    .run_if(in_state(TutorialState::OnGoing)),
            )
            .add_systems(
                Update,
                update_healthbar_context.run_if(in_state(GameState::PlayingScreen)),
            );
    }
}

#[derive(States, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum TutorialState {
    OnGoing,
    Finished,
}

#[derive(Resource, Default)]
pub struct TutorialInfo {
    pub has_pressed_d_key: bool,
    pub has_pressed_a_key: bool,
    pub has_pressed_space_bar: bool,
}

#[derive(Component)]
pub struct TutorialText;

#[derive(Component)]
pub struct TutorialParent;

fn set_tutorial_state(
    level: Res<CurrentLevelInfo>,
    mut next_tutorial_state: ResMut<NextState<TutorialState>>,
) {
    if level.current_level_id == 0 {
        log::info!("Starting Tutorial");
        next_tutorial_state.set(TutorialState::OnGoing);
    }
}

fn tutorial_progress(input: Res<ButtonInput<KeyCode>>, mut tutorial_info: ResMut<TutorialInfo>) {
    // This ensures that user will go through each tutorial step in order
    #[allow(clippy::collapsible_if)]
    if !tutorial_info.has_pressed_d_key {
        if input.pressed(KeyCode::KeyD) {
            tutorial_info.has_pressed_d_key = true;
        }
    } else if !tutorial_info.has_pressed_a_key {
        if input.pressed(KeyCode::KeyA) {
            tutorial_info.has_pressed_a_key = true;
        }
    } else if !tutorial_info.has_pressed_space_bar {
        if input.pressed(KeyCode::Space) {
            tutorial_info.has_pressed_space_bar = true;
        }
    }
}

fn spawn_basic_keycodes_overlay(
    mut commands: Commands,
    icons_assets: Res<IconsAssets>,
    font_assets: Res<FontAssets>,
) {
    let font = &font_assets.default_font;

    let keyboard_w = &icons_assets.keyboard_w;
    let keyboard_a = &icons_assets.keyboard_a;
    let keyboard_s = &icons_assets.keyboard_s;
    let keyboard_d = &icons_assets.keyboard_d;

    let keyboard_spacebar_1 = &icons_assets.keyboard_spacebar_1;
    let keyboard_spacebar_2 = &icons_assets.keyboard_spacebar_2;
    let keyboard_spacebar_3 = &icons_assets.keyboard_spacebar_3;

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                top: Val::Px(10.),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                ..default()
            },
            TutorialParent,
        ))
        .with_children(|parent| {
            // Spawn First row
            parent
                .spawn(Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                // Spawn W Key
                .with_child((
                    Node {
                        width: Val::Px(50.),
                        height: Val::Px(50.),
                        ..default()
                    },
                    ImageNode {
                        image: keyboard_w.clone(),
                        ..default()
                    },
                ));

            // Spawn Second Row
            parent
                .spawn(Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.),
                    ..default()
                })
                .with_children(|parent| {
                    // Spawn A Key
                    parent.spawn((
                        Node {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            ..default()
                        },
                        ImageNode {
                            image: keyboard_a.clone(),
                            ..default()
                        },
                    ));

                    // Spawn S Key
                    parent.spawn((
                        Node {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            ..default()
                        },
                        ImageNode {
                            image: keyboard_s.clone(),
                            ..default()
                        },
                    ));

                    // Spawn D Key
                    parent.spawn((
                        Node {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            ..default()
                        },
                        ImageNode {
                            image: keyboard_d.clone(),
                            ..default()
                        },
                    ));
                });

            // Spawn Third Row
            parent
                .spawn(Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(0.),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            ..default()
                        },
                        ImageNode {
                            image: keyboard_spacebar_1.clone(),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Node {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            ..default()
                        },
                        ImageNode {
                            image: keyboard_spacebar_2.clone(),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Node {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            ..default()
                        },
                        ImageNode {
                            image: keyboard_spacebar_3.clone(),
                            ..default()
                        },
                    ));
                });
        });

    // Spawn Tutorial Text
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                bottom: Val::Px(10.),
                ..default()
            },
            TutorialParent,
        ))
        .with_child((
            Text::new("Press of `D` key to move right."),
            TextFont {
                font: font.clone(),
                font_size: 33.,
                ..default()
            },
            TextColor(Color::WHITE),
            TutorialText,
        ));
}

fn update_tutorial_context(
    tutorial_info: Res<TutorialInfo>,
    mut tutorial_text: Query<&mut Text, With<TutorialText>>,
    mut next_tutorial_state: ResMut<NextState<TutorialState>>,
    mut commands: Commands,
    font_assets: Res<FontAssets>,
) {
    if tutorial_info.is_changed() {
        log::info!("Updating Tutorial Text Context");
        for mut tutorial_text in &mut tutorial_text {
            if !tutorial_info.has_pressed_d_key {
                tutorial_text.0 = String::from("Press of `D` key to move right.");
            } else if !tutorial_info.has_pressed_a_key {
                tutorial_text.0 = String::from("Press of `A` key to move left.");
            } else if !tutorial_info.has_pressed_space_bar {
                tutorial_text.0 = String::from("Press the Space Bar key to jump.");
            } else {
                log::info!("Tutorial Finished");
                next_tutorial_state.set(TutorialState::Finished);

                commands
                    .spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(10.),
                            bottom: Val::Px(10.),
                            ..default()
                        },
                        AutoDespawn::default(),
                    ))
                    .with_child((
                        Text::new("New Goal Unlocked: Reach the Gate above."),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: font_assets.default_font.clone(),
                            font_size: 28.,
                            ..default()
                        },
                    ));
            }
        }
    }
}

fn update_healthbar_context(
    current_level_info: Res<CurrentLevelInfo>,
    font_assets: Res<FontAssets>,
    mut commands: Commands,
) {
    if current_level_info.is_changed() && current_level_info.current_level_id == 1 {
        commands
            .spawn((
                AutoDespawn::default(),
                Node {
                    left: Val::Px(10.),
                    bottom: Val::Px(10.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
            ))
            .with_child((
                Text::new("Let's Introduce Some Hostile Entities, It will be fun..."),
                TextFont {
                    font: font_assets.default_font.clone(),
                    font_size: 28.,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
    }
}
