use bevy::{prelude::*, input::{mouse::MouseButtonInput, ButtonState}, window::CursorGrabMode};
use bevy_flycam::MovementSettings;
use web_sys::HtmlElement;
use flume::*;
use wasm_bindgen::prelude::*;
use crate::{input::MouseInput, data::CursorState};

mod load_file;
pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_event::<PointerLockEvent>()
      .add_event::<MouseMoveEvent>()
      .add_event::<WasmInputEvent>()
      .add_plugin(load_file::CustomPlugin)
      .add_systems((update_fullscreen, update_pointer_events))
      .add_system(grab_mouse);

    app
      .add_startup_system(startup)
      .add_system(send_mouse_events);
  }
}

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

  // let send_key = local_res.send_key.clone();
  // let cb = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
  //   let _ = send_key.send(event.char_code());
  // }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

  // window.set_onkeydown(Some(cb.as_ref().unchecked_ref()));
  // cb.forget();
}

fn send_mouse_events(
  local_res: Res<LocalResource>,
  // mut wasm_events: EventWriter<WasmInputEvent>,
  mut mouse_inputs: EventWriter<MouseInput>,
) {
  for e in local_res.recv_mouse_click.drain() {
    // info!("clicked {}", is_pointer_locked());
    if !is_pointer_locked() {
      continue;
    }
    
    // Defer: Improve getting mouse events from WASM
    if e == 0 {
      mouse_inputs.send(MouseInput { mouse_button_input: MouseButtonInput {
        button: MouseButton::Left,
        state: ButtonState::Pressed,
      }});
    }
    if e == 2 {
      mouse_inputs.send(MouseInput { mouse_button_input: MouseButtonInput {
        button: MouseButton::Right,
        state: ButtonState::Pressed,
      }});
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

  return;
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
fn grab_mouse(
  mut windows: Query<&mut Window>,
  mouse: Res<Input<MouseButton>>,
  key: Res<Input<KeyCode>>,
  mut cursor_state_next: ResMut<NextState<CursorState>>,
) {
  let mut window = windows.single_mut();

  if is_pointer_locked() {
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
  } else {
    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
  }

  
  if mouse.just_pressed(MouseButton::Left) {
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
    cursor_state_next.set(CursorState::Locked);
  }

  if key.just_pressed(KeyCode::Escape) {
    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
    cursor_state_next.set(CursorState::None);
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
  send_key: Sender<u32>,
  recv_key: Receiver<u32>,
}

impl Default for LocalResource {
  fn default() -> Self {
    let (send_mouse_click, recv_mouse_click) = flume::bounded(10);
    let (send_key, recv_key) = flume::bounded(10);
    Self {
      send_mouse_click: send_mouse_click,
      recv_mouse_click: recv_mouse_click,
      send_key: send_key,
      recv_key: recv_key,
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