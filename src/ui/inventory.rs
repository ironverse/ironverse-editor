use bevy::{prelude::*, window::PrimaryWindow, asset::LoadState};
use bevy_egui::{EguiContexts, egui::{self, TextureId, Frame, Color32, Style, ImageButton, Rect, Vec2, Pos2, Sense}};
use crate::{input::hotbar::HotbarResource, data::CursorState};
use super::{UIState, hotbar::HotbarUIResource};


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_startup_system(startup)
      .add_system(prepare_texture)
      .add_system(toggle_show)
      .add_systems(
        (render, render_dragging.after(render))
          .in_set(OnUpdate(UIState::Inventory)))
      ;
  }
}

fn startup(
  mut commands: Commands, asset_server: Res<AssetServer>
) {
  commands.insert_resource(InventoryTexture {
    is_loaded: false,
    slot: asset_server.load("slot.png"),
    albedo: asset_server.load("textures/textures_items.png"),
    slot_id: TextureId::default(),
    albedo_id: TextureId::default(),
  });
}


fn prepare_texture(
  mut ctx: EguiContexts,
  mut loading_texture: ResMut<InventoryTexture>,
  mut images: ResMut<Assets<Image>>,
  asset_server: Res<AssetServer>,
) {
  if loading_texture.is_loaded
    || asset_server.get_load_state(loading_texture.albedo.clone()) != LoadState::Loaded {
    return;
  }
  loading_texture.is_loaded = true;
  loading_texture.slot_id = ctx.add_image(loading_texture.slot.clone_weak());
  loading_texture.albedo_id = ctx.add_image(loading_texture.albedo.clone_weak());
}

fn toggle_show(
  key_input: Res<Input<KeyCode>>,
  mut next_state: ResMut<NextState<UIState>>,
  ui_state: Res<State<UIState>>,
) {
  if key_input.just_pressed(KeyCode::I) {
    match ui_state.0 {
      UIState::Default => {
        next_state.set(UIState::Inventory);
      },
      UIState::Inventory => {
        next_state.set(UIState::Default);
      },
      _ => ()
    }

    info!("ui_state.0 {:?}", ui_state.0);
  }
}



fn render(
  mut ctx: EguiContexts,
  windows: Query<&Window, With<PrimaryWindow>>,

  loading_texture: Res<InventoryTexture>,
  mut local_res: ResMut<LocalResource>,
) {
  let res = windows.get_single();
  if res.is_err() || !loading_texture.is_loaded {
    return;
  }
  
  let window = res.unwrap();

  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(50, 50, 50, 255),
    ..Default::default()
  };

  let size = [400.0, 200.0];
  let x = (window.width() * 0.5) - size[0] * 0.5;
  let y = window.height() * 0.1;

  let slot_size = [40.0, 40.0];
  let item_size = [31.0, 31.0];
  egui::Window::new("inventory")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(egui::Rect {
      min: [x, y].into(),
      max: [x + size[0], y + size[1]].into(),
    })
    .show(ctx.ctx_mut(), |ui| {
      ui.set_min_size(size.into());
      ui.set_max_size(size.into());

      egui::Grid::new("inventory_grid").show(ui, |ui| {
        let total_items = 16.0;
        let max = 1.0 / total_items;

        for i in 0..local_res.slots.len() {
          let slot = &mut local_res.slots[i];
          let index = slot.item_id as f32 - 1.0;

          let img = egui::Image::new(loading_texture.slot_id, slot_size.clone());
          let rect = ui.add(img).rect;
          slot.pos = rect.min;

          let item = egui::Image::new(loading_texture.albedo_id, item_size.clone()).uv(Rect {
            min: Pos2::new(0.0, max * index),
            max: Pos2::new(1.0, max * (index + 1.0) ),
          });

          let adj = Vec2::new(4.0, 4.0);
          let pos = slot.pos + adj;
          let rect = Rect::from_min_size(pos, item_size.into());
          let _item_res = ui.put(rect, item);
        }


      });
    });
}


fn render_dragging(
  mut ctx: EguiContexts,
  windows: Query<&Window, With<PrimaryWindow>>,

  loading_texture: Res<InventoryTexture>,
  mut local_res: ResMut<LocalResource>,

  mut hotbar_res: ResMut<HotbarResource>,
  hotbar_ui_res: Res<HotbarUIResource>,
) {
  let res = windows.get_single();
  if res.is_err() || !loading_texture.is_loaded {
    return;
  }
  let window = res.unwrap();

  let item_size = [31.0, 31.0];

  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
    ..Default::default()
  };
  let drag_start = Pos2::new(0.0, 0.0);
  let drag_size = Pos2::new(window.width(), window.height() * 0.7);
  let max_size = Vec2::new(window.width(), window.height());
  
  egui::Window::new("drag_inventory")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(egui::Rect {
      min: drag_start,
      max: drag_size,
    })
    .show(ctx.ctx_mut(), |ui| {
      ui.set_min_size(Vec2::new(drag_size.x, drag_size.y));
      ui.set_max_size(max_size);

      egui::Grid::new("drag_inventory_grid").show(ui, |ui| {
        let total_items = 16.0;
        let max = 1.0 / total_items;

        for i in 0..local_res.slots.len() {
          let slot = &mut local_res.slots[i];

          let mut alpha = 0;
          if slot.is_dragged {
            alpha = 255;
          }

          if slot.item_id == 0 {
            continue;
          }

          let index = slot.item_id as f32 - 1.0;
          let item = egui::ImageButton::new(loading_texture.albedo_id, item_size.clone()).uv(Rect {
            min: Pos2::new(0.0, max * index),
            max: Pos2::new(1.0, max * (index + 1.0) ),
          })
          .sense(Sense::drag())
          .tint(Color32::from_rgba_unmultiplied(255, 255, 255, alpha))
          .frame(slot.is_dragged);

          let pos = slot.pos + slot.anchor_pos;

          let rect = egui::Rect::from_min_size(pos, item_size.into());
          let item_res = ui.put(rect, item);
          if item_res.dragged() {
            slot.anchor_pos += item_res.drag_delta();
          }
          slot.is_dragged = item_res.dragged();

          if item_res.drag_released() {
            slot.anchor_pos = Vec2::new(0.0, 0.0);

            'main: for hot_index in 0..hotbar_ui_res.pos_bars.len() {
              if hotbar_ui_res.pos_bars[hot_index].intersects(rect) {
                hotbar_res.bars[hot_index].voxel = slot.item_id as u8;
                break 'main;
              }
            }
          }
        }

      });
    });

}




#[derive(Resource)]
pub struct InventoryTexture {
  pub is_loaded: bool,
  slot: Handle<Image>,
  pub albedo: Handle<Image>,
  slot_id: TextureId,
  pub albedo_id: TextureId,
}

#[derive(Resource)]
struct LocalResource {
  slots: Vec<Slot>
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      slots: vec![
        Slot { 
          pos: Pos2::new(0.0, 0.0), 
          anchor_pos: Vec2::new(0.0, 0.0),
          is_dragged: false,
          item_id: 1, 
        },
        Slot { 
          pos: Pos2::new(0.0, 0.0), 
          anchor_pos: Vec2::new(0.0, 0.0),
          is_dragged: false,
          item_id: 2, 
        },
        Slot { 
          pos: Pos2::new(0.0, 0.0), 
          anchor_pos: Vec2::new(0.0, 0.0),
          is_dragged: false,
          item_id: 3, 
        }
      ],
    }
  }
}

struct Slot {
  pos: Pos2,
  anchor_pos: Vec2,
  is_dragged: bool,
  item_id: u32,
}
