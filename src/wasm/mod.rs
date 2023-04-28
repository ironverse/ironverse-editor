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
      .add_startup_system(startup)
      .add_system(update_fullscreen)
      .add_system(update_resize)
      ;
  }
}

fn startup(local_res: Res<LocalResource>,) {
  let send_resize = local_res.send_resize.clone();
  let window = web_sys::window().expect("no global `window` exists");
  let cb = Closure::wrap(Box::new(move |_e: ErrorEvent| {
    let _r = send_resize.send(true);
  }) as Box<dyn FnMut(ErrorEvent)>);

  window.set_onresize(Some(cb.as_ref().unchecked_ref()));
  cb.forget();
}

fn update_fullscreen(input: Res<Input<KeyCode>>,) {
  if input.just_pressed(KeyCode::F) {
    info!("Fullscreen");
    html_body().request_fullscreen();
  }
}

fn update_resize(
  local_res: Res<LocalResource>,
) {
  for _resize in local_res.recv_resize.drain() {
    info!("resize");
  }
}


pub fn html_body() -> HtmlElement {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  body
}


#[derive(Resource)]
struct LocalResource {
  send_resize: Sender<bool>,
  recv_resize: Receiver<bool>,
}

impl Default for LocalResource {
  fn default() -> Self {
    let (send, recv) = flume::bounded(1);
    Self {
      send_resize: send,
      recv_resize: recv,
    }
  }
}