use bevy::prelude::*;

pub mod hotbar;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(hotbar::CustomPlugin);
  }
}
