use bevy::{prelude::*, render::{render_resource::{PrimitiveTopology, VertexFormat}, mesh::{MeshVertexAttribute, Indices}}};
use voxels::{utils::key_to_world_coord_f32, chunk::adjacent_keys};
use crate::{data::{GameResource}, components::{chunk::Chunks, player::Player}, graphics::{ChunkGraphics}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(startup)
      .add_system(add)
      .add_system(remove)
      ;
  }
}

fn startup(
  mut commands: Commands, 
) {
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 3000.0,
      ..Default::default()
    },
    transform: Transform::from_xyz(0.0, 5.0, 0.0),
    ..Default::default()
  });
}

fn add(
  game_res: Res<GameResource>,

  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  chunk_query: Query<(Entity, &Chunks), Changed<Chunks>>,
) {
  let config = game_res.chunk_manager.config.clone();
  for (_, chunks) in &chunk_query {
    for mesh in &chunks.data {
      
      'inner: for (entity, graphics) in &chunk_graphics {
        if mesh.key == graphics.key {
          commands.entity(entity).despawn_recursive();
          break 'inner;
        }
      }

      let data = &mesh.data;

      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

      render_mesh.insert_attribute(VOXEL_WEIGHT, data.weights.clone());
      render_mesh.insert_attribute(VOXEL_TYPE_1, data.types_1.clone());

      let mesh_handle = meshes.add(render_mesh);

      let coord_f32 = key_to_world_coord_f32(&mesh.key, config.seamless_size);
      commands
        .spawn(MaterialMeshBundle {
          mesh: mesh_handle,
          material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
          transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
          ..default()
        })
        .insert(ChunkGraphics { key: mesh.key.clone() })
        ;
      
    }
    
  }
}

fn remove(
  mut commands: Commands,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  chunk_query: Query<(Entity, &Chunks, &Player), Changed<Chunks>>,
) {
  for (_, _chunks, player) in &chunk_query {
    let adj_keys = adjacent_keys(&player.key, 1, true);
    for (entity, graphics) in &chunk_graphics {
      if !adj_keys.contains(&graphics.key) {
        commands.entity(entity).despawn_recursive();
      }
    }
  
    
  }
}


pub const VOXEL_WEIGHT: MeshVertexAttribute =
  MeshVertexAttribute::new("Voxel_Weight", 988540917, VertexFormat::Float32x4);

pub const VOXEL_TYPE_1: MeshVertexAttribute =
  MeshVertexAttribute::new("Voxel_Type_1", 988540918, VertexFormat::Uint32x4);


