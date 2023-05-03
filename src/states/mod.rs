use bevy::prelude::*;

mod start;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<GameState>()
      .add_event::<GameEvent>()
      .add_plugin(start::CustomPlugin)
      ;
  }
}



#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
  #[default]
  Start,
  Play,
  Pause,
  End,
}


pub struct GameEvent {
  pub event_type: GameEventType,
  pub pos: Vec3,
}

impl GameEvent {
  pub fn new(e: GameEventType, p: Vec3) -> Self {
    Self {
      event_type: e,
      pos: p
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum GameEventType {
  SpawnPlayer
}



/*
  Spawn player
  Spawn terrains around

 */