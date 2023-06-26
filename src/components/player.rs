use bevy::prelude::*;
use rapier3d::{na::Vector3, prelude::{RigidBodyHandle, ColliderHandle}};
use voxels::utils::posf32_to_world_key;
use crate::{physics::Physics, data::{GameResource}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update);
  }
}

fn update(
  mut query: Query<(&mut Transform, &mut Player)>,
  mut physics: ResMut<Physics>,
  mut game_res: ResMut<GameResource>,
) {
  for (mut trans, mut player) in &mut query {
    let p = trans.translation;
    let rigid_body = &mut physics.rigid_body_set[player.body];
    rigid_body.set_position(Vector3::new(p.x, p.y, p.z).into(), false);

    let k = posf32_to_world_key(&[p.x, p.y, p.z], game_res.chunk_manager.config.seamless_size);
    if player.key != k {
      player.prev_key = player.key.clone();
      player.key = k;
    }
  }
}

#[derive(Component, Debug, Clone)]
pub struct Player {
  pub body: RigidBodyHandle,
  pub collider: ColliderHandle,
  pub prev_key: [i64; 3],
  pub key: [i64; 3],
}

impl Player {
  pub fn new(b: RigidBodyHandle, c: ColliderHandle, k: [i64; 3]) -> Self {
    
    Self {
      body: b,
      collider: c,
      prev_key: k.clone(),
      key: k
    }
  }
}