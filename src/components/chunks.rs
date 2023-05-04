use bevy::prelude::*;
use rapier3d::{na::{Point, Isometry}, prelude::{ColliderBuilder, InteractionGroups, Group}};
use voxels::{chunk::{adjacent_keys, chunk_manager::ChunkManager}, utils::key_to_world_coord_f32, data::voxel_octree::{VoxelMode, MeshData}};
use crate::{states::GameState, data::GameResource, physics::Physics};
use super::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(
        enter.in_schedule(OnEnter(GameState::Start))
      )
      .add_system(
        update.in_set(OnUpdate(GameState::Play))
      );
  }
}

fn enter() {
  // Test terrain
  
}

fn update(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  mut physics: ResMut<Physics>,

  player_query: Query<(Entity, &Player), Added<Player>>,
) {
  for (entity, player) in &player_query {
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
      // self.collider_handles.insert(key.clone(), handle.clone());
      // meshes.push((*key, data));
      
      commands.spawn(Chunk {
        key: key.clone(),
        mesh_data: data.clone() 
      });
    }
  }
}

#[derive(Component, Debug, Clone)]
pub struct Chunk {
  pub key: [i64; 3],
  pub mesh_data: MeshData
}