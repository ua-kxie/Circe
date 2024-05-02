#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}

struct UIMaterial {
    color: vec4<f32>,
};
@group(2) @binding(0) var<uniform> material: UIMaterial;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );
    out.clip_position[2] = 1.0;  // keep the z coordinate fixed to maintain rendered size of lines and points
    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return material.color;
}