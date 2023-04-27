#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::fog
// #import bevy_pbr::pbr_functions
#import bevy_pbr::pbr_ambient

#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif


@group(1) @binding(0)
var albedo: texture_2d_array<f32>;
@group(1) @binding(1)
var albedo_sampler: sampler;
@group(1) @binding(2)
var normal: texture_2d_array<f32>;
@group(1) @binding(3)
var normal_sampler: sampler;


fn alpha_discard(material: StandardMaterial, output_color: vec4<f32>) -> vec4<f32> {
    var color = output_color;
    let alpha_mode = material.flags & STANDARD_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS;
    if alpha_mode == STANDARD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE {
        // NOTE: If rendering as opaque, alpha should be ignored so set to 1.0
        color.a = 1.0;
    } else if alpha_mode == STANDARD_MATERIAL_FLAGS_ALPHA_MODE_MASK {
        if color.a >= material.alpha_cutoff {
            // NOTE: If rendering as masked alpha and >= the cutoff, render as fully opaque
            color.a = 1.0;
        } else {
            // NOTE: output_color.a < in.material.alpha_cutoff should not is not rendered
            // NOTE: This and any other discards mean that early-z testing cannot be done!
            discard;
        }
    }
    return color;
}


struct PbrInput {
  material: StandardMaterial,
  occlusion: f32,
  frag_coord: vec4<f32>,
  world_position: vec4<f32>,
  // Normalized world normal used for shadow mapping as normal-mapping is not used for shadow
  // mapping
  world_normal: vec3<f32>,
  // Normalized normal-mapped world normal used for lighting
  N: vec3<f32>,
  // Normalized view vector in world space, pointing from the fragment world position toward the
  // view world position
  V: vec3<f32>,
  is_orthographic: bool,
  flags: u32,
};

// Creates a PbrInput with default values
fn pbr_input_new() -> PbrInput {
  var pbr_input: PbrInput;

  pbr_input.material = standard_material_new();
  pbr_input.occlusion = 1.0;

  pbr_input.frag_coord = vec4<f32>(0.0, 0.0, 0.0, 1.0);
  pbr_input.world_position = vec4<f32>(0.0, 0.0, 0.0, 1.0);
  pbr_input.world_normal = vec3<f32>(0.0, 0.0, 1.0);

  pbr_input.is_orthographic = false;

  pbr_input.N = vec3<f32>(0.0, 0.0, 1.0);
  pbr_input.V = vec3<f32>(1.0, 0.0, 0.0);

  pbr_input.flags = 0u;

  return pbr_input;
}

fn prepare_world_normal(
    world_normal: vec3<f32>,
    double_sided: bool,
    is_front: bool,
) -> vec3<f32> {
  var output: vec3<f32> = world_normal;
#ifndef VERTEX_TANGENTS
#ifndef STANDARDMATERIAL_NORMAL_MAP
  // NOTE: When NOT using normal-mapping, if looking at the back face of a double-sided
  // material, the normal needs to be inverted. This is a branchless version of that.
  output = (f32(!double_sided || is_front) * 2.0 - 1.0) * output;
#endif
#endif
  return output;
}

// NOTE: Correctly calculates the view vector depending on whether
// the projection is orthographic or perspective.
fn calculate_view(
    world_position: vec4<f32>,
    is_orthographic: bool,
) -> vec3<f32> {
  var V: vec3<f32>;
  if is_orthographic {
    // Orthographic view vector
    V = normalize(vec3<f32>(view.view_proj[0].z, view.view_proj[1].z, view.view_proj[2].z));
  } else {
    // Only valid for a perpective projection
    V = normalize(view.world_position.xyz - world_position.xyz);
  }
  return V;
}

