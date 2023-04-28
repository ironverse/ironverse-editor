use bevy_flycam::prelude::*;
use bevy::{prelude::*, render::{mesh::{MeshVertexAttribute, MeshVertexBufferLayout, Indices}, render_resource::{VertexFormat, AsBindGroup, ShaderRef, RawRenderPipelineDescriptor, SpecializedMeshPipelineError, RenderPipelineDescriptor, PrimitiveTopology}}, reflect::TypeUuid, pbr::{MaterialPipeline, MaterialPipelineKey}, asset::LoadState, window::PresentMode};
use voxels::{chunk::{adjacent_keys, chunk_manager::ChunkManager}, utils::key_to_world_coord_f32, data::voxel_octree::VoxelMode};

mod terrain;
mod physics;
mod graphics;
mod utils;

#[cfg(target_arch = "wasm32")]
mod wasm;

fn main() {
  let mut app = App::new();
  app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Ironverse Editor".into(),
        resolution: (800., 600.).into(),
        present_mode: PresentMode::AutoVsync,
        fit_canvas_to_parent: true,
        prevent_default_event_handling: false,
        ..default()
      }),
      ..default()
    }))
    .add_plugin(PlayerPlugin)
    .add_plugin(terrain::CustomPlugin)
    .add_plugin(physics::CustomPlugin)
    .add_plugin(graphics::CustomPlugin);
  
  #[cfg(target_arch = "wasm32")]
  app
    .add_plugin(wasm::CustomPlugin);

  app.run();

}