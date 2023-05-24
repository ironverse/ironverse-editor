use bevy::{prelude::*, input::mouse::MouseButtonInput};
use bevy_flycam::MovementSettings;

use crate::data::CursorState;

pub mod hotbar;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<MouseInput>()
      .add_plugin(hotbar::CustomPlugin)
      .add_system(on_cursor_none
        .in_schedule(OnEnter(CursorState::None))
      )
      .add_system(on_cursor_locked
        .in_schedule(OnEnter(CursorState::Locked))
      );
      
  }
}

fn on_cursor_none(mut move_setting_res: ResMut<MovementSettings>,) {
  move_setting_res.sensitivity = 0.0;
  move_setting_res.speed = 0.0;
}

fn on_cursor_locked(mut move_setting_res: ResMut<MovementSettings>,) {
  move_setting_res.sensitivity = 0.00012;
  move_setting_res.speed = 6.0;
}



pub struct MouseInput {
  pub mouse_button_input: MouseButtonInput,
}

