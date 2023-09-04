#import bevy_pbr::mesh_view_bindings globals
#import bevy_pbr::mesh_view_bindings view
#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_bindings

#import bevy_pbr::pbr_types PbrInput
#import bevy_pbr::pbr_types STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT
#import bevy_pbr::pbr_types as pbr_types
#import bevy_pbr::pbr_types STANDARD_MATERIAL_FLAGS_UNLIT_BIT
#import bevy_pbr::pbr_bindings as pbr_bindings
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_core_pipeline::tonemapping tone_mapping
#import bevy_pbr::pbr_functions as fns

struct CustomMaterial {
    color: vec4<f32>,
};

struct XScrollSpeed {
    value: f32,
};

struct YScrollSpeed {
    value: f32,
};

struct Scale {
    value: f32,
};

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;
@group(1) @binding(2)
var<uniform> material: CustomMaterial;
@group(1) @binding(3)
var<uniform> x_scroll_speed: XScrollSpeed;
@group(1) @binding(4)
var<uniform> y_scroll_speed: YScrollSpeed;
@group(1) @binding(5)
var<uniform> scale: Scale;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    mesh: MeshVertexOutput
) -> @location(0) vec4<f32> {
    let x_speed = x_scroll_speed.value;
    let y_speed = y_scroll_speed.value * 10.;
    let scale_value = scale.value * 0.1;

    let uv = vec2((globals.time * x_speed + mesh.uv.x / scale_value) % 1.0, (globals.time * y_speed + mesh.uv.y / scale_value) % 1.0);
    var texture_sample = textureSample(texture, texture_sampler, uv);

    var pbr_input: fns::PbrInput = fns::pbr_input_new();

    pbr_input.material.base_color = texture_sample;
    pbr_input.material.base_color.a = 0.2;
//    pbr_input.material.alpha_cutoff = 0.5;
    pbr_input.material.flags = pbr_types::STANDARD_MATERIAL_FLAGS_ALPHA_MODE_BLEND;

    return fns::pbr(pbr_input);
}

