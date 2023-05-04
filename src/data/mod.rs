use rapier3d::prelude::{RigidBodyHandle, ColliderHandle};
use bevy::{prelude::*, utils::HashMap};
use voxels::chunk::chunk_manager::ChunkManager;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GameResource::default());
  }
}

#[derive(Resource)]
pub struct GameResource {
  pub chunk_manager: ChunkManager,
}

impl Default for GameResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::default(),
    }
  }
}

