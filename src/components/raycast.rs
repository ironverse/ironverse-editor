use bevy::prelude::*;
use bevy_flycam::FlyCam;
use rapier3d::{prelude::{Vector, QueryFilter, Ray}, na::Point3};
use crate::{utils::{Math, nearest_voxel_point_0}, physics::Physics, data::GameResource};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update)
      ;
  }
}

fn update(
  physics: Res<Physics>,
  game_res: Res<GameResource>,
  mut query: Query<&mut Transform, With<FlyCam>>,
) {
  for mut transform in query.iter_mut() {
    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    let look_at = Math::rot_to_look_at(Vec3::new(pitch, yaw, 0.0));
    // info!("{:?}", look_at);

    let adj = Vec3::new(0.0, 1.5, 0.0);
    let start_pos = transform.translation + adj;
    let dir = look_at.clone();
    let ray = Ray::new(Point3::new(start_pos.x, start_pos.y, start_pos.z), Vector::new(dir.x, dir.y, dir.z));
    let max_toi = f32::MAX;
    let solid = true;
    let filter = QueryFilter::only_fixed();

    let range = 12.0;
    let max_range_squared = range * range;
    let mut target_diff_squared = f32::MAX;
    let mut hit_point_op = None;
    if let Some((_handle, toi)) = physics.query_pipeline.cast_ray(
      &physics.rigid_body_set, 
      &physics.collider_set, 
      &ray, 
      max_toi, 
      solid, 
      filter
    ) {
      let hit_point = ray.point_at(toi);
      hit_point_op = Some(hit_point.clone());
      let nearest_op = nearest_voxel_point_0(
        &game_res.chunk_manager, 
        hit_point, 
        true
      );

      info!("hit {:?}", hit_point);
      
      // let target = Vec3::new(hit_point[0], hit_point[1], hit_point[2]);
      // let target_diff = start_pos - target;
      // target_diff_squared = target_diff.length_squared();
      // if target_diff_squared < max_range_squared && nearest_op.is_some() {
      //   let pos_i64 = nearest_op.unwrap();
      //   raycast.target_voxel_op = Some(pos_i64);
      // }
    }



  }
}
