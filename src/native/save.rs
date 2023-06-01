use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use voxels::chunk::chunk_manager::Chunk;
use voxels::chunk::chunk_manager::ChunkManager;
use crate::components::chunks::Chunks;
use crate::data::Data;
use crate::data::Status;
use crate::data::Terrains;

use std::fs::File;
use std::io::Write;

use crate::data::GameState;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(track_modified_chunks)
      .add_system(enter.in_schedule(OnEnter(GameState::SaveGame)));
  }
}

fn enter(
  mut local_res: ResMut<LocalResource>,
) {
  let mut terrains = Terrains { keys: Vec::new(), voxels: Vec::new() };
  for (key, chunk) in local_res.chunks.iter() {
    terrains.keys.push(key.clone());
    terrains.voxels.push(array_bytes::bytes2hex("", &chunk.octree.data));
  }

  let data = Data {
    status: Status {
      position: [0.0, 1.0, 0.0],
    },
    terrains: terrains
  };

  let str = toml::to_string_pretty(&data).unwrap();
  let path = std::env::current_dir().unwrap();
  let res = rfd::FileDialog::new()
    .set_file_name("save.toml")
    .set_directory(&path)
    .save_file();

  if res.is_none() {
    return;
    
  }

  let p = res.unwrap();
  let mut data_file = File::create(p).expect("creation failed");
  data_file.write(str.as_bytes()).expect("write failed");
}


fn track_modified_chunks(
  mut chunks_query: Query<&Chunks, Changed<Chunks>>,
  mut local_res: ResMut<LocalResource>,
) {
  for c in &chunks_query {
    for mesh in c.data.iter() {
      if !mesh.chunk.is_default {
        local_res.chunks.insert(mesh.key.clone(), mesh.chunk.clone());
        // info!("chunks.len() {:?}", local_res.chunks.len());
      }
    }
    
  }
}

#[derive(Resource)]
struct LocalResource {
  chunks: HashMap<[i64; 3], Chunk>,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      chunks: HashMap::default(),
    }
  }
}


