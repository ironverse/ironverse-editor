use bevy::{prelude::*, input::ButtonState};
use rapier3d::prelude::{ColliderBuilder, InteractionGroups, Isometry, Point};
use voxels::{data::voxel_octree::VoxelMode, utils::key_to_world_coord_f32};
use crate::{physics::Physics, data::{GameResource, CursorState}, utils::{nearest_voxel_point, nearest_voxel_point_0}, input::{MouseInput, hotbar::HotbarResource}};
use super::{raycast::Raycast, chunks::{Chunks, Mesh}};
use rapier3d::geometry::Group;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update);
  }
}


/*
  Adapter for Mouse event for wasm and native
 */

fn update(
  mut commands: Commands,
  mut raycasts: Query<(Entity, &Raycast, &mut Chunks), Changed<Raycast>>,
  mut game_res: ResMut<GameResource>,
  // mut wasm_events: EventReader<WasmInputEvent>,

  mut physics: ResMut<Physics>,
  hotbar_res: Res<HotbarResource>,
  mut mouse_inputs: EventReader<MouseInput>,
  
) {
  let mut voxel_op = None;
  for event in mouse_inputs.iter() {
    if event.mouse_button_input.state == ButtonState::Pressed {
      // info!("clicked");

      if event.mouse_button_input.button == MouseButton::Left {
        voxel_op = Some(0);
      }
      
      if event.mouse_button_input.button == MouseButton::Right {
        voxel_op = Some(1);
        for i in 0..hotbar_res.bars.len() {
          let bar = &hotbar_res.bars[i];
          if  hotbar_res.selected_keycode ==  bar.key_code {
            voxel_op = Some(bar.voxel);
          }
        }
      }
    }
  }

  if voxel_op.is_none() {
    return;
  }

  let config = game_res.chunk_manager.config.clone();
  for (entity, raycast, mut chunks) in &mut raycasts {
    if raycast.point.x == f32::NAN {
      continue;
    }

    
    
    let mut res = Vec::new();
    let voxel = voxel_op.unwrap();

    // Delete
    if voxel == 0 {
      let nearest_op = nearest_voxel_point_0(
        &game_res.chunk_manager, 
        raycast.point, 
        true
      );
      if nearest_op.is_none() {
        continue;
      }
      res = game_res.chunk_manager.set_voxel2(&nearest_op.unwrap(), voxel);
    }

    // Add
    if voxel > 0 {
      let nearest_op = nearest_voxel_point(
        &game_res.chunk_manager, 
        raycast.point, 
        true,
        0
      );
  
      if nearest_op.is_none() {
        continue;
      }

      
      res = game_res.chunk_manager.set_voxel2(&nearest_op.unwrap(), voxel);
    }
    
    for (key, chunk) in res.iter() {
      'inner: for i in 0..chunks.data.len() {
        let m = &chunks.data[i];

        if key == &m.key {
          physics.remove_collider(m.handle);
          chunks.data.swap_remove(i);
          break 'inner;
        }
      }
      

      let data = chunk.octree.compute_mesh2(
        VoxelMode::SurfaceNets, 
        &mut game_res.chunk_manager.voxel_reuse
      );

      
      if data.indices.len() > 0 { // Temporary, should be removed once the ChunkMode detection is working
        
        let pos_f32 = key_to_world_coord_f32(key, config.seamless_size);
        let mut pos = Vec::new();
        for d in data.positions.iter() {
          pos.push(Point::from([d[0], d[1], d[2]]));
        }
    
        let mut indices = Vec::new();
        for ind in data.indices.chunks(3) {
          // println!("i {:?}", ind);
          indices.push([ind[0], ind[1], ind[2]]);
        }
    
        let mut collider = ColliderBuilder::trimesh(pos, indices)
          .collision_groups(InteractionGroups::new(Group::GROUP_1, Group::GROUP_2))
          .build();
        collider.set_position(Isometry::from(pos_f32));
    
        let handle = physics.collider_set.insert(collider);


        let mut c = chunk.clone();
        c.is_default = false;
        chunks.data.push(Mesh {
          key: key.clone(),
          chunk: c,
          data: data.clone(),
          handle: handle,
        })
      }


    }
  }


}



#[derive(Component)]
pub struct ChunkEdit { }

impl Default for ChunkEdit {
  fn default() -> Self {
    Self { }
  }
}