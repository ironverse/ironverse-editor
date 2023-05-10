use bevy::prelude::*;
use bevy_flycam::MovementSettings;
use web_sys::HtmlElement;
use flume::*;
use wasm_bindgen::prelude::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_event::<PointerLockEvent>()
      .add_event::<MouseMoveEvent>()
      .add_event::<WasmInputEvent>()
      .add_systems((update_fullscreen, update_pointer_events, update_mouse_events));

    app
      .add_startup_system(startup)
      .add_system(mouse_move);
  }
}

#[allow(dead_code)]
fn startup(local_res: Res<LocalResource>,) {
  let send_mouse_click = local_res.send_mouse_click.clone();
  let cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    // let _ = send_mouse_move.send((event.movement_x() as f32, event.movement_y() as f32));
    let _ = send_mouse_click.send(event.button());
    // info!("test");
  }) as Box<dyn FnMut(web_sys::MouseEvent)>);

  let window = web_sys::window().expect("no global `window` exists");
  window.set_onmousedown(Some(cb.as_ref().unchecked_ref()));
  cb.forget();


  // let cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
  //   // let _ = send_mouse_move.send((event.movement_x() as f32, event.movement_y() as f32));
  //   event.bubbles()
  // }) as Box<dyn FnMut(web_sys::MouseEvent)>);

  // window.set_onmousedown(Some(cb.as_ref().unchecked_ref()));
}

fn mouse_move(
  local_res: Res<LocalResource>,
  mut wasm_events: EventWriter<WasmInputEvent>,
) {
  for e in local_res.recv_mouse_click.drain() {
    if !is_pointer_locked() {
      continue;
    }
    
    if e == 0 {
      wasm_events.send(WasmInputEvent { mouse: MouseButton::Left });
    }
    if e == 2 {
      wasm_events.send(WasmInputEvent { mouse: MouseButton::Right });
    }
  }
}


fn update_fullscreen(
  input: Res<Input<KeyCode>>,
) {
  if input.just_pressed(KeyCode::F) {
    let _ = html_body().request_fullscreen();
    html_body().request_pointer_lock();
  }
}

fn update_pointer_events(
  mut events: EventReader<PointerLockEvent>,
  mut move_setting_res: ResMut<MovementSettings>,
) {
  // TODO: Need to confirm if truly locked the pointer or exit later on

  for e in events.iter() {
    if e.0 {
      info!("wasm pointer {}", e.0);
      html_body().request_pointer_lock();
      move_setting_res.sensitivity = 0.00012;
    } else {
      info!("wasm pointer {}", e.0);
      let window = web_sys::window().expect("no global `window` exists");
      let document = window.document().expect("should have a document on window");
      document.exit_pointer_lock();
      move_setting_res.sensitivity = 0.0;
    }
  }
}

fn update_mouse_events(
  mut events: EventReader<MouseMoveEvent>,
  mut move_setting_res: ResMut<MovementSettings>,
) {
  for e in events.iter() {
    // if e.0 {
    //   move_setting_res.sensitivity = 0.00012;
    // } else {
    //   move_setting_res.sensitivity = 0.0;
    // }
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
  send_mouse_click: Sender<i16>,
  recv_mouse_click: Receiver<i16>,

}

impl Default for LocalResource {
  fn default() -> Self {
    let (send_mouse_click, recv_mouse_click) = flume::bounded(10);
    Self {
      send_mouse_click: send_mouse_click,
      recv_mouse_click: recv_mouse_click,
    }
  }
}

pub struct PointerLockEvent(pub bool);

pub struct MouseMoveEvent(bool);

pub struct WasmInputEvent {
  pub mouse: MouseButton,
}


pub fn is_pointer_locked() -> bool {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  

  let lock_op = document.pointer_lock_element();
  lock_op.is_some()
}