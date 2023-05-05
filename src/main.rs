use bevy_flycam::prelude::*;
use bevy::{prelude::*, window::PresentMode};
mod terrain;
mod physics;
mod graphics;
mod utils;
mod states;
mod data;
mod components;
// mod ui;
// mod input;
mod debugger;

#[cfg(target_arch = "wasm32")]
mod wasm;

fn main() {
  let mut app = App::new();
  app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Ironverse Editor".into(),
        resolution: (800., 600.).into(),
        present_mode: PresentMode::AutoVsync,
        fit_canvas_to_parent: true,
        prevent_default_event_handling: false,
        ..default()
      }),
      ..default()
    }))
    .configure_set(GameSet::PreUpdate.before(CoreSet::Update))
    .configure_set(GameSet::PostUpdate.after(CoreSet::Update))
    // .add_plugin(PlayerPlugin)
    .add_plugin(NoCameraPlayerPlugin)
    .add_plugin(terrain::CustomPlugin)
    .add_plugin(physics::CustomPlugin)
    .add_plugin(states::CustomPlugin)
    .add_plugin(data::CustomPlugin)
    .add_plugin(components::CustomPlugin)
    .add_plugin(graphics::CustomPlugin)
    // .add_plugin(ui::CustomPlugin)
    // .add_plugin(input::CustomPlugin)
    .add_plugin(debugger::CustomPlugin)
    ;
  
  #[cfg(target_arch = "wasm32")]
  app
    .add_plugin(wasm::CustomPlugin);

  app.run();

}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
enum GameSet {
  PreUpdate,
  PostUpdate,
}