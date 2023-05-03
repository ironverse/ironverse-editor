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
  pub players: HashMap<u32, Player>,
  pub chunk_manager: ChunkManager,
}

impl Default for GameResource {
  fn default() -> Self {
    Self {
      players: HashMap::default(),
      chunk_manager: ChunkManager::default(),
    }
  }
}

pub struct Player {
  pub body: RigidBodyHandle,
  pub collider: ColliderHandle
}

impl Player {
  pub fn new(b: RigidBodyHandle, c: ColliderHandle) -> Self {
    Self {
      body: b,
      collider: c
    }
  }
}


