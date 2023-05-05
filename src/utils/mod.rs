#![allow(dead_code, unused_variables)]  // Forced to use, even though the look_at_to_rotation_quat() is being used, it is still showing warning

use bevy::math::{Vec3, Quat};
use rapier3d::na::Point3;
use voxels::chunk::chunk_manager::ChunkManager;
use voxels::chunk::{adjacent_keys_i64};
use voxels::{chunk::{voxel_pos_to_key}};

pub struct Math;

impl Math {
  pub fn look_at_to_rotation_quat(look_at: Vec3) -> Quat {
    let rot = Math::look_at_to_rotation(look_at);
    // Quat::from_rotation_ypr(rot.y, rot.x, 0.0)
    Quat::from_rotation_y(rot.y) * Quat::from_rotation_x(rot.x)
  }

  pub fn look_at_to_rotation(look_at: Vec3) -> Vec3 {
    let tmp_look_at = look_at.normalize();
    let mut rad_x = tmp_look_at.y;
    if rad_x.is_nan() {
      rad_x = 0.0;
    }

    let mut rad_y = tmp_look_at.x / tmp_look_at.z;
    if rad_y.is_nan() {
      rad_y = 0.0;
    }

    
    let mut y_rot = rad_y.atan();
    if tmp_look_at.z > 0.0 {
      let half_pi = std::f32::consts::PI * 0.5;
      y_rot = -((half_pi) + (half_pi - y_rot));
    }

    Vec3::new(rad_x.asin(), y_rot, 0.0)
  }

  pub fn rot_to_look_at(rot: Vec3) -> Vec3 {
    let yaw = -rot.y - std::f32::consts::PI * 0.5;

    let len = rot.x.cos();
    return Vec3::new(yaw.cos() * len, rot.x.sin(), yaw.sin() * len).normalize();
  }
}

pub fn to_key(translation: &Vec3, seamless_size: u32) -> [i64; 3] {
  let pos = [translation.x as i64, translation.y as i64, translation.z as i64];
  voxel_pos_to_key(&pos, seamless_size)
}

// pub fn create_collider_mesh(
//   octree: &VoxelOctree, 
//   voxel_reuse: &mut VoxelReuse
// ) -> MeshColliderData {
//   let mesh = get_surface_nets(octree, voxel_reuse);

//   let mut positions = Vec::new();
//   let mut indices = Vec::new();
  
//   for pos in mesh.positions.iter() {
//     // positions.push(Point::new(pos[0], pos[1], pos[2]));
//     positions.push(Vec3::new(pos[0], pos[1], pos[2]));
//   }
  
//   for ind in mesh.indices.chunks(3) {
//     // println!("i {:?}", ind);
//     indices.push([ind[0], ind[1], ind[2]]);
//   }


//   MeshColliderData {
//     positions: positions,
//     indices: indices,
//   }
// }

#[derive(Clone)]
pub struct MeshColliderData {
  // pub positions: Vec<Point<f32>>,
  pub positions: Vec<Vec3>,
  pub indices: Vec<[u32; 3]>,
}






// pub fn create_mesh(
//   meshes: &mut ResMut<Assets<Mesh>>,
//   positions: Vec<[f32; 3]>,
//   normals: Vec<[f32; 3]>,
//   uvs: Vec<[f32; 2]>,
//   indices: Vec<u32>,
// ) -> Handle<Mesh> {
//   let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
//   mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
//   mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
//   mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
//   mesh.set_indices(Some(Indices::U32(indices)));
//   meshes.add(mesh)
// }

// pub fn get_raycasts(
//   origin: &[f32; 3],
//   look_at: &Vec3,
//   range: i64, 
//   seamless_size: u32
// ) -> Vec<[i64; 3]> {
//   let dist = (range + 1) * seamless_size as i64;
//   let look_at_vec = look_at.normalize();
//   let divisions = range * 30;
//   let div_dist = dist as f32 / divisions as f32;

//   let mut prev_ray = [i64::MIN, i64::MIN, i64::MIN];
//   let mut rays = Vec::new();
//   for div in 1..divisions + 1 {
//     let interpolated = div_dist * div as f32;
//     let mut div_x = origin[0] + (look_at_vec.x * interpolated);
//     let mut div_y = origin[1] + (look_at_vec.y * interpolated);
//     let mut div_z = origin[2] + (look_at_vec.z * interpolated);

