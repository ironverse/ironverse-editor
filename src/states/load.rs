use bevy::prelude::*;
use voxels::{data::voxel_octree::VoxelOctree, chunk::chunk_manager::Chunk};
use crate::{ui::menu::UIMenuResource, components::{save::Data, player::Player}, data::GameResource, physics::Physics};

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

/* 
  Clear everything in-game
    Defer better way of doing it
    For now just clear everything

  */
fn enter(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  player_query: Query<(Entity, &Player)>,
  
) {
  
}

/*  
  Have to find a way not to use UIMenuResource later

  Load terrain
    Send the data to components chunks and graphics chunks?
    Separate physics manager?
*/
fn update(
  mut commands: Commands,
  ui_menu_res: Res<UIMenuResource>,
  mut game_res: ResMut<GameResource>,
  mut next_state: ResMut<NextState<GameState>>,

  mut physics: ResMut<Physics>,
  player_query: Query<(Entity, &Player)>,
) {
  for file in ui_menu_res.recv.drain() {
    game_res.chunk_manager.chunks.clear();
    *physics = Physics::default();
    for (entity, _) in &player_query {
      commands.entity(entity).despawn_recursive();
    }


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
    game_res.data = data;
    next_state.set(GameState::Play);
  }
}


fn exit(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  player_query: Query<(Entity, &Player)>,
) {
  
}

/*
  Clear all chunk data
  Insert loaded data from file
  Start game
 */