#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var material_colour_texture: texture_2d<f32>;
@group(2) @binding(1) var material_colour_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return textureSample(material_colour_texture, material_colour_sampler, mesh.uv);
}