//     div_x = div_x.round();
//     div_y = div_y.round();
//     div_z = div_z.round();
//     let ray = [div_x as i64, div_y as i64, div_z as i64];

//     // if !in_range_by_chunk(origin, ray, dist) {
//     //   continue;
//     // }
    
//     if same_coord_i64(&ray, &prev_ray) {
//       continue;
//     }

//     prev_ray = ray.clone();
//     rays.push(ray);
//   }
//   rays
// }


// pub fn get_raycast_keys(
//   origin: &[f32; 3],
//   look_at: &Vec3,
//   range: i64, 
//   seamless_size: u32
// ) -> Vec<([i64; 3], [i64; 3])> {
//   let pos_key = &posf32_to_world_key(origin, seamless_size);

//   let dist = ((range + 1) * seamless_size as i64) as f32;
//   let look_at_vec = look_at.normalize();
//   let divisions = range * 30;
//   let div_dist = dist / divisions as f32;

//   let mut prev_ray_key = [i64::MIN, i64::MIN, i64::MIN];
//   let mut current_keys = Vec::new();
//   for div in 1..divisions + 1 {
//     let interpolated = div_dist * div as f32;
//     let mut div_x = origin[0] + (look_at_vec.x * interpolated);
//     let mut div_y = origin[1] + (look_at_vec.y * interpolated);
//     let mut div_z = origin[2] + (look_at_vec.z * interpolated);

//     div_x = div_x.round();
//     div_y = div_y.round();
//     div_z = div_z.round();
//     let div_pos = [div_x as i64, div_y as i64, div_z as i64];
//     let ray_key = &voxel_pos_to_key(&div_pos, seamless_size);

//     if !in_range_by_chunk(pos_key, ray_key, range) {
//       continue;
//     }
//     if same_coord_i64(ray_key, &prev_ray_key) {
//       continue;
//     }

//     prev_ray_key = ray_key.clone();
//     current_keys.push((ray_key.clone(), div_pos));
//   }
//   current_keys
// }

pub fn nearest_voxel_point(
  chunk_manager: &ChunkManager,
  intersection: Point3<f32>,
  _include_current: bool,
  voxel: u8,
) -> Option<[i64; 3]> {
  let point = [
    (intersection[0].round()) as i64,
    (intersection[1].round()) as i64,
    (intersection[2].round()) as i64,
  ];

  let mut shortest_dist = f32::MAX;

  let mut nearest = None;
  let points_around = adjacent_keys_i64(&point, 1, true);
  for pa in points_around.iter() {
    let val = chunk_manager.get_voxel(pa);
    if val == voxel {
      let tmp_intersect = Vec3::new(intersection[0], intersection[1], intersection[2]);
      let current_point = Vec3::new(pa[0] as f32, pa[1] as f32, pa[2] as f32);

      let dist = tmp_intersect - current_point;
      // println!("tmp2 {:?} {:?}", (tmp_point[0], tmp_point[1], tmp_point[2]), dist);
      if shortest_dist > dist.length_squared() {
        shortest_dist = dist.length_squared();
        nearest = Some(pa.clone());
      }
    }
  }
  return nearest;
}

pub fn nearest_voxel_point_0(
  chunk_manager: &ChunkManager,
  intersection: Vec3,
  _include_current: bool,
) -> Option<[i64; 3]> {
  let point = [
    (intersection.x.round()) as i64,
    (intersection.y.round()) as i64,
    (intersection.z.round()) as i64,
  ];

  let mut shortest_dist = f32::MAX;
  let mut nearest = None;
  let points_around = adjacent_keys_i64(&point, 1, true);
  for pa in points_around.iter() {
    let val = chunk_manager.get_voxel(pa);
    if val > 0 {
      let current_point = Vec3::new(pa[0] as f32, pa[1] as f32, pa[2] as f32);

      let dist = intersection - current_point;
      // println!("tmp2 {:?} {:?}", (tmp_point[0], tmp_point[1], tmp_point[2]), dist);
      if shortest_dist > dist.length_squared() {
        shortest_dist = dist.length_squared();
        nearest = Some(pa.clone());
      }
    }
  }
  return nearest;
}