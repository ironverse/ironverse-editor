use bevy::prelude::*;

pub mod player_movement;
pub mod chunks;
// pub mod raycast;
// pub mod save;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(player_movement::CustomPlugin)
      .add_plugin(chunks::CustomPlugin)
      // .add_plugin(raycast::CustomPlugin)
      // .add_plugin(save::CustomPlugin)
      ;
  }
}


