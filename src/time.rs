use crate::{assets::FontAssets, screens::despawn_screen, GameState};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::utils::{Duration, HashMap};

pub struct TimeTakenPlugin;

impl Plugin for TimeTakenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RestartTimeEvent>()
            .add_event::<RecordTimeEvent>()
            .insert_resource(TimeTakenRes::default())
            .add_systems(Update, (restart_time, record_final_time))
            .add_systems(
                Update,
                (tick_time, update_time)
                    .chain()
                    .run_if(in_state(GameState::PlayingScreen)),
            )
            .add_systems(OnEnter(GameState::PlayingScreen), render_time)
            .add_systems(
                OnExit(GameState::PlayingScreen),
                despawn_screen::<TimeComponent>,
            );
    }
}

#[derive(Event)]
pub struct RestartTimeEvent;

#[derive(Event)]
pub struct RecordTimeEvent(pub i32);

#[derive(Resource, Debug, better_default::Default)]
#[default(stopwatch: Stopwatch::new())]
pub struct TimeTakenRes {
    pub all_times: HashMap<i32, Duration>,
    pub stopwatch: Stopwatch,
}

fn restart_time(
    mut level_changed_event: EventReader<RestartTimeEvent>,
    mut time_taken_res: ResMut<TimeTakenRes>,
) {
    for _level_changed in level_changed_event.read() {
        log::info!("Restarting Time");
        time_taken_res.stopwatch.reset();

        if time_taken_res.stopwatch.is_paused() {
            time_taken_res.stopwatch.unpause();
        }
    }
}

fn record_final_time(
    mut level_finished_event: EventReader<RecordTimeEvent>,
    mut time_taken_res: ResMut<TimeTakenRes>,
) {
    for level_finished in level_finished_event.read() {
        log::info!("Recoding Final Time");
        let time_taken = time_taken_res.stopwatch.elapsed();

        if let Some(exisiting_best_time) = time_taken_res.all_times.get_mut(&level_finished.0) {
            if time_taken < *exisiting_best_time {
                *exisiting_best_time = time_taken;
            }
        } else {
            time_taken_res
                .all_times
                .insert(level_finished.0, time_taken);
        }

        // Pause The Clock
        time_taken_res.stopwatch.pause();
    }
}

#[derive(Component)]
pub struct TimeComponent;

#[derive(Component)]
pub struct TimeText;

fn render_time(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.),
                top: Val::Px(10.),
                ..default()
            },
            TimeComponent,
        ))
        .with_child((
            Text::new("00:00"),
            TextFont {
                font: font_assets.default_font.clone(),
                font_size: 33.,
                ..default()
            },
            TextColor(Color::WHITE),
            TimeText,
        ));
}

fn tick_time(mut time_taken_res: ResMut<TimeTakenRes>, time: Res<Time<Virtual>>) {
    time_taken_res
        .stopwatch
        .tick(Duration::from_secs_f32(time.delta_secs()));
}

fn update_time(mut query: Query<&mut Text, With<TimeText>>, time_taken_res: Res<TimeTakenRes>) {
    for mut text in &mut query {
        let time_elapsed = time_taken_res.stopwatch.elapsed();
        text.0 = convert_time_to_text(&time_elapsed);
    }
}

pub fn convert_time_to_text(duration: &Duration) -> String {
    let time_elapsed = duration.as_secs_f64().round();

    let minutes = (time_elapsed / 60.0).floor() as u32;
    let seconds = (time_elapsed % 60.0).floor() as u32;
    format!("{:02}:{:02}", minutes, seconds)
}

pub fn spawn_best_time(
    commands: &mut Commands,
    time_taken_res: Res<TimeTakenRes>,
    font: &Handle<Font>,
    screen_component: impl Component + 'static,
    top: f32,
    left: f32,
) {
    commands
        .spawn((
            screen_component,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(left),
                top: Val::Px(top),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                ..default()
            },
            ZIndex(2),
        ))
        .with_children(|parent| {
            if !time_taken_res.all_times.is_empty() {
                parent.spawn((
                    Text::new("My Best Time"),
                    TextColor(Color::hsl(327., 0.24, 0.16)),
                    TextFont {
                        font: font.clone(),
                        font_size: 33.,
                        ..default()
                    },
                ));

                let mut sorted_times: Vec<_> = time_taken_res.all_times.iter().collect();
                sorted_times.sort_by_key(|&(level_id, _)| level_id);

                for (level_id, time) in sorted_times {
                    parent.spawn((
                        Text::new(format!(
                            "Level {} - {}",
                            level_id,
                            convert_time_to_text(time)
                        )),
                        TextColor(Color::hsl(327., 0.24, 0.16)),
                        TextFont {
                            font: font.clone(),
                            font_size: 28.,
                            ..default()
                        },
                    ));
                }
            }
        });
}
