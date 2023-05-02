use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{EguiContexts, egui::{self, TextureId, Frame, Color32, Style, ImageButton, Rect, Vec2, Pos2}};
use crate::input::hotbar::HotbarResource;


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(HotbarUIResource::default())
      .add_system(render)
      ;
  }
}


fn render(
  mut ctx: EguiContexts,
  mut is_initialized: Local<bool>,
  mut texture_id: Local<TextureId>,
  images: Local<Images>,

  windows: Query<&Window, With<PrimaryWindow>>,

  // windows: Res<Windows>,
  // mut play_res: ResMut<PlayResource>,
  hotbar_res: Res<HotbarResource>,
  mut hotbar_ui_res: ResMut<HotbarUIResource>,
) {
  let res = windows.get_single();
  if res.is_err() {
    return;
  }
  let window = res.unwrap();


  if !*is_initialized {
    *is_initialized = true;
    *texture_id = ctx.add_image(images.block.clone_weak());
  }

  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
    ..Default::default()
  };

  let size = [50.0, 50.0];
  // let blocks = play_res.key_mapping.len();
  let blocks = 10;
  let x = (window.width() * 0.5) - size[0] * (blocks as f32) * 0.5;
  let y = (window.height() * 1.0) - size[1] * 1.5;

  egui::Window::new("block_ui")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(Rect {
      min: [x, y].into(),
      max: [x, y].into(),
    })
    .show(ctx.ctx_mut(), |ui| {
      ui.horizontal(|ui| {
        let mut style = Style::default();
        style.spacing.item_spacing = Vec2::new(0.0, 0.0);
        ui.set_style(style);

        for index in 0..hotbar_res.bars.len() {
          // let bar = hotbar_res.bars[index];
          let mut img_button = ImageButton::new(*texture_id, size.clone()).frame(false);
          let res = ui.add(img_button);
          if res.clicked() {
            
          }

          hotbar_ui_res.pos_bars[index] = res.rect.clone();
        }
        // for _ in 0..blocks {
        //   // let key_map = &key_mapping[index];
        //   let mut img_button = ImageButton::new(*texture_id, size.clone()).frame(false);

        //   // if play_res.selected_key_code == key_map.key_code {
        //   //   img_button = img_button.tint(Color32::RED);
        //   //   play_res.selected_voxel = key_map.voxel_type;
        //   // }

        //   let res = ui.add(img_button);
        //   // if res.clicked() {
        //   //   play_res.selected_voxel = key_map.voxel_type;
        //   // }

        //   // hotbar_res.pos_bars[index] = res.rect.clone();
        // }
      });
    });
}

struct Images {
  block: Handle<Image>,
}

impl FromWorld for Images {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
    Self {
      block: asset_server.load("block_ui.png"),
    }
  }
}

#[derive(Resource)]
pub struct HotbarUIResource {
  pub pos_bars: [Rect; 10]
}

impl Default for HotbarUIResource {
  fn default() -> Self {
    Self {
      pos_bars: [Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(0.0, 0.0)); 10],
    }
  }
}