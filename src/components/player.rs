use bevy::prelude::*;
use rapier3d::prelude::{ColliderHandle, RigidBodyHandle};
use voxels::utils::posf32_to_world_key;

use crate::{states::GameState, physics::Physics, data::GameResource};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(
        enter.in_schedule(OnEnter(GameState::Start))
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

  let pos = [0.0, 5.0, 0.0];
  let (body, collider) = physics.spawn_character(1.0, 0.5, Vec3::new(pos[0], pos[1], pos[2]));

  let k = posf32_to_world_key(&pos, game_res.chunk_manager.config.seamless_size);
  commands.spawn((Player::new(body, collider, k)));
}

fn update() {

}





#[derive(Component, Debug, Clone)]
pub struct Player {
  pub body: RigidBodyHandle,
  pub collider: ColliderHandle,
  pub key: [i64; 3],
}

impl Player {
  pub fn new(b: RigidBodyHandle, c: ColliderHandle, k: [i64; 3]) -> Self {
    
    Self {
      body: b,
      collider: c,
      key: k
    }
  }
}
