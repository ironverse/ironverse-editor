use bevy::prelude::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(HotbarResource::default())
      .add_startup_system(startup);
  }
}

fn startup() {
  
}



#[derive(Resource)]
pub struct HotbarResource {
  pub bars: Vec<Bar>,
  pub selected_index: u8,
}

impl Default for HotbarResource {
  fn default() -> Self {
    Self {
      bars: vec![
        Bar::new(KeyCode::Key1, 1),
        Bar::new(KeyCode::Key2, 2),
        Bar::new(KeyCode::Key3, 3),
        Bar::new(KeyCode::Key4, 4),
        Bar::new(KeyCode::Key5, 5),
        Bar::new(KeyCode::Key6, 6),
        Bar::new(KeyCode::Key7, 7),
        Bar::new(KeyCode::Key8, 8),
        Bar::new(KeyCode::Key9, 9),
        Bar::new(KeyCode::Key0, 10),
      ],
      selected_index: 0,
    }
  }
}

pub struct Bar {
  pub key_code: KeyCode,
  pub voxel: u8,
}

impl Bar {
  pub fn new(k: KeyCode, v: u8) -> Self {
    Bar { key_code: k, voxel: v }
  }
}


