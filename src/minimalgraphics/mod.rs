use bevy::prelude::*;

mod chunks;
mod chunk_preview;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      // .add_plugin(player::CustomPlugin)
      .add_plugin(chunks::CustomPlugin)
      .add_plugin(chunk_preview::CustomPlugin)
      ;
  }
}

pub enum GraphicsMode {
  Minimal,
  Normal,
}