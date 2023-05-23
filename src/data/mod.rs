use rapier3d::prelude::{RigidBodyHandle, ColliderHandle};
use bevy::prelude::*;
use voxels::chunk::chunk_manager::ChunkManager;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<GameState>()
      .add_state::<CursorState>()
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

#[derive(Component, Debug, Clone)]
pub struct Player {
  pub body: RigidBodyHandle,
  pub collider: ColliderHandle,
  pub prev_key: [i64; 3],
  pub key: [i64; 3],
}

impl Player {
  pub fn new(b: RigidBodyHandle, c: ColliderHandle, k: [i64; 3]) -> Self {
    
    Self {
      body: b,
      collider: c,
      prev_key: k.clone(),
      key: k
    }
  }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
  #[default]
  Start,
  New,
  Load,
  Play,
  Pause,
  End,
}


#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum CursorState {
  #[default]
  None,
  Locked,
}