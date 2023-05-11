use bevy::{prelude::*, window::PrimaryWindow, diagnostic::Diagnostic};
use bevy_egui::{EguiContexts, egui::{self, TextureId, Frame, Color32, Style, ImageButton, Rect, Vec2, Pos2, RichText}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(show_texts)
      ;
  }
}


/*
  Fps
  Total loaded chunks
  Colliders
  Player position
  Chunks rendered
 */
fn show_texts(
  mut ctx: EguiContexts,
  windows: Query<&Window, With<PrimaryWindow>>,
  diagnostic: &Diagnostic
) {
  let res = windows.get_single();
  if res.is_err() {
    return;
  }
  let window = res.unwrap();
  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
    ..Default::default()
  };

  let x = 0.0;
  let y = 0.0;

  egui::Window::new("block_ui")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(Rect::from_min_max(
      Pos2::new(0.0, 0.0),
      Pos2::new(window.width(), window.height())
    ))
    .show(ctx.ctx_mut(), |ui| {
      ui.vertical(|ui| {
        let mut style = Style::default();
        style.spacing.item_spacing = Vec2::new(0.0, 0.0);
        ui.set_style(style);

        ui.label(
          RichText::new("FPS:")
            .color(Color32::WHITE)
            .size(20.0)
        );
      });
    });
}