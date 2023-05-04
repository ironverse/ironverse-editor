use bevy::prelude::*;

mod chunks;
mod player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(chunks::CustomPlugin)
      .add_plugin(player::CustomPlugin);
  }
}