use bevy::prelude::*;
use voxels::chunk::{adjacent_keys, chunk_manager::ChunkManager};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(TerrainResource::default())
      .add_startup_system(startup)
      .add_system(update);
  }
}

fn startup() {
  // Load terrains
  let keys = adjacent_keys(&[0, 0, 0], 1, true);
  let config = ChunkManager::default().config.clone();

  for key in keys.iter() {
    let _chunk = ChunkManager::new_chunk(key, config.depth, config.lod, config.noise);
    
  }
}

fn update() {

}

#[derive(Resource)]
pub struct TerrainResource {

}

impl Default for TerrainResource {
  fn default() -> Self {
    Self {

    }
  }
}