use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use rapier3d::prelude::{Point, ColliderBuilder, InteractionGroups, Isometry};
use rapier3d::geometry::Group;
use voxels::{data::voxel_octree::VoxelMode, utils::key_to_world_coord_f32};
use crate::components::raycast::Raycast;
use crate::data::Player;
use crate::graphics::{ChunkPreviewGraphics, GraphicsResource};
use crate::{data::GameResource, components::chunk_preview::ChunkPreview};
use super::chunks::{VOXEL_WEIGHT, VOXEL_TYPE_1};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(hook_to_player)
      .add_system(update);
  }
}

fn hook_to_player(
  mut commands: Commands,
  players: Query<Entity, Added<Player>>,
) {
  for entity in &players {
    commands
      .entity(entity)
      .insert(ChunkPreviewRender::default());
  }
}


fn update(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut game_res: ResMut<GameResource>,
  mut chunk_previews: Query<
    (Entity, &ChunkPreview, &mut ChunkPreviewRender), Changed<ChunkPreview>
  >,
  mut materials: ResMut<Assets<StandardMaterial>>,

  // mut local: Local<bool>,
  graphics_res: Res<GraphicsResource>,
) {
  let config = game_res.chunk_manager.config.clone();

  for (entity, chunk_preview, mut render) in &mut chunk_previews {
    for e in render.entities.iter() {
      commands.entity(*e).despawn_recursive();
    }
    render.entities.clear();

    for (key, chunk) in chunk_preview.chunks.iter() {
      // if *local {
      //   return;
      // }
  
      // if !*local {
      //   *local = true;
      // }
      // info!("chunk_preview");
      
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

        let chunk_size = (chunk.octree.get_size() / 2) as f32;
        let p = &chunk_preview.new;
        let adj = [p[0] as f32, p[1] as f32, p[2] as f32];
        let coord_f32 = [adj[0] - chunk_size, adj[1] - chunk_size, adj[2] - chunk_size];
        
        let mut visibility = Visibility::Visible;
        if !graphics_res.show_preview {
          visibility = Visibility::Hidden;
        }
        let entity = commands
          .spawn(MaterialMeshBundle {
            visibility: visibility,
            mesh: meshes.add(render_mesh),
            material: materials.add(Color::rgba(0.0, 0.0, 1.0, 0.25).into()),
            transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
            ..default()
          })
          .insert(ChunkPreviewGraphics { })
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
