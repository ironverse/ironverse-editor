use bevy::{prelude::*, input::mouse::MouseButtonInput, window::CursorGrabMode};
use bevy_flycam::MovementSettings;

use crate::{data::CursorState, ui::UIState};

pub mod hotbar;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<MouseInput>()
      .add_plugin(hotbar::CustomPlugin)
      ;
      
  }
}

pub struct MouseInput {
  pub mouse_button_input: MouseButtonInput,
}

