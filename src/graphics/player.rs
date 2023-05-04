use bevy::prelude::*;

use crate::components::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update)
      ;
  }
}

fn update(
  mut commands: Commands,
  query: Query<(Entity, &Player), Added<Player>>,

  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for (entity, player) in &query {
    info!("{:?} changed: {:?}", entity, player,);

    let parent = commands.spawn(PbrBundle {
      mesh: meshes.add(shape::Capsule { 
        ..default()
      }.into()),
      material: materials.add(Color::rgba(0.3, 0.5, 0.3, 0.1).into()),
      ..default()
    })
    .id();

    commands.entity(entity).set_parent_in_place(parent);
  }
}