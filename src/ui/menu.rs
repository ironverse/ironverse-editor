use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, Color32, Frame, Vec2, Button}, EguiContexts};
use bevy_egui::egui::Rect;
use super::{UIResource, UIState};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(render.in_set(OnUpdate(UIState::Menu)))
      ;
  }
}

fn render(
  mut commands: Commands,
  mut contexts: EguiContexts,
  windows: Query<(Entity, &Window), With<PrimaryWindow>>,
  mut ui_res: ResMut<UIResource>,
  state: Res<State<UIState>>,
  mut next_state: ResMut<NextState<UIState>>,
) {
  let res = windows.get_single();
  if res.is_err() {
    return;
  }
  let (entity, window) = res.unwrap();
  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 255),
    ..Default::default()
  };

  let size = [200.0, 300.0];
  let x = (window.width() * 0.5) - size[0] * 0.5;
  let y = window.height() * 0.1;
  let button_size = Vec2::new(125.0, 50.0);

  egui::Window::new("menu")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(Rect {
      min: [x, y].into(),
      max: [x + size[0], y + size[1]].into(),
    })
    .show(contexts.ctx_mut(), |ui| {
      ui.set_min_size(size.into());

      ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        let back_to_game = Button::new("Back to Game")
          .min_size(button_size);
        if ui.add(back_to_game).clicked() {
          next_state.set(UIState::Default);
        }

        ui.add_space(20.0);
        let new = Button::new("New")
          .min_size(button_size);
        if ui.add(new).clicked() {
          next_state.set(UIState::New);
        }

        ui.add_space(20.0);
        let load = Button::new("Load")
          .min_size(button_size);
        if ui.add(load).clicked() {
          if let Some(path) = rfd::FileDialog::new().pick_file() {
            ui_res.load_file_path = path.to_str().unwrap().to_string();
            ui_res.load_file_init = false;
            next_state.set(UIState::Load);
          }
        }

        ui.add_space(20.0);
        let save = Button::new("Save")
          .min_size(button_size);
        if ui.add(save).clicked() {
          if let Some(path) = rfd::FileDialog::new().save_file() {
            ui_res.load_file_path = path.to_str().unwrap().to_string();
            next_state.set(UIState::Save);
          }
        }

        ui.add_space(20.0);
        let quit = Button::new("Quit")
          .min_size(button_size);
        if ui.add(quit).clicked() {
          commands.entity(entity).despawn();
        }
      });
    });

}