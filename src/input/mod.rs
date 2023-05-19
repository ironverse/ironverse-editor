use bevy::prelude::*;

// pub mod hotbar;
pub mod mouse_input;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(mouse_input::CustomPlugin);
  }
}
