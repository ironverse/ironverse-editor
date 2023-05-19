use bevy::{prelude::*, input::mouse::MouseButtonInput};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<MouseInput>();
  }
}

pub struct MouseInput {
  pub mouse_button_input: MouseButtonInput,
}