#ifndef NORMAL_PREPASS
fn pbr(
    in: PbrInput,
) -> vec4<f32> {
    var output_color: vec4<f32> = in.material.base_color;

    // TODO use .a for exposure compensation in HDR
    let emissive = in.material.emissive;

    // calculate non-linear roughness from linear perceptualRoughness
    let metallic = in.material.metallic;
    let perceptual_roughness = in.material.perceptual_roughness;
    let roughness = perceptualRoughnessToRoughness(perceptual_roughness);

    let occlusion = in.occlusion;

    output_color = alpha_discard(in.material, output_color);

    // Neubelt and Pettineo 2013, "Crafting a Next-gen Material Pipeline for The Order: 1886"
    let NdotV = max(dot(in.N, in.V), 0.0001);

    // Remapping [0,1] reflectance to F0
    // See https://google.github.io/filament/Filament.html#materialsystem/parameterization/remapping
    let reflectance = in.material.reflectance;
    let F0 = 0.16 * reflectance * reflectance * (1.0 - metallic) + output_color.rgb * metallic;

    // Diffuse strength inversely related to metallicity
    let diffuse_color = output_color.rgb * (1.0 - metallic);

    let R = reflect(-in.V, in.N);

    let f_ab = F_AB(perceptual_roughness, NdotV);

    var direct_light: vec3<f32> = vec3<f32>(0.0);

    let view_z = dot(vec4<f32>(
        view.inverse_view[0].z,
        view.inverse_view[1].z,
        view.inverse_view[2].z,
        view.inverse_view[3].z
    ), in.world_position);
    let cluster_index = fragment_cluster_index(in.frag_coord.xy, view_z, in.is_orthographic);
    let offset_and_counts = unpack_offset_and_counts(cluster_index);

    // Point lights (direct)
    for (var i: u32 = offset_and_counts[0]; i < offset_and_counts[0] + offset_and_counts[1]; i = i + 1u) {
        let light_id = get_light_id(i);
        var shadow: f32 = 1.0;
        if ((in.flags & MESH_FLAGS_SHADOW_RECEIVER_BIT) != 0u
                && (point_lights.data[light_id].flags & POINT_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
            shadow = fetch_point_shadow(light_id, in.world_position, in.world_normal);
        }
        let light_contrib = point_light(in.world_position.xyz, light_id, roughness, NdotV, in.N, in.V, R, F0, f_ab, diffuse_color);
        direct_light += light_contrib * shadow;
    }

    // Spot lights (direct)
    for (var i: u32 = offset_and_counts[0] + offset_and_counts[1]; i < offset_and_counts[0] + offset_and_counts[1] + offset_and_counts[2]; i = i + 1u) {
        let light_id = get_light_id(i);
        var shadow: f32 = 1.0;
        if ((in.flags & MESH_FLAGS_SHADOW_RECEIVER_BIT) != 0u
                && (point_lights.data[light_id].flags & POINT_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
            shadow = fetch_spot_shadow(light_id, in.world_position, in.world_normal);
        }
        let light_contrib = spot_light(in.world_position.xyz, light_id, roughness, NdotV, in.N, in.V, R, F0, f_ab, diffuse_color);
        direct_light += light_contrib * shadow;
    }

    // Directional lights (direct)
    let n_directional_lights = lights.n_directional_lights;
    for (var i: u32 = 0u; i < n_directional_lights; i = i + 1u) {
        var shadow: f32 = 1.0;
        if ((in.flags & MESH_FLAGS_SHADOW_RECEIVER_BIT) != 0u
                && (lights.directional_lights[i].flags & DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
            shadow = fetch_directional_shadow(i, in.world_position, in.world_normal, view_z);
        }
        var light_contrib = directional_light(i, roughness, NdotV, in.N, in.V, R, F0, f_ab, diffuse_color);
#ifdef DIRECTIONAL_LIGHT_SHADOW_MAP_DEBUG_CASCADES
        light_contrib = cascade_debug_visualization(light_contrib, i, view_z);
#endif
        direct_light += light_contrib * shadow;
    }

    // Ambient light (indirect)
    var indirect_light = ambient_light(in.world_position, in.N, in.V, NdotV, diffuse_color, F0, perceptual_roughness, occlusion);

    // Environment map light (indirect)
#ifdef ENVIRONMENT_MAP
    let environment_light = environment_map_light(perceptual_roughness, roughness, diffuse_color, NdotV, f_ab, in.N, R, F0);
    indirect_light += (environment_light.diffuse * occlusion) + environment_light.specular;
#endif

    let emissive_light = emissive.rgb * output_color.a;

    // Total light
    output_color = vec4<f32>(
        direct_light + indirect_light + emissive_light,
        output_color.a
    );

    output_color = cluster_debug_visualization(
        output_color,
        view_z,
        in.is_orthographic,
        offset_and_counts,
        cluster_index,
    );

    return output_color;
}
#endif // NORMAL_PREPASS





struct Vertex {
  @location(0) position: vec3<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) voxel_weight: vec4<f32>,
  @location(3) voxel_type_1: vec4<u32>,
};

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) world_position: vec4<f32>,
  @location(1) world_normal: vec3<f32>,
  @location(2) voxel_weight: vec4<f32>,
  @location(3) voxel_type_1: vec4<u32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
  var out: VertexOutput;
  out.world_position = mesh_position_local_to_world(mesh.model, vec4<f32>(vertex.position, 1.0));
  out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
  out.world_normal = vertex.normal;

  out.voxel_weight = vertex.voxel_weight;
  out.voxel_type_1 = vertex.voxel_type_1;
  return out;
}

