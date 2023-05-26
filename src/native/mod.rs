use bevy::{prelude::*, input::mouse::MouseButtonInput, window::CursorGrabMode};
use crate::{input::MouseInput, data::CursorState};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update)
      .add_system(grab_mouse)
      .add_system(cursor_free.in_schedule(OnEnter(CursorState::None)))
      .add_system(cursor_locked.in_schedule(OnEnter(CursorState::Locked)))
      ;
  }
}

fn update(
  mut mouse_events: EventReader<MouseButtonInput>,
  mut mouse_inputs: EventWriter<MouseInput>,
  cursor_state: Res<State<CursorState>>,
) {
  for event in mouse_events.iter() {
    if cursor_state.0 == CursorState::None {
      return;
    }

    mouse_inputs.send(MouseInput { mouse_button_input: event.clone() });
  }
}


fn grab_mouse(
  mouse: Res<Input<MouseButton>>,
  key: Res<Input<KeyCode>>,
  mut cursor_state_next: ResMut<NextState<CursorState>>,
) {
  if mouse.just_pressed(MouseButton::Left) {
    cursor_state_next.set(CursorState::Locked);
  }

  if key.just_pressed(KeyCode::Escape) {
    cursor_state_next.set(CursorState::None);
  }
}

fn cursor_free(
  mut windows: Query<&mut Window>,
) {
  let mut window = windows.single_mut();
  window.cursor.visible = true;
  window.cursor.grab_mode = CursorGrabMode::None;
}

fn cursor_locked(mut windows: Query<&mut Window>,) {
  let mut window = windows.single_mut();
  window.cursor.visible = false;
  window.cursor.grab_mode = CursorGrabMode::Locked;
}