#import bevy_pbr::mesh_view_bindings globals
#import bevy_pbr::mesh_view_bindings  view
#import bevy_pbr::mesh_vertex_output  MeshVertexOutput
#import bevy_pbr::utils               coords_to_viewport_uv

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

fn get_texture_sample(coords: vec2<f32>) -> vec3<f32> {
    let num_repeats = 10.;
    let repeated_coords = (coords % (1./num_repeats)) * num_repeats;
    return textureSample(texture, texture_sampler, repeated_coords).rgb;
}


@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let textureSize: vec2<u32> = textureDimensions(texture);

    var texCoord: vec2<f32> = mesh.uv;
    texCoord.x = texCoord.x - floor(texCoord.x);

    let y = texCoord.y - (globals.time / 90.0);
    texCoord.y = y - floor(y) ;

    var color: vec4<f32> = textureSample(texture, texture_sampler, texCoord);
    let texture = get_texture_sample(texCoord);
    return vec4(texture, 0.);
}
