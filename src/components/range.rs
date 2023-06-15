use bevy::{prelude::*, input::{mouse::MouseWheel, keyboard::KeyboardInput}};
use bevy_flycam::FlyCam;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update_point)
      .add_system(update_range);
  }
}

fn update_point(
  mut query: Query<(&Transform, &mut Range), With<FlyCam>>,
) {
  for (trans, mut range) in query.iter_mut() {
    let mut point = trans.translation + trans.forward() * range.dist;
    point = point.round();
    if range.point != point {
      range.point = point;
      // info!("range.point {:?}", point);
    }
  }
}

fn update_range(
  mut query: Query<&mut Range>,
  mut mouse_wheels: EventReader<MouseWheel>,
  keyboard_input: Res<Input<KeyCode>>,
  time: Res<Time>,

  // mut keyboard_input_events: EventReader<KeyboardInput>,
) {

  // for event in keyboard_input_events.iter() {
  //   info!("{:?}", event);
  // }

  for event in mouse_wheels.iter() {
    // info!("{:?}", event);

    for mut range in query.iter_mut() {
      range.dist += event.y * time.delta_seconds() * 30.0;
      // info!("dist {}", range.dist);
    }
  }

  if keyboard_input.just_pressed(KeyCode::Equals) {
    for mut range in query.iter_mut() {
      range.scale += 1;
      info!("range.scale {}", range.scale);
    }
  }

  if keyboard_input.just_pressed(KeyCode::Minus) {
    for mut range in query.iter_mut() {
      range.scale -= 1;
      info!("range.scale {}", range.scale);
    }
  }
}

/*
  It is redundant, will rename/refactor it later
 */
#[derive(Component)]
pub struct Range {
  pub point: Vec3,
  pub dist: f32,
  pub scale: u8,
}

impl Default for Range {
  fn default() -> Self {
    Self {
      point: Vec3::new(f32::NAN, f32::NAN, f32::NAN),
      dist: 8.0,
      scale: 2,
    }
  }
}
