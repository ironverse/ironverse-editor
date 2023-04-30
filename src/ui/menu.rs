use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, Color32, Frame, Vec2, Button}, EguiContexts};
use bevy_egui::egui::Rect;
use bevy_flycam::{MovementSettings, WasmResource};
use flume::{Sender, Receiver};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use crate::wasm::html_body;

use super::{UIResource, UIState};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(enter.in_schedule(OnEnter(UIState::Menu)))
      .add_system(exit.in_schedule(OnExit(UIState::Menu)))
      .add_system(render.in_set(OnUpdate(UIState::Menu)))
      .add_system(recv_file.in_set(OnUpdate(UIState::Menu)))
      ;

      // app
      //   .add_system(test_download_file.in_schedule(OnEnter(UIState::Menu)));
      app
        .add_startup_system(test_download_file);
  }
}

fn test_download_file() {
  let body = html_body();
  
  let res = body.query_selector("#download");
  
  let a_ops = match res {
    Ok(ops) => ops,
    Err(e) => { 
      info!("{:?}", e);
      return ()
    }
  };


  if a_ops.is_some() {
    let data = Data {
      player: Player {
        position: [0.0, 1.0, 0.0],
      },
      terrains: Terrains { keys: vec![[0, 0, 0]], voxels: vec!["Test".to_string()] }
    };

    let config = config::standard();
    // let encoded: Vec<u8> = bincode::encode_to_vec(&data, config).unwrap();
    // let str = array_bytes::bytes2hex("", encoded);

    // let str = toml::to_string_pretty(&data).unwrap();
    // let t = toml::to_string(&data).unwrap();
    let encoded: Vec<u8> = bincode::encode_to_vec(&data, config).unwrap();
    let str = array_bytes::bytes2hex("", encoded);
    
    info!("a: {:?}", str);
    let a = a_ops.unwrap();
    a.set_attribute("download", "save.toml");
    a.set_attribute("href", format!("data:,{:?}", str).as_str());
    // a.set_attribute("innerHTML", "download");

    let a1: HtmlElement = a.dyn_into::<HtmlElement>().unwrap();
    a1.click();

  }




  // let opt = body.child_nodes().get(0);
  // if opt.is_some() {
  //   let o = opt.unwrap();
  //   o.
  // }
}



fn enter(
  mut move_setting_res: ResMut<MovementSettings>,
  #[cfg(target_arch = "wasm32")]
  mut wasm_res: ResMut<WasmResource>,
) {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  document.exit_pointer_lock();
  move_setting_res.sensitivity = 0.0;

  wasm_res.pointer_lock_enabled = false;
}

fn exit(
  mut move_setting_res: ResMut<MovementSettings>,
  #[cfg(target_arch = "wasm32")]
  mut wasm_res: ResMut<WasmResource>,
) {
  move_setting_res.sensitivity = 0.00012;
  wasm_res.pointer_lock_enabled = true;
  html_body().request_pointer_lock();
}

fn render(
  mut commands: Commands,
  mut contexts: EguiContexts,
  windows: Query<(Entity, &Window), With<PrimaryWindow>>,
  mut ui_res: ResMut<UIResource>,
  state: Res<State<UIState>>,
  mut next_state: ResMut<NextState<UIState>>,
  local_res: Res<LocalResource>,
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

          load_file(local_res.send.clone());
        }

        ui.add_space(20.0);
        let save = Button::new("Save")
          .min_size(button_size);
        if ui.add(save).clicked() {
          // if let Some(path) = rfd::FileDialog::new().save_file() {
          //   ui_res.load_file_path = path.to_str().unwrap().to_string();
          //   next_state.set(UIState::Save);
          // }
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


fn recv_file(local_res: Res<LocalResource>,) {
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
struct LocalResource {
  send: Sender<Vec<u8>>,
  recv: Receiver<Vec<u8>>,
}

impl Default for LocalResource {
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