use bevy::{prelude::*, render::{render_resource::{PrimitiveTopology, VertexFormat}, mesh::{Indices, MeshVertexAttribute}}};
use bevy_flycam::FlyCam;
use voxels::{data::{voxel_octree::{VoxelOctree, ParentValueType, VoxelMode}, surface_nets::VoxelReuse}, utils::key_to_world_coord_f32, chunk::chunk_manager::ChunkManager};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(setup_camera)
      .add_startup_system(startup);
  }
}

fn setup_camera(
  mut commands: Commands,
) {
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_xyz(0.0, 2.0, -5.0).looking_to(Vec3::Z, Vec3::Y),
      ..Default::default()
    })
    .insert(FlyCam);

  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 6000.0,
      ..Default::default()
    },
    transform: Transform::from_xyz(0.0, 15.0, 0.0),
    ..Default::default()
  });
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let start = 2;
  let end = 14;

  let mut voxels = Vec::new();
  for x in start..end {
    for y in start..end {
      for z in start..end {

        if y < 7 {
          voxels.push([x as u32, y as u32, z as u32, 1 as u32]);
        }
      }
    }
  }

  let mut octree = VoxelOctree::new_from_3d_array(
    0, 
    4, 
    &voxels, ParentValueType::DefaultValue
  );

  let mut manager = ChunkManager::default();
  let data = octree.compute_mesh2(VoxelMode::SurfaceNets, &mut manager.voxel_reuse);

  let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
  render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

  render_mesh.insert_attribute(VOXEL_WEIGHT, data.weights.clone());
  render_mesh.insert_attribute(VOXEL_TYPE_1, data.types_1.clone());

  let mesh_handle = meshes.add(render_mesh);

  let coord_f32 = key_to_world_coord_f32(&[0, 0, 0], manager.config.seamless_size);
  commands
    .spawn(MaterialMeshBundle {
      mesh: mesh_handle,
      material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
      transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
      ..default()
    });
}

pub const VOXEL_WEIGHT: MeshVertexAttribute =
  MeshVertexAttribute::new("Voxel_Weight", 988540917, VertexFormat::Float32x4);

pub const VOXEL_TYPE_1: MeshVertexAttribute =
  MeshVertexAttribute::new("Voxel_Type_1", 988540918, VertexFormat::Uint32x4);




/*
  Data and rendering include here
  Just separate later

*/