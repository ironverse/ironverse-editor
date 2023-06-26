use bevy::{prelude::*, window::PresentMode};
use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "chunk")] {
    mod components;
    mod data;
    mod physics;
  } else {}
}

cfg_if! {
  if #[cfg(feature = "chunk_graphics")] {
    mod graphics;
  }
}

cfg_if! {
  if #[cfg(feature = "tests")] {
    mod tests;
    use bevy_flycam::NoCameraPlayerPlugin;
  }
}

mod input;
mod native;



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
    .add_plugin(data::CustomPlugin)
    .add_plugin(physics::CustomPlugin)
    .add_plugin(input::CustomPlugin)

    .add_plugin(native::CustomPlugin)
    ;

  cfg_if! {
    if #[cfg(feature = "player")] {
      use bevy_flycam::NoCameraAndGrabPlugin;
      app
        .add_plugin(NoCameraAndGrabPlugin)
        .add_plugin(components::camera::CustomPlugin)
        .add_plugin(components::player::CustomPlugin);
    }
  }
  
  cfg_if! {
    if #[cfg(feature = "chunk")] {
      app
        .add_plugin(components::chunk::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(feature = "chunk_graphics")] {
      app
        .add_plugin(graphics::chunks::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(feature = "tests")] {
      app
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(tests::ChunkPlugin);
    }
  }
  
  app.run();
}




/* 
use bevy::{prelude::*, window::PresentMode};
use bevy_flycam::NoCameraAndGrabPlugin;

mod physics;
mod data;
mod states;
mod components;
mod graphics;
mod input;
mod utils;
mod debugger;
mod ui;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(feature = "minimalgraphics")]
mod minimalgraphics;

#[cfg(feature = "normalgraphics")]
mod normalgraphics;

/*
  Able to modularized the features


  Notes:
    Debugger text and Egui are connected
    Create a common plugins?
    Disabling graphics module makes compilation time between ~12s to ~6s
    Disabling Egui and ui compilation time between ~6s to 3s?
    Use config file when necessary
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
    .add_plugin(NoCameraAndGrabPlugin)
    .add_plugin(physics::CustomPlugin)
    .add_plugin(data::CustomPlugin)
    .add_plugin(states::CustomPlugin)
    .add_plugin(components::CustomPlugin)
    .add_plugin(input::CustomPlugin)
    .add_plugin(graphics::CustomPlugin)
    // .add_plugin(debugger::CustomPlugin)
    ;

  #[cfg(feature = "gui_none")]
  app
    .add_plugin(ui::NonePlugin);

  #[cfg(feature = "gui_normal")]
  app
    .add_plugin(ui::CustomPlugin);


  #[cfg(feature = "minimalgraphics")]
  app
    .add_plugin(minimalgraphics::CustomPlugin);
  

  #[cfg(feature = "normalgraphics")]
  app
    .add_plugin(normalgraphics::CustomPlugin);

  
  #[cfg(target_arch = "wasm32")]
  app
    .add_plugin(wasm::CustomPlugin);

  #[cfg(not(target_arch = "wasm32"))]
  app
    .add_plugin(native::CustomPlugin);

  app.run();
}

// Comment out to make rust-analyzer work when compiling on linux/native
#[cfg(not(target_arch = "wasm32"))]
mod native;
 */



/* 
cfg_if::cfg_if! {
  if #[cfg(feature = "tests")] {
    mod tests;

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
        }));
      
      cfg_if::cfg_if! {
        if #[cfg(feature = "tests")] {
          app
            .add_plugin(tests::ChunkPlugin);
        }
      }
      
      app.run();
    }
  }
}
 */
