use bevy::prelude::*;
use rapier3d::{na::{Point, Isometry}, prelude::{ColliderBuilder, InteractionGroups, Group, ColliderHandle}};
use voxels::{chunk::{adjacent_keys, chunk_manager::ChunkManager}, utils::{key_to_world_coord_f32, posf32_to_world_key}, data::voxel_octree::{VoxelMode, MeshData}};
use crate::{states::GameState, data::GameResource, physics::Physics, utils::nearest_voxel_point_0, wasm::WasmInputEvent};
use super::{player::Player, raycast::Raycast};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(
        enter.in_schedule(OnEnter(GameState::Start))
      )
      .add_system(
        spawn_on_add_player.in_set(OnUpdate(GameState::Play))
      )
      .add_system(on_raycast)
      .add_system(add);
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
    let mut chunks = Vec::new();

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

      chunks.push(Mesh {
        key: key.clone(),
        data: data.clone(),
        handle: handle,
      });
  
    }

    commands.spawn(Chunks {
      data: chunks
    });
  }
}

fn add(
  mut commands: Commands,
  mut raycasts: Query<(Entity, &Raycast, &mut Chunks), Changed<Raycast>>,
  mut game_res: ResMut<GameResource>,
  mut wasm_events: EventReader<WasmInputEvent>,

  mut physics: ResMut<Physics>,
) {
  // let mut mesh_data = Vec::new();

  let mut voxel = 0;
  for e in wasm_events.iter() {
    if e.mouse == MouseButton::Left {
      voxel = 1;
    }
    
  }

  let config = game_res.chunk_manager.config.clone();
  for (entity, raycast, chunks) in &mut raycasts {
    if raycast.point.x != f32::NAN {
      continue;
    }

    let pos = [raycast.point.x, raycast.point.y, raycast.point.z];
    // let res = game_res.chunk_manager.set_voxel2(&pos, voxel);

    // for (key, chunk) in res.iter() {
    //   let data = chunk.octree.compute_mesh2(
    //     VoxelMode::SurfaceNets, 
    //     &mut game_res.chunk_manager.voxel_reuse
    //   );

    //   if data.indices.len() == 0 { // Temporary, should be removed once the ChunkMode detection is working
    //     continue;
    //   }
    //   // chunks.data.push((key.clone(), data.clone());

    //   let pos_f32 = key_to_world_coord_f32(key, config.seamless_size);
    //   let mut pos = Vec::new();
    //   for d in data.positions.iter() {
    //     pos.push(Point::from([d[0], d[1], d[2]]));
    //   }
  
    //   let mut indices = Vec::new();
    //   for ind in data.indices.chunks(3) {
    //     // println!("i {:?}", ind);
    //     indices.push([ind[0], ind[1], ind[2]]);
    //   }
  
    //   let mut collider = ColliderBuilder::trimesh(pos, indices)
    //     .collision_groups(InteractionGroups::new(Group::GROUP_1, Group::GROUP_2))
    //     .build();
    //   collider.set_position(Isometry::from(pos_f32));
  
    //   let handle = physics.collider_set.insert(collider);
    // }
  }
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
pub struct Chunks {
  pub data: Vec<Mesh>,
}

#[derive(Component, Debug, Clone)]
pub struct Mesh {
  pub key: [i64; 3],
  pub data: MeshData,
  pub handle: ColliderHandle,
}