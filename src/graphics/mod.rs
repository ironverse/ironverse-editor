use bevy::prelude::*;

mod player;
mod range;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GraphicsResource::default())
      .add_plugin(player::CustomPlugin)
      // .add_plugin(range::CustomPlugin)
      .add_system(toggle_showhide);
  }
}


fn toggle_showhide(
  key_input: Res<Input<KeyCode>>,
  mut previews: Query<(&mut Visibility, &ChunkPreviewGraphics)>,
  mut graphics_res: ResMut<GraphicsResource>,
) {
  if key_input.just_pressed(KeyCode::P) {
    graphics_res.show_preview = !graphics_res.show_preview;

    info!("graphics_res.show_preview {}", graphics_res.show_preview);
  }

  for (mut visibility, _preview) in &mut previews {

    if !graphics_res.show_preview {
      *visibility = Visibility::Hidden;
    }

    if graphics_res.show_preview {
      *visibility = Visibility::Visible;
    }
  }
}


#[derive(Resource)]
pub struct GraphicsResource {
  pub show_preview: bool,
}

impl Default for GraphicsResource {
  fn default() -> Self {
    Self {
      show_preview: true,
    }
  }
}



#[derive(Component)]
pub struct ChunkGraphics {
  pub key: [i64; 3],
}

impl Default for ChunkGraphics {
  fn default() -> Self {
    Self {
      key: [i64::MAX; 3],
    }
  }
}


#[derive(Component, Clone)]
pub struct ChunkPreviewGraphics { }