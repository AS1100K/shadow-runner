use super::despawn_screen;
use crate::{
    assets::*,
    time::{convert_time_to_text, RestartTimeEvent, TimeTakenRes},
    GameState,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOverScreen), spawn_screen)
            .add_systems(
                OnExit(GameState::GameOverScreen),
                despawn_screen::<OnGameOverScreen>,
            )
            .add_systems(
                Update,
                restart_game.run_if(in_state(GameState::GameOverScreen)),
            );
    }
}

#[derive(Component)]
pub struct OnGameOverScreen;

#[derive(Component)]
pub struct RestartGameButton;

// TODO: Hover Effect
fn spawn_screen(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    icon_assets: Res<IconsAssets>,
    time_taken_res: Res<TimeTakenRes>,
) {
    let font = &font_assets.default_font;
    let reset_icon = &icon_assets.reset_icon;

    commands
        .spawn((
            OnGameOverScreen,
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
            // White With 10% Opacity
            BackgroundColor(Color::hsla(0., 1., 1., 0.1)),
        ))
        .with_children(|parent| {
            // Spawn Text
            parent.spawn((
                Text::new("Game Over"),
                // hsl(31, 72%, 46%)
                TextColor(Color::hsl(31., 0.72, 0.46)),
                TextFont {
                    font: font.clone(),
                    font_size: 100.,
                    ..default()
                },
            ));

            // Spawn Time Information
            parent.spawn((
                Text::new(format!(
                    "Time Taken: {}",
                    convert_time_to_text(&time_taken_res.stopwatch.elapsed())
                )),
                // hsl(31, 72%, 46%)
                TextColor(Color::hsl(31., 0.72, 0.46)),
                TextFont {
                    font: font.clone(),
                    font_size: 28.,
                    ..default()
                },
            ));

            // Spawn Buttons
            parent
                .spawn(Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
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
                            BackgroundColor(Color::hsl(31., 0.72, 0.46)),
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
                });
        });
}

#[allow(clippy::too_many_arguments)]
fn restart_game(
    query: Query<&Interaction, (With<RestartGameButton>, Changed<Interaction>)>,
    level_selection: Res<LevelSelection>,
    levels: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut time: ResMut<Time<Virtual>>,
    mut restart_time_event: EventWriter<RestartTimeEvent>,
) {
    for interaction in &query {
        if Interaction::Pressed == *interaction {
            let current_level = match level_selection.as_ref() {
                LevelSelection::Iid(iid) => iid,
                LevelSelection::Indices(indice) => {
                    let ldtk_project = ldtk_project_assets
                        .get(ldtk_projects.single())
                        .expect("Project should be loaded if level has spawned");

                    &match ldtk_project.get_raw_level_at_indices(indice) {
                        Some(iid) => LevelIid::new(iid.iid.clone()),
                        None => {
                            log::error!("Level Indice: {:?} didn't exits", indice);
                            return;
                        }
                    }
                }
                _ => {
                    log::error!(
                        "The Current level is not of Iid. LevelSelection: {:?}",
                        level_selection
                    );
                    return;
                }
            };

            for (level_entity, level_iid) in &levels {
                if level_iid == current_level {
                    restart_time_event.send(RestartTimeEvent);
                    commands.entity(level_entity).insert(Respawn);
                    next_game_state.set(GameState::PlayingScreen);
                    time.unpause();

                    return;
                }
            }

            log::error!("Failed to find level with iid {:?}", current_level);
        }
    }
}
