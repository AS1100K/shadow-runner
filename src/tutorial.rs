use crate::{
    assets::{EntitySpriteAssets, FontAssets, IconsAssets},
    level_manager::CurrentLevelInfo,
    screens::despawn_screen,
    sprite_animation::Animation,
    AutoDespawn, GameState,
};
use bevy::prelude::*;
use bevy::utils::Duration;

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
                update_level_specific_context.run_if(in_state(GameState::PlayingScreen)),
            )
            .add_systems(Update, auto_remove_tutorial);
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

// This component is only used for first level tutorial
#[derive(Component)]
pub struct TutorialParent;

#[derive(Component)]
pub struct TutorialLevelSpecific(pub i32);

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

fn update_level_specific_context(
    current_level_info: Res<CurrentLevelInfo>,
    font_assets: Res<FontAssets>,
    entity_sprite_assets: Res<EntitySpriteAssets>,
    icons_assets: Res<IconsAssets>,
    mut commands: Commands,
) {
    if current_level_info.is_changed() {
        match current_level_info.current_level_id {
            1 => {
                commands
                    .spawn((
                        AutoDespawn::default(),
                        Node {
                            left: Val::Px(10.),
                            bottom: Val::Px(10.),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        TutorialLevelSpecific(1),
                    ))
                    .with_child((
                        Text::new(
                            "Let's Introduce Some Hostile Entities, It will be fun...\nDodge Them",
                        ),
                        TextFont {
                            font: font_assets.default_font.clone(),
                            font_size: 33.,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
            }
            2 => {
                commands
                    .spawn((
                        AutoDespawn::new_recursive_despawn(Duration::from_secs(30)),
                        Node {
                            right: Val::Px(10.),
                            bottom: Val::Px(10.),
                            position_type: PositionType::Absolute,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::FlexEnd,
                            row_gap: Val::Px(20.),
                            ..default()
                        },
                        TutorialLevelSpecific(2),
                    ))
                    .with_children(|parent| {
                        // Sand Ghoul
                        parent
                            .spawn(Node {
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                column_gap: Val::Px(20.),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(Node {
                                        width: Val::Px(75.),
                                        height: Val::Px(75.),
                                        ..default()
                                    })
                                    .with_child((
                                        ImageNode {
                                            image: entity_sprite_assets.sand_ghoul.clone(),
                                            texture_atlas: Some(
                                                entity_sprite_assets.layout.clone().into(),
                                            ),
                                            ..default()
                                        },
                                        Animation::new_image_node(
                                            0,
                                            3,
                                            Timer::from_seconds(0.25, TimerMode::Repeating),
                                        ),
                                    ));

                                parent
                                    .spawn(Node {
                                        height: Val::Px(75.),
                                        display: Display::Flex,
                                        justify_content: JustifyContent::FlexEnd,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    })
                                    .with_child((
                                        Text::new("1 Heart Damage"),
                                        TextColor(Color::WHITE),
                                        TextFont {
                                            font: font_assets.default_font.clone(),
                                            font_size: 28.,
                                            ..default()
                                        },
                                    ));
                            });

                        // Grave Revenant
                        parent
                            .spawn(Node {
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                column_gap: Val::Px(20.),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(Node {
                                        width: Val::Px(75.),
                                        height: Val::Px(75.),
                                        ..default()
                                    })
                                    .with_child((
                                        ImageNode {
                                            image: entity_sprite_assets.grave_revenant.clone(),
                                            texture_atlas: Some(
                                                entity_sprite_assets.layout.clone().into(),
                                            ),
                                            ..default()
                                        },
                                        Animation::new_image_node(
                                            0,
                                            3,
                                            Timer::from_seconds(0.25, TimerMode::Repeating),
                                        ),
                                    ));

                                parent
                                    .spawn(Node {
                                        height: Val::Px(75.),
                                        display: Display::Flex,
                                        justify_content: JustifyContent::FlexEnd,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    })
                                    .with_child((
                                        Text::new("2 Heart Damage"),
                                        TextColor(Color::WHITE),
                                        TextFont {
                                            font: font_assets.default_font.clone(),
                                            font_size: 28.,
                                            ..default()
                                        },
                                    ));
                            });

                        // Mutilated Stumbler
                        parent
                            .spawn(Node {
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                column_gap: Val::Px(20.),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(Node {
                                        width: Val::Px(75.),
                                        height: Val::Px(75.),
                                        ..default()
                                    })
                                    .with_child((
                                        ImageNode {
                                            image: entity_sprite_assets.mutilated_stumbler.clone(),
                                            texture_atlas: Some(
                                                entity_sprite_assets.layout.clone().into(),
                                            ),
                                            ..default()
                                        },
                                        Animation::new_image_node(
                                            0,
                                            3,
                                            Timer::from_seconds(0.25, TimerMode::Repeating),
                                        ),
                                    ));

                                parent
                                    .spawn(Node {
                                        height: Val::Px(75.),
                                        display: Display::Flex,
                                        justify_content: JustifyContent::FlexEnd,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    })
                                    .with_child((
                                        Text::new("3 Heart Damage"),
                                        TextColor(Color::WHITE),
                                        TextFont {
                                            font: font_assets.default_font.clone(),
                                            font_size: 28.,
                                            ..default()
                                        },
                                    ));
                            });
                    });
            }
            3 => {
                commands
                    .spawn((
                        AutoDespawn::default(),
                        Node {
                            top: Val::Px(10.),
                            left: Val::Px(10.),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        TutorialLevelSpecific(3),
                    ))
                    .with_child((
                        Text::new("New Chapter: Dungeons\nDungeons are heavily guarded"),
                        TextColor::WHITE,
                        TextFont {
                            font: font_assets.default_font.clone(),
                            font_size: 33.,
                            ..default()
                        },
                    ));

                // Spawn New Entities Information
                commands
                    .spawn((
                        AutoDespawn::new(Duration::from_secs(30)),
                        Node {
                            right: Val::Px(10.),
                            bottom: Val::Px(10.),
                            position_type: PositionType::Absolute,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::FlexStart,
                            row_gap: Val::Px(50.),
                            ..default()
                        },
                        TutorialLevelSpecific(3),
                    ))
                    .with_children(|parent| {
                        // Spawn Jump Booster Info
                        parent
                            .spawn(Node {
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(50.),
                                ..default()
                            })
                            .with_children(|parent| {
                                // Spawn Jump Booster Sprite
                                parent
                                    .spawn(Node {
                                        width: Val::Px(75.),
                                        height: Val::Px(75.),
                                        ..default()
                                    })
                                    .with_child(ImageNode {
                                        image: icons_assets.jump_booster_icon.clone(),
                                        ..default()
                                    });

                                // Spawn Info
                                parent.spawn((
                                    Text::new("Jump Booster: Boosts you\nup in the air."),
                                    TextColor::WHITE,
                                    TextFont {
                                        font: font_assets.default_font.clone(),
                                        font_size: 33.,
                                        ..default()
                                    },
                                ));
                            });

                        // Spawn Spike Info
                        parent
                            .spawn(Node {
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(50.),
                                ..default()
                            })
                            .with_children(|parent| {
                                // Spawn Jump Booster Sprite
                                parent
                                    .spawn(Node {
                                        width: Val::Px(75.),
                                        height: Val::Px(75.),
                                        ..default()
                                    })
                                    .with_child((
                                        ImageNode {
                                            image: icons_assets.spike.clone(),
                                            texture_atlas: Some(
                                                icons_assets.spike_layout.clone().into(),
                                            ),
                                            ..default()
                                        },
                                        Animation::new_image_node(
                                            0,
                                            5,
                                            Timer::from_seconds(0.25, TimerMode::Repeating),
                                        ),
                                    ));

                                // Spawn Info
                                parent.spawn((
                                    Text::new("Spike: Deals 1 Heart\nDamage every second"),
                                    TextColor::WHITE,
                                    TextFont {
                                        font: font_assets.default_font.clone(),
                                        font_size: 33.,
                                        ..default()
                                    },
                                ));
                            });
                    });
            }
            4 => {
                commands
                    .spawn((
                        AutoDespawn::new_recursive_despawn(Duration::from_secs(30)),
                        Node {
                            position_type: PositionType::Absolute,
                            right: Val::Px(10.),
                            bottom: Val::Px(10.),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(20.),
                            ..default()
                        },
                        TutorialLevelSpecific(4),
                    ))
                    .with_children(|parent| {
                        // Spawn Image
                        parent
                            .spawn(Node {
                                width: Val::Px(75.),
                                height: Val::Px(75.),
                                ..default()
                            })
                            .with_child((ImageNode {
                                image: entity_sprite_assets.adept_necromancer.clone(),
                                texture_atlas: Some(entity_sprite_assets.layout.clone().into()),
                                ..default()
                            }, Animation::new_image_node(0, 3, Timer::from_seconds(0.25, TimerMode::Repeating))));

                        // Spawn Text
                        parent.spawn((
                            Text::new("Adept Necromancer:\nA power being with\nthe ability to blind you.\nDeals 1 Heart Damage"),
                            TextColor::WHITE,
                            TextFont {
                                font: font_assets.default_font.clone(),
                                font_size: 33.,
                                ..default()
                            },
                        ));
                    });
            }
            _ => {}
        }
    }
}

fn auto_remove_tutorial(
    current_level_info: Res<CurrentLevelInfo>,
    query: Query<(Entity, &TutorialLevelSpecific)>,
    mut commands: Commands,
) {
    if current_level_info.is_changed() {
        for (entity, tutorial) in &query {
            if tutorial.0 != current_level_info.current_level_id {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
