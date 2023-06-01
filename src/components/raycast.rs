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
  mut query: Query<(&Transform, &mut Raycast), With<FlyCam>>,
) {
  for (trans, mut raycast) in query.iter_mut() {
    let look_at = trans.forward();
    // info!("{:?}", look_at);

    let start_pos = trans.translation + raycast.adj;
    let dir = look_at.clone();
    let ray = Ray::new(
      Point3::new(start_pos.x, start_pos.y, start_pos.z), 
      Vector::new(dir.x, dir.y, dir.z)
    );
    let max_toi = f32::MAX;
    let solid = true;
    let filter = QueryFilter::only_fixed();

    
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
      
      raycast.point = Vec3::new(hit_point[0], hit_point[1], hit_point[2]);
      info!("hit {:?}", hit_point);
    }

    // if hit_point_op.is_none() {
    //   continue;
    // }

    // let range = 12.0;
    // let max_range_squared = range * range;
    // let mut target_diff_squared = f32::MAX;

    // let hit_point = hit_point_op.unwrap();
    // let target = Vec3::new(hit_point[0], hit_point[1], hit_point[2]);
    // let target_diff = start_pos - target;
    // target_diff_squared = target_diff.length_squared();


    // let nearest_op = nearest_voxel_point_0(
    //   &game_res.chunk_manager, 
    //   hit_point, 
    //   true
    // );

    // if target_diff_squared < max_range_squared && nearest_op.is_some() {
    //   let pos_i64 = nearest_op.unwrap();
    //   // raycast.target_voxel_op = Some(pos_i64);
    // }
  }
}


#[derive(Component)]
pub struct Raycast {
  pub point: Vec3,
  pub adj: Vec3,
}

impl Default for Raycast {
  fn default() -> Self {
    Self {
      point: Vec3::new(f32::NAN, f32::NAN, f32::NAN),
      adj: Vec3::new(0.0, 0.2, 0.0),
    }
  }
}