use bevy::prelude::*;

pub mod player;
pub mod chunks;
pub mod raycast;
pub mod camera;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(player::CustomPlugin)
      .add_plugin(chunks::CustomPlugin)
      .add_plugin(raycast::CustomPlugin)
      .add_plugin(camera::CustomPlugin)
      ;
  }
}


