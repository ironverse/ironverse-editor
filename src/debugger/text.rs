use bevy::{prelude::*, window::PrimaryWindow, diagnostic::{Diagnostic, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, Diagnostics}};
use bevy_egui::{EguiContexts, egui::{self, TextureId, Frame, Color32, Style, ImageButton, Rect, Vec2, Pos2, RichText}};

use crate::data::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_plugin(FrameTimeDiagnosticsPlugin::default())
      // .add_plugin(LogDiagnosticsPlugin::default())
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
  diagnostics: Res<Diagnostics>,

  time: Res<Time>,
  mut local_res: ResMut<LocalResource>,

  players: Query<&Transform, With<Player>>,
) {
  let res = windows.get_single();
  if res.is_err() {
    return;
  }

  let fps = match diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
    Some(diag) => {
      let mut fps = 0.0;
      if diag.average().is_some() {
        fps = diag.average().unwrap()
      }
      fps
    },
    None => 0.0
  };

  local_res.fps += 1.0;
  // info!("test {}", local_res.fps);
  if local_res.timer.tick(time.delta()).finished() {

    // info!("fps {}", local_res.fps);

    local_res.fps = 0.0;
  }
  

  // info!("fps {:?}: {:?}", fps, settings.limiter);

  let window = res.unwrap();
  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
    ..Default::default()
  };

  let x = 0.0;
  let y = 0.0;

  let mut player_pos = Vec3::ZERO;
  for trans in &players {
    player_pos = trans.translation.clone();
  }

  egui::Window::new("DebuggerTexts")
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

        
        ui.label(
          RichText::new(format!("Pos: {:?}", player_pos))
            .color(Color32::WHITE)
            .size(20.0)
        );
      });
    });
}


#[derive(Resource)]
struct LocalResource {
  timer: Timer,
  fps: f32,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      timer: Timer::from_seconds(1.0, TimerMode::Repeating),
      fps: 0.0,
    }
  }
}