struct FragmentInput {
  // @builtin(position) frag_coord: vec4<f32>,
  @builtin(front_facing) is_front: bool,
  @builtin(position) frag_coord: vec4<f32>,

  @location(0) world_position: vec4<f32>,
  @location(1) world_normal: vec3<f32>,
  @location(2) voxel_weight: vec4<f32>,
  @location(3) voxel_type_1: vec4<u32>,

  // #import bevy_pbr::mesh_vertex_output
};


struct Triplanar {
  w: vec3<f32>,
  uv_x: vec2<f32>,
  uv_y: vec2<f32>,
  uv_z: vec2<f32>,
}

fn sample_normal_map(uv: vec2<f32>, material_type: u32) -> vec3<f32> {

  
  var normal = textureSample(normal, normal_sampler, uv, i32(material_type)).rgb;
  normal = normal * 2.0 - 1.0;
  return normalize(normal);
  // return vec3<f32>(0.0);
}

fn triplanar_normal_to_world(
  material_type: u32, 
  world_normal: vec3<f32>, 
  triplanar: Triplanar,
) -> vec3<f32> {

  var normal_x = sample_normal_map(triplanar.uv_x, material_type);
  var normal_y = sample_normal_map(triplanar.uv_y, material_type);
  var normal_z = sample_normal_map(triplanar.uv_z, material_type);

  normal_x = vec3(normal_x.xy + world_normal.yz, abs(normal_x.z) * world_normal.x);
  normal_y = vec3(normal_y.xy + world_normal.zx, abs(normal_y.z) * world_normal.y);
  normal_z = vec3(normal_z.xy + world_normal.xy, abs(normal_z.z) * world_normal.z);

  return normalize(
    normal_x.zxy * triplanar.w.x +
    normal_y.yzx * triplanar.w.y +
    normal_z.xyz * triplanar.w.z
  );
  
  // return vec3<f32>(0.0);
}

fn triplanar_normal_to_world_splatted(
  material_weights: vec4<f32>,
  world_normal: vec3<f32>,
  material_types: vec4<u32>,
  triplanar: Triplanar,
) -> vec3<f32> {

  var sum = vec3(0.0);
  if material_weights.x > 0.0 {
    sum += material_weights.x * triplanar_normal_to_world(material_types.x, world_normal, triplanar);
  }
  if material_weights.y > 0.0 {
    sum += material_weights.x * triplanar_normal_to_world(material_types.y, world_normal, triplanar);
  }
  if material_weights.z > 0.0 {
    sum += material_weights.x * triplanar_normal_to_world(material_types.z, world_normal, triplanar);
  }
  if material_weights.w > 0.0 {
    sum += material_weights.x * triplanar_normal_to_world(material_types.w, world_normal, triplanar);
  }

  // return vec3<f32>(0.0);
  return normalize(sum);
}







