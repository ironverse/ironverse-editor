use bevy::prelude::*;


#[derive(Component)]
pub struct ChunkPreviewRender {
  pub entities: Vec<Entity>,
}

impl Default for ChunkPreviewRender {
  fn default() -> Self {
    Self {
      entities: Vec::new(),
    }
  }
}
