use bevy::prelude::*;
use crate::{physics::Physics, entities::GameResource};
use super::{GameState, GameEvent, GameEventType};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(enter.in_schedule(OnEnter(GameState::Start)))
      ;
  }
}

fn enter(
  mut physics: ResMut<Physics>,
  mut game_res: ResMut<GameResource>,
  mut game_events: EventWriter<GameEvent>,
) {
  let pos = Vec3::new(0.0, 1.0, 0.0);
  let (body, collider) = physics.spawn_character(1.0, 0.5, pos);
  game_events.send(GameEvent::new(
    GameEventType::SpawnPlayer, 
    pos,
  ));
}
