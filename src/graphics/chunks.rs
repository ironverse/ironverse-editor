use bevy::{prelude::*, render::{mesh::{MeshVertexAttribute, MeshVertexBufferLayout, Indices}, render_resource::{VertexFormat, AsBindGroup, ShaderRef, SpecializedMeshPipelineError, RenderPipelineDescriptor, PrimitiveTopology}}, reflect::TypeUuid, pbr::{MaterialPipeline, MaterialPipelineKey}, asset::LoadState};
use voxels::{chunk::{adjacent_keys, chunk_manager::ChunkManager}, utils::key_to_world_coord_f32, data::voxel_octree::VoxelMode};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_plugin(MaterialPlugin::<CustomMaterial>::default())
      .add_startup_system(startup)
      .add_system(add)
      ;
  }
}


fn startup(
  mut commands: Commands, 
  asset_server: Res<AssetServer>,
) {
  commands.insert_resource(ChunkTexture {
    is_loaded: false,
    // albedo: asset_server.load("textures/array_texture.png"),
    albedo: asset_server.load("textures/terrain_albedo.png"),
    normal: asset_server.load("textures/terrain_normal.png"),
  });

  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 3000.0,
      ..Default::default()
    },
    transform: Transform::from_xyz(-3.0, 2.0, -1.0),
    ..Default::default()
  });
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 3000.0,
      ..Default::default()
    },
    transform: Transform::from_xyz(3.0, 2.0, 1.0),
    ..Default::default()
  });
}

fn add(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,
  mut _materials: ResMut<Assets<StandardMaterial>>,
  terrains: Query<(Entity, &TerrainGraphics)>,
  asset_server: Res<AssetServer>,

  mut loading_texture: ResMut<ChunkTexture>,
  local_res: Res<LocalResource>,
  mut images: ResMut<Assets<Image>>,
) {
  if loading_texture.is_loaded
    || asset_server.get_load_state(loading_texture.albedo.clone()) != LoadState::Loaded
    || asset_server.get_load_state(loading_texture.normal.clone()) != LoadState::Loaded
  {
    return;
  }
  loading_texture.is_loaded = true;

  let array_layers = 16; // WebGPU implementation not allowing 12 layers
  let image = images.get_mut(&loading_texture.albedo).unwrap();
  image.reinterpret_stacked_2d_as_array(array_layers);

  let normal = images.get_mut(&loading_texture.normal).unwrap();
  normal.reinterpret_stacked_2d_as_array(array_layers);

  // println!("add {}", local_res.output_cache.len());

  let mut chunk_manager = ChunkManager::default();

  for key in local_res.keys.iter() {
    for (entity, terrain) in terrains.iter() {
      if key == &terrain.key {
        commands.entity(entity).despawn_recursive();
      }
    }

    let chunk = ChunkManager::new_chunk(
      key, 
      chunk_manager.config.depth, 
      chunk_manager.config.lod, 
      chunk_manager.noise,
    );

    let data = chunk.octree.compute_mesh2(VoxelMode::SurfaceNets, &mut chunk_manager.voxel_reuse);

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    render_mesh.insert_attribute(VOXEL_WEIGHT, data.weights.clone());
    render_mesh.insert_attribute(VOXEL_TYPE_1, data.types_1.clone());


    // let mut render_mesh = Mesh::from(shape::Cube { size: 1.0 });
    // render_mesh.insert_attribute(
    //     ATTRIBUTE_BLEND_COLOR,
    //     // The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each
    //     vec![[1.0, 0.0, 0.0, 1.0]; 24],
    // );


    let mesh_handle = meshes.add(render_mesh);
    let material_handle = custom_materials.add(CustomMaterial {
      albedo: loading_texture.albedo.clone(),
      normal: loading_texture.normal.clone(),
    });

    let seamless_size = chunk_manager.seamless_size();
    let coord_f32 = key_to_world_coord_f32(key, seamless_size);
    commands
      .spawn(MaterialMeshBundle {
        mesh: mesh_handle,
        material: material_handle,
        transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
        ..default()
      })
      // .insert(TerrainGraphics {key: *key })
      ;




    // let seamless_size = chunk_manager.seamless_size();
    // let coord_f32 = key_to_world_coord_f32(key, seamless_size);
    // commands.spawn((
    //   PbrBundle {
    //     mesh: mesh_handle,
    //     material: materials.add(Color::SILVER.into()),
    //     transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
    //     ..default()
    //   },
    //   TerrainGraphics{ key: *key },
    // ));
    
  }
}

#[derive(Resource)]
struct LocalResource {
  keys: Vec<[i64; 3]>,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      keys: adjacent_keys(&[0, 0, 0], 1, true),
    }
  }
}


#[derive(Resource)]
struct ChunkTexture {
  is_loaded: bool,
  albedo: Handle<Image>,
  normal: Handle<Image>,
}


pub const VOXEL_WEIGHT: MeshVertexAttribute =
  MeshVertexAttribute::new("Voxel_Weight", 988540917, VertexFormat::Float32x4);

pub const VOXEL_TYPE_1: MeshVertexAttribute =
  MeshVertexAttribute::new("Voxel_Type_1", 988540918, VertexFormat::Uint32x4);



#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "5f2e1d29-b8ad-4680-8c96-f8b78a580718"]
struct CustomMaterial {
  #[texture(0, dimension = "2d_array")]
  #[sampler(1)]
  albedo: Handle<Image>,
  #[texture(2, dimension = "2d_array")]
  #[sampler(3)]
  normal: Handle<Image>,
}

impl Material for CustomMaterial {
  fn vertex_shader() -> ShaderRef {
    "shaders/triplanar.wgsl".into()
  }
  fn fragment_shader() -> ShaderRef {
    "shaders/triplanar.wgsl".into()
  }
  fn specialize(
    _pipeline: &MaterialPipeline<Self>,
    descriptor: &mut RenderPipelineDescriptor,
    layout: &MeshVertexBufferLayout,
    _key: MaterialPipelineKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
    let vertex_layout = layout.get_layout(&[
      Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
      Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
      VOXEL_WEIGHT.at_shader_location(2),
      VOXEL_TYPE_1.at_shader_location(3),
    ])?;
    descriptor.vertex.buffers = vec![vertex_layout];

    Ok(())
  }
}

#[derive(Component)]
pub struct TerrainGraphics {
  pub key: [i64; 3]
}