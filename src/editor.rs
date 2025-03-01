//! Internal Editor Module
//!
//! This is an internal module accessible only via
//! feature `debug`. This allows to see the position
//! of any component with `Clickable` Component as
//! well as ability to drag them and change their
//! position.

use crate::camera::MainCamera;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct Editor;

impl Plugin for Editor {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClickedEntitiesInfo::default())
            .insert_resource(DragState::default())
            .add_systems(Startup, setup_text_component)
            .add_systems(Update, cusor_click_system)
            .add_systems(Update, handle_dragging)
            .add_systems(Update, update_text_component);
    }
}

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Clickable(pub &'static str);

#[derive(Resource, Default)]
pub struct DragState {
    dragging_entity_name: &'static str,
    dragging_entity: Option<Entity>,
    drag_start: Option<Vec2>,
}

impl DragState {
    pub fn reset(&mut self) {
        self.dragging_entity_name = "";
        self.dragging_entity = None;
        self.drag_start = None;
    }
}

#[derive(Resource, Default)]
pub struct ClickedEntitiesInfo {
    clicked_entities: Vec<String>,
}

#[derive(Component)]
struct InfoText;

fn cursor_position(
    window: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera_q.single();
    let window = window.single();

    window.cursor_position().and_then(|cursor_position| {
        camera
            .viewport_to_world_2d(camera_transform, cursor_position)
            .ok()
    })
}

fn cusor_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    clickable_query: Query<(Entity, &Transform, &Sprite, &Clickable)>,
    cursor_pos: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut clicked_entities_info: ResMut<ClickedEntitiesInfo>,
    mut drag_state: ResMut<DragState>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let shift_held = keyboard.pressed(KeyCode::ShiftLeft);

        let mut clickable_vec = Vec::new();
        if let Some(world_position) = cursor_position(cursor_pos, camera_q) {
            for (entity, transform, sprite, clickable) in clickable_query.iter() {
                let sprite_position = transform.translation.truncate();
                let distance = world_position.distance(sprite_position);

                if let Some(sprite_size) = sprite.custom_size {
                    // Check if click is within sprite bounds using size
                    let half_width = sprite_size.x / 2.0;
                    let half_height = sprite_size.y / 2.0;

                    let x_distance = (world_position.x - sprite_position.x).abs();
                    let y_distance = (world_position.y - sprite_position.y).abs();

                    if x_distance <= half_width && y_distance <= half_height {
                        log::debug!(
                            "Clicked \"{}\": Sprite Pos: {:?}, Sprite Size: {:?}, Distance: {:?}",
                            clickable.0,
                            sprite_position,
                            sprite_size,
                            distance
                        );

                        if shift_held {
                            drag_state.dragging_entity_name = clickable.0;
                            drag_state.dragging_entity = Some(entity);
                            drag_state.drag_start = Some(world_position);
                        }

                        clickable_vec.push(format!(
                            "Clicked \"{}\"\n Sprite Pos: {:?}\n Sprite Size: {:?}",
                            clickable.0, sprite_position, sprite_size
                        ));
                    }
                }
            }
        }

        if !clickable_vec.is_empty() {
            clicked_entities_info.clicked_entities = clickable_vec;
        }
    }
}

fn setup_text_component(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            bottom: Val::Px(10.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((Text::new(""), TextColor(Color::hsl(0., 1., 0.5)), InfoText));
        });
}

fn update_text_component(
    mut query: Query<&mut Text, With<InfoText>>,
    windows: Query<&Window>,
    clicked_entities_info: Res<ClickedEntitiesInfo>,
    drag_state: Res<DragState>,
) {
    for mut text in &mut query {
        let window = windows.single();
        let info_string = clicked_entities_info.clicked_entities.join("\n");

        let debug_info = format!(
            "Debug Info\nWindow Width: {}\nWindow Height: {}\n\n{}",
            window.width(),
            window.height(),
            info_string
        );

        let info_text = if drag_state.dragging_entity.is_some() {
            format!(
                "{}\n\nDragging Info:\nDragging Component: \"{}\"\n Drag Start: {:?}",
                debug_info, drag_state.dragging_entity_name, drag_state.drag_start
            )
        } else {
            debug_info
        };

        text.0 = info_text;
    }
}

fn handle_dragging(
    mut drag_state: ResMut<DragState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    cursor_pos: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut transforms: Query<&mut Transform, With<Clickable>>,
) {
    let shift_held = keyboard.pressed(KeyCode::ShiftLeft);
    let left_click_held = mouse_button.pressed(MouseButton::Left);

    let current_pos = cursor_position(cursor_pos, camera_q);

    if let (Some(entity), Some(current_world_pos)) = (drag_state.dragging_entity, current_pos) {
        if let Ok(mut transform) = transforms.get_mut(entity) {
            if !shift_held || !left_click_held {
                drag_state.reset();
            }

            transform.translation.x = current_world_pos.x;
            transform.translation.y = current_world_pos.y;
        } else {
            drag_state.reset();
        }
    }
}
