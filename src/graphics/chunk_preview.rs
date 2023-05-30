use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use rapier3d::prelude::{Point, ColliderBuilder, InteractionGroups, Isometry};
use rapier3d::geometry::Group;
use voxels::{data::voxel_octree::VoxelMode, utils::key_to_world_coord_f32};
use crate::{data::GameResource, components::chunk_preview::ChunkPreview};

use super::chunks::{ChunkTexture, CustomMaterial, VOXEL_WEIGHT, VOXEL_TYPE_1};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update);
  }
}

fn update(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut game_res: ResMut<GameResource>,
  mut chunk_previews: Query<
    (Entity, &ChunkPreview, &mut ChunkPreviewRender), Changed<ChunkPreview>
  >,

  chunk_texture: Res<ChunkTexture>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
  let config = game_res.chunk_manager.config.clone();

  for (entity, chunk_preview, mut render) in &mut chunk_previews {
    // info!("Test");
    for e in render.entities.iter() {
      commands.entity(*e).despawn_recursive();
    }
    render.entities.clear();

    for (key, chunk) in chunk_preview.chunks.iter() {
      info!("chunk_preview");
      
      let data = chunk.octree.compute_mesh2(
        VoxelMode::SurfaceNets, 
        &mut game_res.chunk_manager.voxel_reuse
      );

      if data.indices.len() > 0 { // Temporary, should be removed once the ChunkMode detection is working
        let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
        render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
        render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));
    
        render_mesh.insert_attribute(VOXEL_WEIGHT, data.weights.clone());
        render_mesh.insert_attribute(VOXEL_TYPE_1, data.types_1.clone());
    
        let mesh_handle = meshes.add(render_mesh);
        let material_handle = custom_materials.add(CustomMaterial {
          base_color: Color::rgb(0.0, 0.0, 1.0),
          albedo: chunk_texture.albedo.clone(),
          normal: chunk_texture.normal.clone(),
        });

        let chunk_size = (chunk.octree.get_size() / 2) as f32;
        let coord_f32 = [-chunk_size, -chunk_size, -chunk_size];
    
        // let coord_f32 = key_to_world_coord_f32(key, config.seamless_size);
        let entity = commands
          .spawn(MaterialMeshBundle {
            mesh: mesh_handle,
            material: material_handle,
            transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
              // .with_scale(Vec3::new(0.99, 0.999, 0.99 )),
            ..default()
          })
          .id();

        render.entities.push(entity);
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
