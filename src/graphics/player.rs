use bevy::prelude::*;
use rapier3d::na::Vector3;

use crate::{physics::Physics, data::Player};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add);
  }
}

fn add(
  mut commands: Commands,
  query: Query<(Entity, &Player), Added<Player>>,

  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for (entity, player) in &query {
    info!("{:?} changed: {:?}", entity, player,);

    commands
      .entity(entity)
      .insert(PbrBundle {
        mesh: meshes.add(shape::Capsule { 
          ..default()
        }.into()),
        material: materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into()),
        ..default()
      });
  }
}
