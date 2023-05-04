use bevy::prelude::*;
use bevy_flycam::FlyCam;
use super::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(startup)
      .add_system(add);
  }
}


fn startup() {

}

fn add(
  mut commands: Commands,
  query: Query<(Entity, &Player), Added<Player>>,
) {
  for (entity, player) in &query {

    info!("Add cam");
    commands
      .entity(entity)
      .insert((
        Camera3dBundle {
          transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
          ..Default::default()
        },
        FlyCam,
      ));
  }
}








