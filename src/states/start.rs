use bevy::prelude::*;
use crate::{physics::Physics, data::{GameResource}, GameSet};
use super::{GameState, GameEvent, GameEventType};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(
        enter.in_schedule(OnEnter(GameState::Start))
      )
      ;
  }
}

fn enter(
  mut physics: ResMut<Physics>,
  mut game_res: ResMut<GameResource>,
  mut next_state: ResMut<NextState<GameState>>,
  
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  next_state.set(GameState::Play);
}
