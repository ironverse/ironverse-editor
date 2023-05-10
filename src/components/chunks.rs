use bevy::prelude::*;
use rapier3d::{na::{Point, Isometry}, prelude::{ColliderBuilder, InteractionGroups, Group, ColliderHandle}};
use voxels::{chunk::{adjacent_keys, chunk_manager::{ChunkManager, Chunk}}, utils::{key_to_world_coord_f32, posf32_to_world_key}, data::voxel_octree::{VoxelMode, MeshData}};
use crate::{states::GameState, data::GameResource, physics::Physics, utils::{nearest_voxel_point_0, nearest_voxel_point}, wasm::WasmInputEvent, input::hotbar::HotbarResource};
use super::{player::Player, raycast::Raycast};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(
        enter.in_schedule(OnEnter(GameState::Start))
      )
      .add_system(
        spawn_on_add_player.in_set(OnUpdate(GameState::Play))
      )
      // .add_system(on_raycast)
      .add_system(add_meshes)
      .add_system(add_chunks.after(add_meshes));
  }
}

fn enter() {
  // Test terrain
  
}

fn spawn_on_add_player(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  mut physics: ResMut<Physics>,

  player_query: Query<(Entity, &Player), Added<Player>>,
) {
  for (entity, player) in &player_query {
    let mut meshes = Vec::new();

    let config = game_res.chunk_manager.config.clone();
    
    let keys = adjacent_keys(&player.key, 1, true);
    for key in keys.iter() {
      let chunk = ChunkManager::new_chunk(
        key, 
        config.depth, 
        config.lod, 
        game_res.chunk_manager.noise,
      );
  
      let data = chunk.octree.compute_mesh2(
        VoxelMode::SurfaceNets, 
        &mut game_res.chunk_manager.voxel_reuse
      );

      game_res.chunk_manager.set_chunk(key, &chunk);

      if data.indices.len() == 0 { // Temporary, should be removed once the ChunkMode detection is working
        continue;
      }
      
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

      meshes.push(Mesh {
        key: key.clone(),
        data: data.clone(),
        handle: handle,
      });
  
    }

    commands
      .entity(entity)
      .insert(Meshes {
        data: meshes
      })
      .insert(Chunks { data: Vec::new() });
  }
}

fn add_meshes(
  mut commands: Commands,
  mut raycasts: Query<(Entity, &Raycast, &mut Meshes), Changed<Raycast>>,
  mut game_res: ResMut<GameResource>,
  mut wasm_events: EventReader<WasmInputEvent>,

  mut physics: ResMut<Physics>,
  hotbar_res: Res<HotbarResource>,
  mut local_res: ResMut<LocalResource>,
) {

  let mut voxel_op = None;
  for e in wasm_events.iter() {
    if e.mouse == MouseButton::Left {
      voxel_op = Some(0);
    }

    if e.mouse == MouseButton::Right {
      // voxel_op = Some(1);
      for i in 0..hotbar_res.bars.len() {
        let bar = &hotbar_res.bars[i];
        if  hotbar_res.selected_keycode ==  bar.key_code {
          voxel_op = Some(bar.voxel);
        }
      }
      
      
    }
  }

  if voxel_op.is_none() {
    return;
  }

  let config = game_res.chunk_manager.config.clone();
  for (entity, raycast, mut meshes) in &mut raycasts {
    if raycast.point.x == f32::NAN {
      continue;
    }

    
    
    let mut res = Vec::new();
    let voxel = voxel_op.unwrap();

    // Add new
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
      local_res.res.push((entity, res.clone()));
    }

    // Delete
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
      local_res.res.push((entity, res.clone()));
    }
    
    for (key, chunk) in res.iter() {
      'inner: for i in 0..meshes.data.len() {
        let m = &meshes.data[i];

        if key == &m.key {
          physics.remove_collider(m.handle);
          meshes.data.swap_remove(i);
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


        meshes.data.push(Mesh {
          key: key.clone(),
          data: data.clone(),
          handle: handle,
        })
      } else {
        meshes.data.push(Mesh {
          key: key.clone(),
          data: data.clone(),
          handle: ColliderHandle::default(),
        })
      }


    }
  }


}

fn add_chunks(
  mut commands: Commands,
  mut local_res: ResMut<LocalResource>,
  mut chunks_query: Query<&mut Chunks>,
) {
  for (entity, chunks_data) in local_res.res.iter() {
    let res = chunks_query.get_mut(*entity);
    if res.is_ok() {
      let mut chunks = res.unwrap();
      chunks.data.clear();

      for (key, data) in chunks_data.iter() {
        chunks.data.push(data.clone());
      }
     
    }
  }

  local_res.res.clear();
}



fn on_raycast(
  mut raycasts: Query<(&Transform, &mut Raycast), Changed<Raycast>>,
  game_res: Res<GameResource>,

  mouse: Res<Input<MouseButton>>, // Not working on wasm, because of pointer lock
  mut wasm_events: EventReader<WasmInputEvent>,
) {
  for (trans, mut raycast) in &mut raycasts {
    if raycast.point.x != f32::NAN {
      
      let nearest_op = nearest_voxel_point_0(
        &game_res.chunk_manager, 
        raycast.point, 
        true
      );

      if nearest_op.is_some() {
        for e in wasm_events.iter() {
          if e.mouse == MouseButton::Left {
            let nearest = nearest_op.unwrap();
            info!("Raycast {:?}", nearest);
          }
          
        }

        // if mouse.just_pressed(MouseButton::Left) {
        //   let nearest = nearest_op.unwrap();
        //   // info!("Raycast {:?}: {:?}", raycast.point, nearest);
        //   info!("Raycast {:?}", nearest);
        // }

        

      }
    }
    
  }
}


#[derive(Component, Debug, Clone)]
pub struct Meshes {
  pub data: Vec<Mesh>,
}

#[derive(Component, Debug, Clone)]
pub struct Mesh {
  pub key: [i64; 3],
  pub data: MeshData,
  pub handle: ColliderHandle,
}

#[derive(Component, Debug, Clone)]
pub struct Chunks {
  pub data: Vec<Chunk>,
}



#[derive(Resource)]
struct LocalResource {
  res: Vec<(Entity, Vec<([i64; 3], Chunk)>)>
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      res: Vec::new(),
    }
  }
}