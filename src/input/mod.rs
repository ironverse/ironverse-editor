use bevy::{prelude::*, input::mouse::MouseButtonInput};

pub mod hotbar;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<MouseInput>()
      .add_plugin(hotbar::CustomPlugin);
      
  }
}

pub struct MouseInput {
  pub mouse_button_input: MouseButtonInput,
}

