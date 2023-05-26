use bevy_flycam::prelude::*;
use bevy::{prelude::*, window::PresentMode};
use bevy_framepace::{FramepacePlugin, Limiter, FramepaceSettings};

mod physics;
mod data;
mod states;
mod components;
mod graphics;
mod input;
mod utils;
mod debugger;
mod ui;

#[cfg(not(target_arch = "wasm32"))] // rust-analyzer won't work
mod native;

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
    .add_plugin(NoCameraAndGrabPlugin)
    .add_plugin(physics::CustomPlugin)
    .add_plugin(data::CustomPlugin)
    .add_plugin(states::CustomPlugin)
    .add_plugin(components::CustomPlugin)
    .add_plugin(graphics::CustomPlugin)
    .add_plugin(input::CustomPlugin)
    .add_plugin(ui::CustomPlugin)

    // .add_plugin(debugger::CustomPlugin)
    // .add_startup_system(startup)
    ;
  
  #[cfg(not(target_arch = "wasm32"))]
  app
    .add_plugin(native::CustomPlugin);
  
  #[cfg(target_arch = "wasm32")]
  app
    .add_plugin(wasm::CustomPlugin);

  


  app.run();

}

// fn startup(mut frame_settings: ResMut<FramepaceSettings>) {
//   // Not working on wasm?
//   frame_settings.limiter = Limiter::from_framerate(30.0);
// }

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
enum GameSet {
  PreUpdate,
  PostUpdate,
}