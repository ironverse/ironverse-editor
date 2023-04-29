use bevy::prelude::*;

mod chunks;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(chunks::CustomPlugin);
  }
}