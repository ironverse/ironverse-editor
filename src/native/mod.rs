use bevy::{prelude::*, input::mouse::MouseButtonInput};
use crate::input::MouseInput;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update);
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
