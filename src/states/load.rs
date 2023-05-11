use bevy::prelude::*;
use voxels::{data::voxel_octree::VoxelOctree, chunk::chunk_manager::Chunk};
use crate::{ui::menu::UIMenuResource, components::save::Data, data::GameResource};

use super::GameState;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(enter.in_schedule(OnEnter(GameState::Load)))
      .add_system(update.in_set(OnUpdate(GameState::Load)))
      .add_system(exit.in_schedule(OnExit(GameState::Load)))
      ;
  }
}


fn enter(mut game_res: ResMut<GameResource>,) {
  /* Clear everything in-game */
  game_res.chunk_manager.chunks.clear();
}

/*  
  Have to find a way not to use UIMenuResource later

  Load terrain
    Send the data to components chunks and graphics chunks?
    Separate physics manager?
*/
fn update(
  ui_menu_res: Res<UIMenuResource>,
  mut game_res: ResMut<GameResource>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  for file in ui_menu_res.recv.drain() {

    info!("file");
    let s = match String::from_utf8(file) {
      Ok(v) => v,
      Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // info!("result: {}", s);
    let res = toml::from_str::<Data>(s.as_str());
    if res.is_err() {
      return;
    }
    let data = res.unwrap();
    info!("load data");

    for i in 0..data.terrains.keys.len() {
      let key = &data.terrains.keys[i];
      let voxels_str = &data.terrains.voxels[i];
      let voxels_res = array_bytes::hex2bytes(voxels_str);
      if voxels_res.is_ok() {
        let data = voxels_res.unwrap();
        let octree = VoxelOctree::new_from_bytes(data);
        let chunk = Chunk {
          key: key.clone(),
          octree: octree,
          is_default: false,
          ..Default::default()
        };
        game_res.chunk_manager.set_chunk(key, &chunk);

        info!("load data key {:?}", key);
      }
    }

    next_state.set(GameState::Play);
  }
}


fn exit() {

}

/*
  Clear all chunk data
  Insert loaded data from file
  Start game
 */