use bevy::prelude::*;

mod terrain;
mod physics;
mod graphics;
mod camera;
mod utils;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(terrain::CustomPlugin)
    .add_plugin(physics::CustomPlugin)
    .add_plugin(graphics::CustomPlugin)
    // .add_plugin(camera::CustomPlugin)
    // .add_startup_system(startup)
    .run();

}

fn startup(mut commands: Commands,) {
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 3000.0,
      ..Default::default()
    },
    transform: Transform::from_xyz(-3.0, 2.0, -1.0),
    ..Default::default()
  });
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 3000.0,
      ..Default::default()
    },
    transform: Transform::from_xyz(3.0, 2.0, 1.0),
    ..Default::default()
  });

  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}