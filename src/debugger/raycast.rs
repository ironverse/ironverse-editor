use bevy::{
  pbr::{MaterialPipeline, MaterialPipelineKey},
  prelude::*,
  reflect::TypeUuid,
  render::{
      mesh::{MeshVertexBufferLayout, PrimitiveTopology},
      render_resource::{
          AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
          SpecializedMeshPipelineError,
      },
  },
};
use bevy_flycam::FlyCam;

use crate::{components::player::Player, utils::Math};

/* pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(MaterialPlugin::<LineMaterial>::default())
      .add_system(add)
      .add_system(update);
  }
}

fn add(
  mut commands: Commands,
  player_query: Query<(Entity, &Player), Added<Player>>,

  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut line_materials: ResMut<Assets<LineMaterial>>,
) {
  for (entity, player) in &player_query {

    info!("Add raycast debugger");
    // commands
    //   .spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgba(0.0, 0.0, 0.6, 0.0).into()),
    //     transform: Transform::from_translation(Vec3::ZERO),
    //     visibility: Visibility::Visible,
    //     ..Default::default()
    //   })
    //   .with_children(|parent| {
    //     let height = 200.0;
    //     let mut transform = Transform::from_scale(Vec3::new(0.1, height, 0.1));
    //     transform.translation = Vec3::new(0.0, height * 0.5, 0.0);

    //     parent
    //       .spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(Color::rgba(0.0, 0.0, 0.6, 0.3).into()),
    //         transform: transform,
    //         ..Default::default()
    //       });
    //   })
    //   .insert(Raycast);

    commands.spawn((
      MaterialMeshBundle {
        mesh: meshes.add(
          Mesh::from(LineList {
            lines: vec![
              (Vec3::ZERO, Vec3::new(0.0, 0.0, 5.0)),
              // (Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
            ],
          })
        ),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: line_materials.add(LineMaterial {
          color: Color::GREEN,
        }),
        ..default()
      },
      Raycast,
    ));
  }
}

fn update(
  mut raycast: Query<(&mut Transform, &Raycast)>,
  mut cam_query: Query<&GlobalTransform, With<FlyCam>>,
) {
  for cam_trans in &cam_query {
    for (mut ray_trans, raycast) in &mut raycast {
      let trans = cam_trans.compute_transform();
      // let (mut pitch, mut yaw, _) = trans.rotation.to_euler(EulerRot::XYZ);

      // let t = Transform::from_xyz(0.0, 0.0, 0.0).looking_to(Vec3::X, Vec3::Y);

      // let look_at = Math::rot_to_look_at(Vec3::new(pitch, yaw, 0.0));

      // let adj = Vec3::new(0.0, 1.5, 0.0);
      // // let start_pos = trans.translation + adj;
      // let start_pos = Vec3::new(0.0, 0.0, 0.0);

      // ray_trans.translation = start_pos;
      // ray_trans.rotation = Math::look_at_to_rotation_quat(look_at);
      // ray_trans.rotation = ray_trans.rotation * Quat::from_rotation_x(std::f32::consts::PI * 0.5);
    }
  }
}


#[derive(Component)]
pub struct Raycast;


#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "050ce6ac-080a-4d8c-b6b5-b5bab7560d8f"]
struct LineMaterial {
  #[uniform(0)]
  color: Color,
}

impl Material for LineMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/line_material.wgsl".into()
  }

  fn specialize(
    _pipeline: &MaterialPipeline<Self>,
    descriptor: &mut RenderPipelineDescriptor,
    _layout: &MeshVertexBufferLayout,
    _key: MaterialPipelineKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
    // This is the important part to tell bevy to render this material as a line between vertices
    descriptor.primitive.polygon_mode = PolygonMode::Line;
    Ok(())
  }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
  fn from(line: LineList) -> Self {
    // This tells wgpu that the positions are list of lines
    // where every pair is a start and end point
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);

    let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh
  }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct LineStrip {
  pub points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
  fn from(line: LineStrip) -> Self {
    // This tells wgpu that the positions are a list of points
    // where a line will be drawn between each consecutive point
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line.points);
    mesh
  }
}
 */


 pub struct CustomPlugin;
 impl Plugin for CustomPlugin {
   fn build(&self, app: &mut App) {
     app
      .add_plugin(MaterialPlugin::<LineMaterial>::default())
      .add_startup_system(setup);
   }
 }


fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<LineMaterial>>,
) {
  // Spawn a list of lines with start and end points for each lines
  commands.spawn(MaterialMeshBundle {
      mesh: meshes.add(Mesh::from(LineList {
          lines: vec![
              (Vec3::ZERO, Vec3::new(1.0, 1.0, 0.0)),
              (Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
          ],
      })),
      transform: Transform::from_xyz(-1.5, 0.0, 0.0),
      material: materials.add(LineMaterial {
          color: Color::GREEN,
      }),
      ..default()
  });

  // Spawn a line strip that goes from point to point
  commands.spawn(MaterialMeshBundle {
      mesh: meshes.add(Mesh::from(LineStrip {
          points: vec![
              Vec3::ZERO,
              Vec3::new(1.0, 1.0, 0.0),
              Vec3::new(1.0, 0.0, 0.0),
          ],
      })),
      transform: Transform::from_xyz(0.5, 0.0, 0.0),
      material: materials.add(LineMaterial { color: Color::BLUE }),
      ..default()
  });
}

#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "050ce6ac-080a-4d8c-b6b5-b5bab7560d8f"]
struct LineMaterial {
  #[uniform(0)]
  color: Color,
}

impl Material for LineMaterial {
  fn fragment_shader() -> ShaderRef {
      "shaders/line_material.wgsl".into()
  }

  fn specialize(
      _pipeline: &MaterialPipeline<Self>,
      descriptor: &mut RenderPipelineDescriptor,
      _layout: &MeshVertexBufferLayout,
      _key: MaterialPipelineKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
      // This is the important part to tell bevy to render this material as a line between vertices
      descriptor.primitive.polygon_mode = PolygonMode::Line;
      Ok(())
  }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
  pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
  fn from(line: LineList) -> Self {
      // This tells wgpu that the positions are list of lines
      // where every pair is a start and end point
      let mut mesh = Mesh::new(PrimitiveTopology::LineList);

      let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();
      mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
      mesh
  }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct LineStrip {
  pub points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
  fn from(line: LineStrip) -> Self {
      // This tells wgpu that the positions are a list of points
      // where a line will be drawn between each consecutive point
      let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

      mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line.points);
      mesh
  }
}

