use bevy::{prelude::*, input::mouse::MouseButtonInput, window::CursorGrabMode};
use bevy_flycam::MovementSettings;
use crate::{input::MouseInput, data::CursorState, ui::UIState};

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
  cursor_state: Res<State<CursorState>>,
  mut ui_state_next: ResMut<NextState<UIState>>,
  ui_state: Res<State<UIState>>,
) {
  if mouse.just_pressed(MouseButton::Left) {
    match ui_state.0 {
      UIState::Inventory => { },
      UIState::Default => { cursor_state_next.set(CursorState::Locked); },
      _ => {  }
    };
    
  }

  if key.just_pressed(KeyCode::LControl) {
    match cursor_state.0 {
      CursorState::None => {
        cursor_state_next.set(CursorState::Locked);

        if ui_state.0 != UIState::Default {
          ui_state_next.set(UIState::Default);
        }
      },
      CursorState::Locked => {
        cursor_state_next.set(CursorState::None);
      },
      _ => {}
    };
    
  }
}

fn cursor_free(
  mut windows: Query<&mut Window>,
  mut move_setting_res: ResMut<MovementSettings>,
) {
  let mut window = windows.single_mut();
  window.cursor.visible = true;
  window.cursor.grab_mode = CursorGrabMode::None;

  move_setting_res.sensitivity = 0.0;
  move_setting_res.speed = 0.0;
}

fn cursor_locked(
  mut windows: Query<&mut Window>,
  mut move_setting_res: ResMut<MovementSettings>,
) {
  let mut window = windows.single_mut();
  window.cursor.visible = false;
  window.cursor.grab_mode = CursorGrabMode::Locked;

  move_setting_res.sensitivity = 0.00012;
  move_setting_res.speed = 6.0;
}