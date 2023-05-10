use bevy::prelude::*;
use rapier3d::na::Vector3;

use crate::{components::player::Player, physics::Physics};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add)
      .add_system(update)
      ;
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
        material: materials.add(Color::rgba(0.3, 0.5, 0.3, 0.1).into()),
        ..default()
      });
  }
}

fn update(
  mut query: Query<(&mut Transform, &Player)>,
  mut physics: ResMut<Physics>,
) {
  for (mut trans, player) in &mut query {
    // let rigid_body = &physics.rigid_body_set[player.body];
    // let t = rigid_body.translation().xyz();
    // trans.translation = Vec3::new(t.x, t.y, t.z);


    let p = trans.translation;
    let rigid_body = &mut physics.rigid_body_set[player.body];
    rigid_body.set_position(Vector3::new(p.x, p.y, p.z).into(), false);


    // info!("{:?}: {:?}", t.xyz(), trans.translation);
  }
}