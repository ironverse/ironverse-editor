use bevy::prelude::*;
use serde::{Deserialize, Serialize};
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
  pub data: Data,

  pub preview_chunk_manager: ChunkManager,
}

impl Default for GameResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::default(),
      data: Data::default(),
      preview_chunk_manager: ChunkManager::default(),
    }
  }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
  #[default]
  Start,
  New,
  LoadGame,
  SaveGame,
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



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Data {
  pub status: Status,
  pub terrains: Terrains,
}

impl Default for Data {
  fn default() -> Self {
    Self {
      status: Status { position: [0.0, 5.0, 0.0] },
      terrains: Terrains { keys: Vec::new(), voxels: Vec::new() }
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Status {
  pub position: [f32; 3]
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Terrains {
  pub keys: Vec<[i64; 3]>,
  pub voxels: Vec<String>,
}
