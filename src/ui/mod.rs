use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, Frame, Ui, Rect, Color32}, EguiPlugin, EguiContexts};

pub mod hotbar;
pub mod inventory;
pub mod menu;


pub struct NonePlugin;
impl Plugin for NonePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(UIResource::default())
      .add_state::<UIState>()
      .add_plugin(menu::NonePlugin);
  }
}


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(UIResource::default())
      .add_state::<UIState>()
      .add_plugin(EguiPlugin)
      .add_plugin(hotbar::CustomPlugin)
      .add_plugin(inventory::CustomPlugin)
      .add_plugin(menu::CustomPlugin)
      .add_system(crosshair);
  }
}


fn crosshair(
  mut ctx: EguiContexts,
  mut is_initialized: Local<bool>,
  mut texture_id: Local<egui::TextureId>,
  images: Local<Images>,

  windows: Query<&Window, With<PrimaryWindow>>,
) {
  let res = windows.get_single();
  if res.is_err() {
    return;
  }
  let window = res.unwrap();

  if !*is_initialized {
    *is_initialized = true;
    *texture_id = ctx.add_image(images.crosshair.clone_weak());
  }

  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
    ..Default::default()
  };

  let size = [50.0, 50.0];
  let x = (window.width() * 0.5) - size[0] * 0.5;
  let y = (window.height() * 0.5) - size[1] * 0.5;

  egui::Window::new("crosshair")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(Rect {
      min: [x, y].into(),
      max: [x, y].into(),
    })
    .show(ctx.ctx_mut(), |ui| {
      ui.image(*texture_id, size.clone());
    });
}



#[derive(Resource)]
pub struct UIResource {
  pub load_file_path: String,
  pub load_file_init: bool,   // Have to change later after updating bevy version
  pub total_materials: u8,
}

impl Default for UIResource {
  fn default() -> Self {
    Self {
      load_file_path: "".to_string(),
      load_file_init: true,
      total_materials: 16,
    }
  }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum UIState {
  #[default]
  Default,
  Menu,
  New,
  Restarting,
  Load,
  Save,

  // #[default]
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



struct Images {
  crosshair: Handle<Image>,
}

impl FromWorld for Images {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
    Self {
      crosshair: asset_server.load("crosshair.png"),
    }
  }
}


