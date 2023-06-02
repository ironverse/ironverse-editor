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

// Comment out to make rust-analyzer work when compiling on linux/native
#[cfg(not(target_arch = "wasm32"))] 
mod native;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(feature = "minimal")]
mod minimalgraphics;

#[cfg(feature = "normal")]
mod defaultgraphics;

/*
  Able to modularized the features
    To make it faster to iterate
    We only need the test repo to show the trimmed down code
      For other developers to examine the code
      Otherwise the plugin here should modularized the features

    Current feature
      Center the origin of the mesh
        Features to isolate and enable:
          Physics
            Raycast
          Chunk Creation
            Data chunk
            Graphics chunk

    Notes:
      Debugger text and Egui are connected
      Create a common plugins?
      Disabling graphics module makes compilation time between ~12s to ~6s
      Disabling Egui and ui compilation time between ~6s to 3s?

 */

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

    .add_plugin(input::CustomPlugin)
    .add_plugin(ui::CustomPlugin)
    .add_plugin(graphics::CustomPlugin)
    // .add_plugin(debugger::CustomPlugin)
    ;

  #[cfg(feature = "minimal")]
  app
    .add_plugin(NoCameraAndGrabPlugin)
    .add_plugin(physics::CustomPlugin)
    .add_plugin(data::CustomPlugin)
    .add_plugin(states::CustomPlugin)
    .add_plugin(components::CustomPlugin)
    .add_plugin(minimalgraphics::CustomPlugin);

  #[cfg(feature = "normal")]
  app
    .add_plugin(defaultgraphics::CustomPlugin);


  
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