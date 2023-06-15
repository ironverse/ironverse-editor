use bevy::{prelude::*, utils::HashMap};
use voxels::chunk::{chunk_manager::Chunk};
use crate::{data::{GameResource, Player}, utils::{nearest_voxel_point_0, nearest_voxel_point}, input::hotbar::HotbarResource};
use super::{raycast::Raycast, range::Range};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(hook_to_player)
      .add_system(on_add)
      .add_system(on_range)
      // .add_system(toggle_showhide)
      ;
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

fn on_add(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  mut raycasts: Query<
  (Entity, &Raycast, &mut ChunkPreview), Changed<Raycast>
  >,

  hotbar_res: Res<HotbarResource>,
) {


  for (entity, raycast, mut chunk_preview) in &mut raycasts {
    info!("raycasts");
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
      info!("create");
      chunk_preview.new = new.clone();

      let bar_op = hotbar_res
        .bars
        .iter()
        .find(|bar| bar.key_code == hotbar_res.selected_keycode);

      let mut voxel = 0;
      if bar_op.is_some() {
        voxel = bar_op.unwrap().voxel;
      }


      let res = game_res.preview_chunk_manager.set_voxel2(&new, voxel);
      chunk_preview.chunks.clear();
      // chunk_preview.chunks.push((chunk.key, chunk));


      let mut chunk = Chunk::default();
      let pos = chunk.octree.get_size() / 2;

      let range = 2;
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
}


fn on_range(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  mut ranges: Query<
  (Entity, &Range, &mut ChunkPreview), Changed<Range>
  >,
) {
  for (entity, range, mut chunk_preview) in &mut ranges {
    if range.point.x == f32::NAN {
      continue;
    }

    game_res.preview_chunk_manager.chunks = game_res.chunk_manager.chunks.clone();

    let min = -(range.scale as i64);
    let max = (range.scale as i64) + 1;

    // info!("min {} max {}", min, max);

    chunk_preview.chunks.clear();
    chunk_preview.new = [
      range.point.x as i64,
      range.point.y as i64,
      range.point.z as i64,
    ];

    let mut chunk = Chunk::default();
    let chunk_pos = chunk.octree.get_size() / 2;

    for x in min..max {
      // info!("x {}", x);

      for y in min..max {
        for z in min..max {

          let pos = [
            range.point.x as i64 + x,
            range.point.y as i64 + y,
            range.point.z as i64 + z
          ];

          // game_res.preview_chunk_manager.set_voxel2(&pos, voxel);

          // info!("pos {:?}", pos);

          // let val = game_res.preview_chunk_manager.get_voxel(&pos);

          let local_x = chunk_pos as i64 + x;
          let local_y = chunk_pos as i64 + y;
          let local_z = chunk_pos as i64 + z;
          chunk.octree.set_voxel(local_x as u32, local_y as u32, local_z as u32, 1);
        }
      }
    }
    chunk_preview.chunks.push((chunk.key, chunk));
  }

  
}


fn toggle_showhide(
  key_input: Res<Input<KeyCode>>,
  mut chunk_previews: Query<(&mut Visibility, &ChunkPreview)>,
) {
  if key_input.just_pressed(KeyCode::P) {
    info!("chunk_previes.len() {}", chunk_previews.iter().len());
    for (mut visibility, preview) in &mut chunk_previews {
      *visibility = Visibility::Hidden;
      info!("Hide");
    }
  }
}

#[derive(Component, Clone)]
pub struct ChunkPreview {
  pub target: [i64; 3],
  pub new: [i64; 3],
  pub chunks: Vec<([i64; 3], Chunk)>,
  pub is_showing: bool,
}

impl Default for ChunkPreview {
  fn default() -> Self {
    Self {
      target: [i64::MAX; 3],
      new: [i64::MAX; 3],
      chunks: Vec::new(),
      is_showing: true,
    }
  }
}
