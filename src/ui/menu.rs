use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, Color32, Frame, Vec2, Button}, EguiContexts};
use bevy_egui::egui::Rect;
use flume::{Sender, Receiver};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use crate::data::CursorState;

use super::{UIResource, UIState};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(UIMenuResource::default());

    app
      .add_system(toggle_show)
      .add_system(render.in_set(OnUpdate(UIState::Menu)));
  }
}


fn toggle_show(
  key_input: Res<Input<KeyCode>>,
  state: Res<State<UIState>>,
  mut next_state: ResMut<NextState<UIState>>,
  cursor_state: Res<State<CursorState>>,
) {
  // if key_input.just_pressed(KeyCode::Escape) {
  //   match state.0 {
  //     UIState::Default => { next_state.set(UIState::Menu); },
  //     UIState::Menu => { next_state.set(UIState::Default); },
  //     _ => { next_state.set(UIState::Default); },
  //   }
  //   info!("Toggle show menu {:?}", state.0);
  // }

  // match cursor_state.0 {
  //   CursorState::None => { next_state.set(UIState::Menu); },
  //   CursorState::Locked => { next_state.set(UIState::Default); },
  //   _ => {}
  // };
}

fn render(
  mut commands: Commands,
  mut contexts: EguiContexts,
  windows: Query<(Entity, &Window), With<PrimaryWindow>>,
  mut ui_res: ResMut<UIResource>,
  state: Res<State<UIState>>,
  mut next_state: ResMut<NextState<UIState>>,
  // mut next_game_state: ResMut<NextState<GameState>>,
  local_res: Res<UIMenuResource>,
) {
  let res = windows.get_single();
  if res.is_err() {
    return;
  }
  let (entity, window) = res.unwrap();
  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 255),
    ..Default::default()
  };

  let size = [200.0, 300.0];
  let x = (window.width() * 0.5) - size[0] * 0.5;
  let y = window.height() * 0.1;
  let button_size = Vec2::new(125.0, 50.0);

  egui::Window::new("menu")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(Rect {
      min: [x, y].into(),
      max: [x + size[0], y + size[1]].into(),
    })
    .show(contexts.ctx_mut(), |ui| {
      ui.set_min_size(size.into());

      ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        let back_to_game = Button::new("Back to Game")
          .min_size(button_size);
        if ui.add(back_to_game).clicked() {
          info!("Back to game");
          next_state.set(UIState::Default);
        }

        ui.add_space(20.0);
        let new = Button::new("New")
          .min_size(button_size);
        if ui.add(new).clicked() {
          next_state.set(UIState::New);
        }

        ui.add_space(20.0);
        let load = Button::new("Load")
          .min_size(button_size);
        if ui.add(load).clicked() {
          // if let Some(path) = rfd::FileDialog::new().pick_file() {
          //   ui_res.load_file_path = path.to_str().unwrap().to_string();
          //   ui_res.load_file_init = false;
          //   next_state.set(UIState::Load);
          // }

          // load_file(local_res.send.clone());
          // next_state.set(UIState::Load);
          // next_game_state.set(GameState::Load);
        }

        ui.add_space(20.0);
        let save = Button::new("Save")
          .min_size(button_size);
        if ui.add(save).clicked() {
          // if let Some(path) = rfd::FileDialog::new().save_file() {
          //   ui_res.load_file_path = path.to_str().unwrap().to_string();
          //   next_state.set(UIState::Save);
          // }

          next_state.set(UIState::Save);
        }

        ui.add_space(20.0);
        let quit = Button::new("Quit")
          .min_size(button_size);
        if ui.add(quit).clicked() {
          commands.entity(entity).despawn();
        }
      });
    });

}


fn recv_file(local_res: Res<UIMenuResource>,) {
  for file in local_res.recv.drain() {
    let config = config::standard();
    
    
    // let res = bincode::decode_from_slice::<Data, Configuration>(&file[..], config);
    // match res {
    //   Ok(r) => { info!("ok"); },
    //   Err(e) => info!("{:?}", e),
    // }

    // let en1 = array_bytes::hex2bytes(str).unwrap();
    // let (data, len): (Data, usize) = bincode::decode_from_slice(&en1[..], config).unwrap();

    // info!("data {:?}", data);
    let mut str = String::from_utf8(file).unwrap();
    str = str.replace('"', "");
    info!("str {:?}", str);
    let vec = array_bytes::hex2bytes(str).unwrap();

    let res = bincode::decode_from_slice::<Data, Configuration>(&vec[..], config);
    let (data, usize) = match res {
      Ok(r) => { 
        info!("ok");
        r
      },
      Err(e) => {
        info!("{:?}", e);
        return;
      }
    };

    info!("data {:?}", data);
  }
}




#[cfg(target_arch = "wasm32")]
fn load_file(send: Sender<Vec<u8>>) {
  let task = rfd::AsyncFileDialog::new().pick_file();

  let send = send.clone();
  // Await somewhere else
  execute(async move {
    let file = task.await;

    if let Some(file) = file {
      // If you are on native platform you can just get the path
      #[cfg(not(target_arch = "wasm32"))]
      println!("{:?}", file.path());

      // If you care about wasm support you just read() the file
      let res = file.read().await;
      info!("Send {}", res.len());
      send.send(res);
    }
  });
}

use std::future::Future;

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
  wasm_bindgen_futures::spawn_local(f);
}


#[derive(Resource)]
pub struct UIMenuResource {
  send: Sender<Vec<u8>>,
  pub recv: Receiver<Vec<u8>>,
}

impl Default for UIMenuResource {
  fn default() -> Self {
    let (send, recv) = flume::bounded(1);
    Self {
      send: send,
      recv: recv,
    }
  }
}


use serde::{Deserialize, Serialize};
use bincode::{config::{self, Configuration}, Decode, Encode};


#[derive(Serialize, Deserialize, Encode, Decode, PartialEq, Debug)]
pub struct Data {
  pub player: Player,
  pub terrains: Terrains,
}

#[derive(Serialize, Deserialize, Encode, Decode, PartialEq, Debug)]
pub struct Player {
  pub position: [f32; 3]
}

#[derive(Serialize, Deserialize, Encode, Decode, PartialEq, Debug)]
pub struct Terrains {
  pub keys: Vec<[i64; 3]>,
  pub voxels: Vec<String>,
}