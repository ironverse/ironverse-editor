use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use voxels::chunk::chunk_manager::Chunk;
use voxels::chunk::chunk_manager::ChunkManager;
use crate::{ui::UIState, wasm::html_body};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use super::chunks::Chunks;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default());
    // app
    //   .add_system(download.in_schedule(OnEnter(UIState::Save)));

    // app
    //   .add_startup_system(download);

    app
      .add_system(track_modified_chunks)
      .add_system(download.in_schedule(OnEnter(UIState::Save)));
  }
}

/*
  Create an example file to be downloaded

 */

fn download(
  local_res: Res<LocalResource>,
  mut next_state: ResMut<NextState<UIState>>,
) {
  let body = html_body();
  let res = body.query_selector("#download");
  
  let a_ops = match res {
    Ok(ops) => ops,
    Err(e) => { 
      info!("{:?}", e);
      return ()
    }
  };

  if a_ops.is_some() {
    let chunk_manager = ChunkManager::default();
    let chunk = chunk_manager.new_chunk3(&[0, -1, 0], 4);
    
    let mut keys = vec![];
    let mut voxels = vec![];
    for (key, chunk) in local_res.chunks.iter() {
      keys.push(key.clone());
      voxels.push(array_bytes::bytes2hex("", &chunk.octree.data));
    }
    

    // keys.push([0, 0, 0]);
    // voxels.push(array_bytes::bytes2hex("", chunk.octree.data));

    let data = Data {
      player: Player {
        position: [0.0, 1.0, 0.0],
      },
      terrains: Terrains { keys: keys, voxels: voxels }
    };

    let str = toml::to_string_pretty(&data).unwrap();
    

    let parts = js_sys::Array::of1(&unsafe {
      js_sys::Uint8Array::view(str.as_bytes())
          .into()
    });
    let blob_res = web_sys::Blob::new_with_u8_array_sequence(&parts);
    if blob_res.is_err() {
      return;
    }
    let blob = blob_res.unwrap();

    let url_res = web_sys::Url::create_object_url_with_blob(&blob);
    if url_res.is_err() {
      return;
    }

    let a = a_ops.unwrap();
    a.set_attribute("download", "save.toml");
    a.set_attribute("href", &url_res.unwrap());
    let a1: HtmlElement = a.dyn_into::<HtmlElement>().unwrap();
    a1.click();
  }


  next_state.set(UIState::Default);
}


fn track_modified_chunks(
  mut chunks_query: Query<&Chunks, Changed<Chunks>>,
  mut local_res: ResMut<LocalResource>,
) {
  for c in &chunks_query {
    for chunk in c.data.iter() {
      local_res.chunks.insert(chunk.key.clone(), chunk.clone());

    }
    info!("chunks.len() {:?}", local_res.chunks.len());
  }
}



#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Data {
  pub player: Player,
  pub terrains: Terrains,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Player {
  pub position: [f32; 3]
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Terrains {
  pub keys: Vec<[i64; 3]>,
  pub voxels: Vec<String>,
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