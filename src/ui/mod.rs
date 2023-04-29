use bevy::prelude::*;
use bevy_egui::{egui::{self, Frame, Ui, Rect}, EguiPlugin};

mod menu;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(UIResource::default())
      .add_plugin(EguiPlugin)
      .add_state::<UIState>()
      // .add_plugin(hotbar::CustomPlugin)
      .add_plugin(menu::CustomPlugin)
      // .add_plugin(new::CustomPlugin)
      // .add_plugin(load::CustomPlugin)
      // .add_plugin(inventory::CustomPlugin)
      ;

    app
      .add_system(update);
  }
}

fn update(
  key_input: Res<Input<KeyCode>>,
  state: Res<State<UIState>>,
  mut next_state: ResMut<NextState<UIState>>,
) {
  if key_input.just_pressed(KeyCode::F) {
    
    match state.0 {
      UIState::Default => { next_state.set(UIState::Menu); },
      UIState::Menu => { next_state.set(UIState::Default); },
      _ => { next_state.set(UIState::Default); },
    }

    println!("Toggle show menu {:?}", state.0);
  }
}

#[derive(Resource)]
pub struct UIResource {
  pub load_file_path: String,
  pub load_file_init: bool,   // Have to change later after updating bevy version
}

impl Default for UIResource {
  fn default() -> Self {
    Self {
      load_file_path: "".to_string(),
      load_file_init: true,
    }
  }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum UIState {
  #[default]
  None,
  Default,
  Menu,
  New,
  Restarting,
  Load,
  Save,
  Inventory,
}


pub struct UIUtils;

impl UIUtils {

  pub fn window(
    name: &str,
    frame: Frame,
    rect: Rect,
    context: &egui::Context, 
    add_contents: impl FnOnce(&mut Ui)
  ) {
    
    egui::Window::new(name)
      .title_bar(false)
      .frame(frame)
      .fixed_rect(rect)
      .show(context, add_contents);
  }
}

