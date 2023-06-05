use bevy::prelude::*;
use bevy_flycam::FlyCam;
use voxels::utils::posf32_to_world_key;
use crate::{physics::Physics, data::{GameResource, GameState, Player}, components::{player_movement::PlayerMovement, chunks::Chunks, raycast::Raycast, chunk_edit::ChunkEdit}, debugger::raycast::RaycastDebugger};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(
        enter.in_schedule(OnEnter(GameState::Start))
      )
      // .add_system(update)
      ;
  }
}

fn enter(
  mut commands: Commands,
  mut physics: ResMut<Physics>,
  mut game_res: ResMut<GameResource>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  let pos = [0.0, 5.0, -10.0];

  info!("enter");

  let (body, collider) = physics.spawn_character(1.0, 0.5, Vec3::new(pos[0], pos[1], pos[2]));
  let k = posf32_to_world_key(&pos, game_res.chunk_manager.config.seamless_size);
  commands.spawn((
    Player::new(body, collider, k),
    PlayerMovement { },
    Chunks::default(),
    ChunkEdit::default(),
    Raycast::default(),
    FlyCam {},
    Camera3dBundle {
      transform: Transform::from_xyz(pos[0], pos[1], pos[2])
        .looking_at(Vec3::ZERO, Vec3::Y),
      ..Default::default()
    },
    RaycastDebugger::default(),
  ));
}

fn update(
  mut light_query: Query<&mut Transform, With<PointLight>>,
  time: Res<Time>,
) {
  let t = time.elapsed_seconds();
  for mut tfm in light_query.iter_mut() {
    tfm.translation = 5.0 * Vec3::new(t.cos(), 1.0, t.sin());
  }
}

