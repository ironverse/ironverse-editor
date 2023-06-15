use bevy::prelude::*;

use crate::data::Player;

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
  query: Query<(Entity, &Player), Added<Player>>,

  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for (entity, player) in &query {
    commands
      .spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
    .insert(Range { parent: entity.clone() });
  }
}

fn update(
  players: Query<&crate::components::range::Range, With<Player>>,
  mut ranges: Query<(&mut Transform, &Range), Without<Player>>,
) {

  for (mut trans, range) in &mut ranges {
    let range_comp = players.get(range.parent).unwrap();
    trans.translation = range_comp.point;
  }
  
}


#[derive(Component)]
struct Range {
  parent: Entity
}

impl Default for Range {
  fn default() -> Self {
    Self {
      parent: Entity::PLACEHOLDER,
    }
  }
}
