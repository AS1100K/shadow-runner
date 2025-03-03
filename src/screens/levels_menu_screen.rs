use super::despawn_screen;
use crate::{
    assets::FontAssets,
    level_manager::{AllLevels, CurrentLevelInfo},
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

fn spawn_screen(mut commands: Commands, all_levels: Res<AllLevels>, font_assets: Res<FontAssets>) {
    let font = &font_assets.default_font;
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
                TextColor(Color::hsl(31., 0.72, 0.46)),
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
                                BackgroundColor(Color::hsl(31., 0.72, 0.46)),
                            ))
                            .with_child((
                                Text::new(format!("{}", level.0)),
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
}

#[allow(clippy::type_complexity)]
fn choose_level(
    button_query: Query<(&Interaction, &LevelButton), (With<Button>, Changed<Interaction>)>,
    mut current_level_info: ResMut<CurrentLevelInfo>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, level_button) in &button_query {
        if Interaction::Pressed == *interaction {
            current_level_info.current_level_id = level_button.level_id;
            next_game_state.set(GameState::PlayingScreen);

            return;
        }
    }
}
