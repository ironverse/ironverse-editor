use bevy::{prelude::*, input::mouse::MouseButtonInput, window::CursorGrabMode};
use crate::input::MouseInput;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update)
      // .add_system(grab_mouse)
      ;
  }
}

fn update(
  mut mouse_events: EventReader<MouseButtonInput>,
  mut mouse_inputs: EventWriter<MouseInput>,
) {
  for event in mouse_events.iter() {
    mouse_inputs.send(MouseInput { mouse_button_input: event.clone() });
  }
}


fn grab_mouse(
  mut windows: Query<&mut Window>,
  mouse: Res<Input<MouseButton>>,
  key: Res<Input<KeyCode>>,
) {
  // let mut window = windows.single_mut();
  // if mouse.just_pressed(MouseButton::Left) {
  //   window.cursor.visible = false;
  //   window.cursor.grab_mode = CursorGrabMode::Confined;
  // }

  // if key.just_pressed(KeyCode::Escape) {
  //   window.cursor.visible = true;
  //   window.cursor.grab_mode = CursorGrabMode::None;
  // }
}
