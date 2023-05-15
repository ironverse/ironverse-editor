use bevy::prelude::*;
use bevy_flycam::FlyCam;

use crate::{components::player::Player, utils::Math};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add)
      .add_system(update);
  }
}

fn add(
  mut commands: Commands,
  player_query: Query<(Entity, &Player), Added<Player>>,

  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for (entity, player) in &player_query {

    info!("Add raycast debugger");
    commands
      .spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgba(0.0, 0.0, 0.6, 0.0).into()),
        transform: Transform::from_translation(Vec3::ZERO),
        visibility: Visibility::Visible,
        ..Default::default()
      })
      .with_children(|parent| {
        let height = 200.0;
        let mut transform = Transform::from_scale(Vec3::new(0.1, height, 0.1));
        transform.translation = Vec3::new(0.0, height * 0.5, 0.0);

        parent
          .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgba(0.0, 0.0, 0.6, 0.3).into()),
            transform: transform,
            ..Default::default()
          });
      })
      .insert(Raycast);
  }
}

fn update(
  mut raycast: Query<(&mut Transform, &Raycast)>,
  mut cam_query: Query<(&GlobalTransform, &crate::components::raycast::Raycast), With<FlyCam>>,
) {
  for (cam_trans, ray) in &cam_query {
    for (mut ray_trans, raycast) in &mut raycast {
      let trans = cam_trans.compute_transform();
      let look_at = trans.forward();

      let pos = trans.translation + ray.adj;
      let t = Transform::from_xyz(pos.x, pos.y, pos.z).looking_to(look_at, Vec3::Y);
      
      *ray_trans = t;
      ray_trans.rotation *= Quat::from_rotation_x(std::f32::consts::PI * -0.5);
    }
  }
}


#[derive(Component)]
pub struct Raycast;