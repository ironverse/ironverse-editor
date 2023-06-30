use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use rapier3d::prelude::{Point, ColliderBuilder, InteractionGroups, Isometry};
use voxels::chunk::chunk_manager::Chunk;
use voxels::{data::voxel_octree::VoxelMode};
use crate::components::player::Player;
use crate::graphics::chunk_preview::ChunkPreviewRender;
use crate::graphics::{ChunkPreviewGraphics, GraphicsResource};
use crate::input::hotbar::HotbarResource;
use crate::{data::GameResource, components::chunk_preview::ChunkPreview};

use super::chunks::{ChunkTexture, CustomMaterial, VOXEL_WEIGHT, VOXEL_TYPE_1};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(hook_to_player)
      .add_system(update)
      // .add_system(selected_voxel_changed)
      .add_system(spawn);
  }
}

fn hook_to_player(
  mut commands: Commands,
  mut players: Query<(Entity), Added<Player>>,
) {
  for entity in &players {
    commands
      .entity(entity)
      .insert(ChunkPreviewRender::default());
  }
}



/*
  When the selected keycode change
  Rerender the preview chunk
 */

fn update(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut game_res: ResMut<GameResource>,
  mut chunk_previews: Query<
    (Entity, &ChunkPreview, &mut ChunkPreviewRender), Changed<ChunkPreview>
  >,

  chunk_texture: Res<ChunkTexture>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,

  hotbar_res: Res<HotbarResource>,
  mut local_res: ResMut<LocalResource>,
) {
  let config = game_res.chunk_manager.config.clone();

  for (entity, chunk_preview, mut render) in &mut chunk_previews {
    for e in render.entities.iter() {
      commands.entity(*e).despawn_recursive();
    }
    render.entities.clear();

    for (key, chunk) in chunk_preview.chunks.iter() {
      local_res.last_chunk_op = Some(chunk.clone());
      local_res.chunk_op = Some(chunk.clone());
      // local_res.chunk_preview = chunk_preview.clone();
      local_res.preview_entity = entity;
    }
  }
}


fn selected_voxel_changed(
  mut local_res: ResMut<LocalResource>,

  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut game_res: ResMut<GameResource>,
  mut chunk_previews: Query<
    (Entity, &ChunkPreview, &mut ChunkPreviewRender), Changed<ChunkPreview>
  >,

  chunk_texture: Res<ChunkTexture>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,

  hotbar_res: Res<HotbarResource>,
) {
  if local_res.selected_keycode == hotbar_res.selected_keycode {
    return;
  }

  local_res.selected_keycode = hotbar_res.selected_keycode;
  local_res.chunk_op = local_res.last_chunk_op.clone();
}

fn spawn(
  mut local_res: ResMut<LocalResource>,
  graphics_res: Res<GraphicsResource>,
  hotbar_res: Res<HotbarResource>,

  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut game_res: ResMut<GameResource>,
  mut chunk_previews: Query<
    (&ChunkPreview, &mut ChunkPreviewRender)
  >,

  chunk_texture: Res<ChunkTexture>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
  if local_res.chunk_op.is_none() {
    return;
  }

  let (preview, mut render) = chunk_previews.get_mut(local_res.preview_entity).unwrap();
  for e in render.entities.iter() {
    commands.entity(*e).despawn_recursive();
  }
  render.entities.clear();

  let chunk = local_res.chunk_op.take().unwrap();
  let data = chunk.octree.compute_mesh(
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

    // let bar_op = hotbar_res
    //   .bars
    //   .iter()
    //   .find(|bar| bar.key_code == hotbar_res.selected_keycode);

    // let mut voxel = 0;
    // if bar_op.is_some() {
    //   voxel = bar_op.unwrap().voxel as u32 - 1;
    // }

    // let mut voxels = Vec::<[u32; 4]>::new();
    // for _ in 0..data.types_1.len() {
    //   voxels.push([voxel; 4]);
    // }
    // render_mesh.insert_attribute(VOXEL_TYPE_1, voxels);

    let mesh_handle = meshes.add(render_mesh);
    let material_handle = custom_materials.add(CustomMaterial {
      base_color: Color::rgb(1.0, 1.0, 1.0),
      albedo: chunk_texture.albedo.clone(),
      normal: chunk_texture.normal.clone(),
    });

    let chunk_size = (chunk.octree.get_size() / 2) as f32;
    let p = &preview.new;
    let adj = [p[0] as f32, p[1] as f32, p[2] as f32];
    let coord_f32 = [adj[0] - chunk_size, adj[1] - chunk_size, adj[2] - chunk_size];

    let mut visibility = Visibility::Visible;
    if !graphics_res.show_preview {
      visibility = Visibility::Hidden;
    }

    let entity = commands
      .spawn(MaterialMeshBundle {
        visibility: visibility,
        mesh: mesh_handle,
        material: material_handle,
        transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
          // .with_scale(Vec3::new(0.99, 0.999, 0.99 )),
        ..default()
      })
      .insert(ChunkPreviewGraphics { })
      .id();

    render.entities.push(entity);
  }

}


#[derive(Resource)]
struct LocalResource {
  last_chunk_op: Option<Chunk>,
  chunk_op: Option<Chunk>,
  selected_keycode: KeyCode,
  // chunk_preview: ChunkPreview,
  preview_entity: Entity,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      last_chunk_op: None,
      chunk_op: None,
      selected_keycode: KeyCode::Key1,
      // chunk_preview: ChunkPreview::default()
      preview_entity: Entity::PLACEHOLDER,
    }
  }
}

