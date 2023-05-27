use bevy::prelude::*;
use rapier3d::prelude::{Point, ColliderBuilder, InteractionGroups, Isometry};
use rapier3d::geometry::Group;
use voxels::{data::voxel_octree::VoxelMode, utils::key_to_world_coord_f32};
use crate::{data::GameResource, components::chunk_preview::ChunkPreview};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update);
  }
}

fn update(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  mut chunk_previews: Query<
    (Entity, &ChunkPreview, &mut ChunkPreviewRender), Changed<ChunkPreview>
  >,
) {

  for (entity, chunk_preview, mut render) in &mut chunk_previews {
    for e in render.entities.iter() {
      commands.entity(*e).despawn_recursive();
    }

    for (key, chunk) in chunk_preview.chunks.iter() {
      let data = chunk.octree.compute_mesh2(
        VoxelMode::SurfaceNets, 
        &mut game_res.chunk_manager.voxel_reuse
      );

      if data.indices.len() > 0 { // Temporary, should be removed once the ChunkMode detection is working

      }
    }
  }
}



#[derive(Component)]
pub struct ChunkPreviewRender {
  entities: Vec<Entity>,
}

impl Default for ChunkPreviewRender {
  fn default() -> Self {
    Self {
      entities: Vec::new(),
    }
  }
}
