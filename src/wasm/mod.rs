use bevy::prelude::*;
use web_sys::HtmlElement;
use flume::*;
use wasm_bindgen::prelude::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(update_fullscreen);
  }
}

#[allow(dead_code)]
fn startup(local_res: Res<LocalResource>,) {
  let send_mouse_move = local_res.send_mouse_move.clone();
  let cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    let _ = send_mouse_move.send((event.movement_x() as f32, event.movement_y() as f32));
  }) as Box<dyn FnMut(web_sys::MouseEvent)>);

  let window = web_sys::window().expect("no global `window` exists");
  window.set_onmousemove(Some(cb.as_ref().unchecked_ref()));
  cb.forget();
}

fn update_fullscreen(
  input: Res<Input<KeyCode>>,
) {
  if input.just_pressed(KeyCode::F) {
    let _ = html_body().request_fullscreen();
    html_body().request_pointer_lock();
  }
}

pub fn html_body() -> HtmlElement {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  body
}

#[allow(dead_code)]
#[derive(Resource)]
struct LocalResource {
  send_mouse_move: Sender<(f32, f32)>,
  recv_mouse_move: Receiver<(f32, f32)>,

}

impl Default for LocalResource {
  fn default() -> Self {
    let (send_mouse_move, recv_mouse_move) = flume::bounded(10);
    Self {
      send_mouse_move: send_mouse_move,
      recv_mouse_move: recv_mouse_move,
    }
  }
}