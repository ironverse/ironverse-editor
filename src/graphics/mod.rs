use bevy::prelude::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
  }
}

#[derive(Component)]
pub struct TerrainGraphics {
  pub key: [i64; 3]
}