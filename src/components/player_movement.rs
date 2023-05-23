use bevy::prelude::*;
use rapier3d::na::Vector3;
use voxels::utils::posf32_to_world_key;
use crate::{physics::Physics, data::{GameResource, Player}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update);
  }
}

fn update(
  mut query: Query<(&mut Transform, &mut Player), With<PlayerMovement>>,
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


#[derive(Component)]
pub struct PlayerMovement {}