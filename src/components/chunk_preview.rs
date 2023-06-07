use bevy::prelude::*;
use voxels::chunk::{chunk_manager::Chunk, adjacent_keys};
use crate::{data::{GameResource, Player}, utils::{nearest_voxel_point_0, nearest_voxel_point}};
use super::raycast::Raycast;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(hook_to_player)
      .add_system(on_add);
  }
}

fn hook_to_player(
  mut commands: Commands,
  mut players: Query<(Entity), Added<Player>>,
) {
  for entity in &players {
    commands
      .entity(entity)
      .insert(ChunkPreview::default());
  }
}

/*
  Have to toggle it on/off
  Create a single chunk just for previewing
  Able to move it, translate the position to world position
    Relative to the original chunk
    
 */

fn on_add(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  mut raycasts: Query<
  (Entity, &Raycast, &mut ChunkPreview), Changed<Raycast>
  >,
) {


  for (entity, raycast, mut chunk_preview) in &mut raycasts {
    if raycast.point.x == f32::NAN {
      continue;
    }

    game_res.preview_chunk_manager.chunks = game_res.chunk_manager.chunks.clone();

    let nearest_op = nearest_voxel_point_0(
      &game_res.chunk_manager, 
      raycast.point, 
      true
    );

    if nearest_op.is_none() { continue; }
    let target = nearest_op.unwrap();
    if chunk_preview.target != target {
      chunk_preview.target = target;

      let new_op = nearest_voxel_point(
        &game_res.chunk_manager, 
        raycast.point, 
        true,
        0
      );
    }


    
    let new_op = nearest_voxel_point(
      &game_res.chunk_manager, 
      raycast.point, 
      true,
      0
    );

    if new_op.is_none() { continue; }
    let new = new_op.unwrap();

    if chunk_preview.new != new {
      chunk_preview.new = new.clone();

      let res = game_res.preview_chunk_manager.set_voxel2(&new, 1);
      chunk_preview.chunks.clear();
      // chunk_preview.chunks.push((chunk.key, chunk));


      let mut chunk = Chunk::default();
      let pos = chunk.octree.get_size() / 2;

      let range = 1;
      for x in -range..range + 1 {
        for y in -range..range + 1 {
          for z in -range..range + 1{
            let p_x = new[0] + x;
            let p_y = new[1] + y;
            let p_z = new[2] + z;

            let val = game_res.preview_chunk_manager.get_voxel(&[p_x, p_y, p_z]);

            let local_x = pos as i64 + x;
            let local_y = pos as i64 + y;
            let local_z = pos as i64 + z;
            chunk.octree.set_voxel(local_x as u32, local_y as u32, local_z as u32, val);
          }
        }
      }

      chunk_preview.chunks.push((chunk.key, chunk));
    }
  }

/*   
  for (entity, raycast, mut chunk_preview) in &mut raycasts {
    if raycast.point.x == f32::NAN {
      continue;
    }

    game_res.preview_chunk_manager.chunks = game_res.chunk_manager.chunks.clone();

    let nearest_op = nearest_voxel_point_0(
      &game_res.chunk_manager, 
      raycast.point, 
      true
    );
    if nearest_op.is_none() { continue; }

    let nearest = nearest_op.unwrap();
    if chunk_preview.target != nearest {
      chunk_preview.target = nearest;

      let nearest_new_op = nearest_voxel_point(
        &game_res.chunk_manager, 
        raycast.point, 
        true,
        0
      );

      if nearest_new_op.is_none() { continue; }
      let nearest_new = nearest_new_op.unwrap();

      let res = game_res.preview_chunk_manager.set_voxel2(&nearest_new, 1);
      chunk_preview.chunks = res;
    }
  }
 */
}


#[derive(Component, Clone)]
pub struct ChunkPreview {
  pub target: [i64; 3],
  pub new: [i64; 3],
  pub chunks: Vec<([i64; 3], Chunk)>
}

impl Default for ChunkPreview {
  fn default() -> Self {
    Self {
      target: [i64::MAX; 3],
      new: [i64::MAX; 3],
      chunks: Vec::new(),
    }
  }
}
