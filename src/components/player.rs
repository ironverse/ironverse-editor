use bevy::prelude::*;
use rapier3d::{prelude::{ColliderHandle, RigidBodyHandle}, na::Vector3};
use voxels::utils::posf32_to_world_key;
use crate::{states::GameState, physics::Physics, data::GameResource};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(
        enter.in_schedule(OnEnter(GameState::Play))
      )
      .add_system(
        update.in_set(OnUpdate(GameState::Play))
      );
  }
}


fn enter(
  mut commands: Commands,
  mut physics: ResMut<Physics>,
  mut game_res: ResMut<GameResource>,
) {

  let pos = game_res.data.player.position;
  let (body, collider) = physics.spawn_character(1.0, 0.5, Vec3::new(pos[0], pos[1], pos[2]));

  let k = posf32_to_world_key(&pos, game_res.chunk_manager.config.seamless_size);
  commands.spawn((Player::new(body, collider, k)));
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
