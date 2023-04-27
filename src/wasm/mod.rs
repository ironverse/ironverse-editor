use bevy::prelude::*;
use web_sys::HtmlElement;
use flume::*;
use wasm_bindgen::prelude::*;
use web_sys::ErrorEvent;

/*
  TODO:
    Have to make it conditional compiling just for the wasm build
 */

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_startup_system(startup);
  }
}

fn startup(local_res: Res<LocalResource>,) {
  // let (req_sender, req_receiver) = tokio::sync::mpsc::unbounded_channel::<ResizeRequest>();
  // if resize_channel.sender.is_none() {
  //   resize_channel.sender = Some(Mutex::new(req_sender.clone()));
  //   resize_channel.receiver = Some(Mutex::new(req_receiver));
  // }

  // let canvas_res = html_body().query_selector("canvas");
  // if canvas_res.is_err() || canvas_res.unwrap().is_none() {
  //   panic!("Error detecting canvas");
  // }
  // let canvas = canvas_res.unwrap().unwrap();
  // canvas.

  // let _canvas = match html_body().get_elements_by_tag_name("canvas").item(0) {
  //   Some(c) => c,
  //   None => {
  //     info!("No canvas detected");
  //     return;
  //   }
  // };

  let send_fullscreen = local_res.send_fullscreen.clone();
  let window = web_sys::window().expect("no global `window` exists");
  let cb = Closure::wrap(Box::new(move |_e: ErrorEvent| {
    let _r = send_fullscreen.send(true);
  }) as Box<dyn FnMut(ErrorEvent)>);

  window.set_onresize(Some(cb.as_ref().unchecked_ref()));
  cb.forget();
}


pub fn html_body() -> HtmlElement {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  body
}


#[derive(Resource)]
struct LocalResource {
  send_fullscreen: Sender<bool>,
  recv_fullscreen: Receiver<bool>,
}

impl Default for LocalResource {
  fn default() -> Self {
    let send, recv = flume::bounded(1);
    Self {
      send_fullscreen: send,
      recv_fullscreen: recv,
    }
  }
}