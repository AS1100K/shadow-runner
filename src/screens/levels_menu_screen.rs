use super::{despawn_screen, game_over_screen::RestartGameEvent, MainMenuButton};
use crate::{
    assets::{self, FontAssets},
    level_manager::{AllLevels, CurrentLevelInfo},
    time::{spawn_best_time, TimeTakenRes},
    GameState,
};
use bevy::prelude::*;

pub struct LevelsMenuPlugin;

impl Plugin for LevelsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LevelsMenuScreen), spawn_screen)
            .add_systems(
                OnExit(GameState::LevelsMenuScreen),
                despawn_screen::<OnLevelMenuScreen>,
            )
            .add_systems(
                Update,
                choose_level.run_if(in_state(GameState::LevelsMenuScreen)),
            );
    }
}

#[derive(Component)]
pub struct OnLevelMenuScreen;

#[derive(Component)]
pub struct LevelButton {
    level_id: i32,
}

fn spawn_screen(
    mut commands: Commands,
    all_levels: Res<AllLevels>,
    font_assets: Res<FontAssets>,
    world: Res<assets::World>,
    time_taken_res: Res<TimeTakenRes>,
) {
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
            OnLevelMenuScreen,
        ))
        .with_child(ImageNode {
            image: world.background.clone(),
            ..default()
        });

    // Back Button
    commands
        .spawn((
            OnLevelMenuScreen,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.),
                left: Val::Px(10.),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.),
                padding: UiRect::axes(Val::Px(20.), Val::Px(10.)),
                ..default()
            },
            BackgroundColor(Color::hsl(327., 0.24, 0.16)),
            Button,
            MainMenuButton,
        ))
        .with_child((
            Text::new("Main Menu"),
            TextColor::WHITE,
            TextFont {
                font: font_assets.default_font.clone(),
                font_size: 20.,
                ..default()
            },
        ));

    commands
        .spawn((
            OnLevelMenuScreen,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                row_gap: Val::Px(100.),
                left: Val::Px(0.),
                top: Val::Px(0.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Choose Level"),
                TextColor(Color::hsl(327., 0.24, 0.16)),
                TextFont {
                    font: font.clone(),
                    font_size: 100.,
                    ..default()
                },
            ));

            // Spawn Grid
            parent
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![
                        GridTrack::px(100.),
                        GridTrack::px(100.),
                        GridTrack::px(100.),
                    ],
                    grid_auto_rows: vec![
                        GridTrack::px(100.),
                        GridTrack::px(100.),
                        GridTrack::px(100.),
                    ],
                    row_gap: Val::Px(50.),
                    column_gap: Val::Px(50.),
                    ..default()
                })
                .with_children(|parent| {
                    for level in &all_levels.all_levels {
                        parent
                            .spawn((
                                Button,
                                LevelButton { level_id: *level.0 },
                                Node {
                                    width: Val::Px(100.),
                                    height: Val::Px(100.),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::hsl(327., 0.24, 0.16)),
                            ))
                            .with_child((
                                Text::new(format!("{}", level.0 + 1)),
                                TextColor(Color::WHITE),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 33.,
                                    ..default()
                                },
                            ));
                    }
                });
        });

    spawn_best_time(
        &mut commands,
        time_taken_res,
        font,
        OnLevelMenuScreen,
        70.,
        16.,
    );
}

#[allow(clippy::type_complexity)]
fn choose_level(
    button_query: Query<(&Interaction, &LevelButton), (With<Button>, Changed<Interaction>)>,
    mut current_level_info: ResMut<CurrentLevelInfo>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut restart_game_event: EventWriter<RestartGameEvent>,
) {
    for (interaction, level_button) in &button_query {
        if Interaction::Pressed == *interaction {
            next_game_state.set(GameState::PlayingScreen);
            if current_level_info.current_level_id == level_button.level_id {
                restart_game_event.send(RestartGameEvent);
            }
            current_level_info.current_level_id = level_button.level_id;
            return;
        }
    }
}
