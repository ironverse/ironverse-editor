use bevy::prelude::*;

mod chunks;
mod player;
mod camera;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(chunks::CustomPlugin)
      .add_plugin(player::CustomPlugin)
      .add_plugin(camera::CustomPlugin);
  }
}