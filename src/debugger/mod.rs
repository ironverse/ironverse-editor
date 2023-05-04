use bevy::prelude::*;

mod raycast;
mod camera;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(raycast::CustomPlugin)
      .add_plugin(camera::CustomPlugin);
  }
}
