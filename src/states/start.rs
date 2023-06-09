use bevy::prelude::*;
use bevy_flycam::FlyCam;
use voxels::utils::posf32_to_world_key;
use crate::{physics::Physics, data::{GameResource, GameState, Player}, components::{player_movement::PlayerMovement, chunks::Chunks, raycast::Raycast, chunk_edit::ChunkEdit}, graphics::chunks::ChunkGraphics, debugger::raycast::RaycastDebugger};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(
        enter.in_schedule(OnEnter(GameState::Start))
      )
      .add_system(update)
      ;
  }
}

fn enter(
  mut commands: Commands,
  mut physics: ResMut<Physics>,
  mut game_res: ResMut<GameResource>,
  mut next_state: ResMut<NextState<GameState>>,
) {

  // info!("Enter GameState::Start");

  // let data = Data::default();
  // game_res.data = data;

  // next_state.set(GameState::Play);

  let pos = [0.0, 5.0, 0.0];
  let (body, collider) = physics.spawn_character(1.0, 0.5, Vec3::new(pos[0], pos[1], pos[2]));

  let k = posf32_to_world_key(&pos, game_res.chunk_manager.config.seamless_size);
  commands.spawn((
    Player::new(body, collider, k),
    PlayerMovement { },
    Chunks::default(),
    ChunkGraphics::default(),
    Raycast::default(),
    Camera3dBundle {
      ..Default::default()
    },
    FlyCam {},
    RaycastDebugger::default(),
    ChunkEdit::default(),
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

