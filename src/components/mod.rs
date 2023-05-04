use bevy::prelude::*;

pub mod player;
pub mod chunks;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(player::CustomPlugin)
      .add_plugin(chunks::CustomPlugin)
      ;
  }
}


