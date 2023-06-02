use bevy::prelude::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
  }
}

#[derive(Component)]
pub struct ChunkGraphics {
  pub key: [i64; 3],
}

impl Default for ChunkGraphics {
  fn default() -> Self {
    Self {
      key: [i64::MAX; 3],
    }
  }
}