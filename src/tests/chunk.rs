use bevy::prelude::*;
use voxels::data::voxel_octree::{VoxelOctree, ParentValueType};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(startup);
  }
}

fn startup() {
  let start = 0;
  let end = 16;

  let mut voxels = Vec::new();
  for x in start..end {
    for y in start..end {
      for z in start..end {
        voxels.push([x as u32, y as u32, z as u32, 1 as u32]);
      }
    }
  }


  let mut octree = VoxelOctree::new_from_3d_array(
    0, 
    4, 
    &voxels, ParentValueType::DefaultValue
  );
}




/*
  Data and rendering include here
  Just separate later

*/