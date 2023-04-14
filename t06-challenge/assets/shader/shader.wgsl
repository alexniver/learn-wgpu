struct VertexInput { 
    @location(0) pos: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
};

struct VertexOutput { 
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

struct InstanceInput {
    @location(5) mat_0: vec4<f32>,
    @location(6) mat_1: vec4<f32>,
    @location(7) mat_2: vec4<f32>,
    @location(8) mat_3: vec4<f32>,
};

@group(1)@binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(in: VertexInput, instance: InstanceInput) -> VertexOutput {
    let model = mat4x4<f32>(instance.mat_0, instance.mat_1, instance.mat_2, instance.mat_3);
    var out: VertexOutput;
    out.tex_coord = in.tex_coord;
    out.clip_pos = camera.view_proj * model * vec4<f32>(in.pos, 1.0);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coord);
}
