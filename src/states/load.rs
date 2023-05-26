use bevy::prelude::*;
use voxels::{data::voxel_octree::VoxelOctree, chunk::chunk_manager::Chunk};
use crate::{data::{GameResource, GameState, Player}, physics::Physics, graphics::chunks::TerrainGraphics, ui::UIState};


#[cfg(target_arch = "wasm32")]
use crate::ui::menu::UIMenuResource;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(enter.in_schedule(OnEnter(GameState::Load)))
      .add_system(exit.in_schedule(OnExit(GameState::Load)))
      ;

    // #[cfg(target_arch = "wasm32")]
    // app
    //   .add_system(update.in_set(OnUpdate(GameState::Load)));
  }
}

fn enter(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  mut physics: ResMut<Physics>,
  player_query: Query<(Entity, &Player)>,
  mut game_state_next: ResMut<NextState<GameState>>,
  mut ui_state_next: ResMut<NextState<UIState>>,


  terrain_graphics: Query<Entity, With<TerrainGraphics>>,
) {
  // Clear the game
  // Start new game

  game_res.chunk_manager.chunks.clear();
  *physics = Physics::default();
  
  for (entity, _) in &player_query {
    commands.entity(entity).despawn_recursive();
  }
  
  for entity in &terrain_graphics {
    commands.entity(entity).despawn_recursive();
  }


  let data = game_res.data.clone();
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

      // info!("load data key {:?}", key);
    }
  }



  ui_state_next.set(UIState::Default);
  game_state_next.set(GameState::Start);
  info!("Enter GameState::Load");
}


/* 
/*  
  Have to find a way not to use UIMenuResource later

  Load terrain
    Send the data to components chunks and graphics chunks?
    Separate physics manager?
*/
#[cfg(target_arch = "wasm32")]
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
 */

fn exit(
  // mut commands: Commands,
  // mut game_res: ResMut<GameResource>,
  // player_query: Query<(Entity, &Player)>,
) {
  
}

/*
  Clear all chunk data
  Insert loaded data from file
  Start game
 */