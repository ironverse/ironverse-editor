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
      // .insert_resource(WasmMouseTracker::default())
      .add_startup_system(startup)
      .add_system(update_fullscreen)
      .add_system(update_resize)
      .add_system(update_mouse_move)
      .add_system(update_mouse_down)
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

  let send_mouse_move = local_res.send_mouse_move.clone();
  let cb1 = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    let _ = send_mouse_move.send((event.movement_x() as f32, event.movement_y() as f32));
  }) as Box<dyn FnMut(web_sys::MouseEvent)>);

  window.set_onmousemove(Some(cb1.as_ref().unchecked_ref()));
  cb1.forget();
}

fn update_fullscreen(input: Res<Input<KeyCode>>,) {
  if input.just_pressed(KeyCode::F) {
    info!("Fullscreen");
    html_body().request_fullscreen();
    html_body().request_pointer_lock();
  }

  if input.just_pressed(KeyCode::LAlt) {
    html_body().request_pointer_lock();
  }
}

fn update_resize(
  local_res: Res<LocalResource>,
) {
  for _resize in local_res.recv_resize.drain() {
    info!("resize");
  }
}

fn update_mouse_move(local_res: Res<LocalResource>,) {
  for (x, y) in local_res.recv_mouse_move.drain() {
    info!("move {} {}", x, y);
  }
}

fn update_mouse_down(
  mouse_button_input: Res<Input<MouseButton>>,
) {
  if mouse_button_input.just_pressed(MouseButton::Left) {
    info!("left mouse currently pressed");
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
  send_mouse_move: Sender<(f32, f32)>,
  recv_mouse_move: Receiver<(f32, f32)>,

}

impl Default for LocalResource {
  fn default() -> Self {
    let (send, recv) = flume::bounded(1);
    let (send_mouse_move, recv_mouse_move) = flume::bounded(10);
    Self {
      send_resize: send,
      recv_resize: recv,
      send_mouse_move: send_mouse_move,
      recv_mouse_move: recv_mouse_move,
    }
  }
}