@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
  var zy = input.world_position.zy % 1.0;
  if zy.x < 0.0 {
    zy.x += 1.0;
  }
  if zy.y < 0.0 {
    zy.y += 1.0;
  }

  var xz = input.world_position.xz % 1.0;
  if xz.x < 0.0 {
    xz.x += 1.0;
  }
  if xz.y < 0.0 {
    xz.y += 1.0;
  }

  var xy = input.world_position.xy % 1.0;
  if xy.x < 0.0 {
    xy.x += 1.0;
  }
  if xy.y < 0.0 {
    xy.y += 1.0;
  }

  var dx0 = textureSample(albedo, albedo_sampler, zy, i32(input.voxel_type_1.x));
  var dy0 = textureSample(albedo, albedo_sampler, xz, i32(input.voxel_type_1.x));
  var dz0 = textureSample(albedo, albedo_sampler, xy, i32(input.voxel_type_1.x));

  var dx1 = textureSample(albedo, albedo_sampler, zy, i32(input.voxel_type_1.y));
  var dy1 = textureSample(albedo, albedo_sampler, xz, i32(input.voxel_type_1.y));
  var dz1 = textureSample(albedo, albedo_sampler, xy, i32(input.voxel_type_1.y));

  var dx2 = textureSample(albedo, albedo_sampler, zy, i32(input.voxel_type_1.z));
  var dy2 = textureSample(albedo, albedo_sampler, xz, i32(input.voxel_type_1.z));
  var dz2 = textureSample(albedo, albedo_sampler, xy, i32(input.voxel_type_1.z));

  var dx3 = textureSample(albedo, albedo_sampler, zy, i32(input.voxel_type_1.w));
  var dy3 = textureSample(albedo, albedo_sampler, xz, i32(input.voxel_type_1.w));
  var dz3 = textureSample(albedo, albedo_sampler, xy, i32(input.voxel_type_1.w));

  let filter_0 = 100;
  let filter_1 = 100;
  let filter_2 = 100;

  var index0 = 1.0;
  if i32(input.voxel_type_1.x) == filter_0 || i32(input.voxel_type_1.x) == filter_1
  || i32(input.voxel_type_1.x) == filter_2 {
    index0 = 0.0;
  }
  var index1 = 1.0;
  if i32(input.voxel_type_1.y) == filter_0 || i32(input.voxel_type_1.y) == filter_1
  || i32(input.voxel_type_1.y) == filter_2 {
    index1 = 0.0;
  }
  var index2 = 1.0;
  if i32(input.voxel_type_1.z) == filter_0 || i32(input.voxel_type_1.z) == filter_1
  || i32(input.voxel_type_1.z) == filter_2 {
    index2 = 0.0;
  }
  var index3 = 1.0;
  if i32(input.voxel_type_1.w) == filter_0 || i32(input.voxel_type_1.w) == filter_1
  || i32(input.voxel_type_1.w) == filter_2 {
    index3 = 0.0;
  }

  var vx = input.voxel_weight.x * index0;
  var vy = input.voxel_weight.y * index1;
  var vz = input.voxel_weight.z * index2;
  var vw = input.voxel_weight.w * index3;

  let dx = dx0 * vx + dx1 * vy +
    dx2 * vz + dx3 * vw;

  let dy = dy0 * vx + dy1 * vy +
    dy2 * vz + dy3 * vw;

  let dz = dz0 * vx + dz1 * vy +
    dz2 * vz + dz3 * vw;

  let dx_normal = dpdx(input.world_position);
  let dy_normal = dpdy(input.world_position);
  // let cross = cross(dx_normal, dy_normal); // Error in WebGPU
  // let normal = normalize(cross(dx_normal, dy_normal));
  let normal = input.world_normal;


  let sharpness = 10.0;
  var weights = pow(abs(normal.xyz), vec3<f32>(sharpness, sharpness, sharpness));
  weights = weights / (weights.x + weights.y + weights.z);

  var color = dx * weights.x + dy * weights.y + dz * weights.z;
  // return color;


  var pbr_input: PbrInput = pbr_input_new();
  pbr_input.material.base_color = pbr_input.material.base_color * color;
  pbr_input.frag_coord = input.frag_coord;
  pbr_input.world_position = input.world_position;
  pbr_input.world_normal = prepare_world_normal(
    input.world_normal,
    true,
    false,
  );

  pbr_input.is_orthographic = view.projection[3].w == 1.0;

  let sharpness_1 = 8.0;
  var weights_1 = pow(abs(input.world_normal), vec3(sharpness_1));
  weights_1 = weights_1 / (weights_1.x + weights_1.y + weights_1.z);

  let scale = 1.0;
  let uv_x = input.world_position.yz * scale;
  let uv_y = input.world_position.zx * scale;
  let uv_z = input.world_position.xy * scale;
  var triplanar = Triplanar(weights_1, uv_x, uv_y, uv_z);

  pbr_input.N = triplanar_normal_to_world_splatted(
    input.voxel_weight, 
    input.world_normal, 
    input.voxel_type_1, 
    triplanar
  );

  pbr_input.V = calculate_view(input.world_position, pbr_input.is_orthographic);

  return tone_mapping(pbr(pbr_input));

  // return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}




