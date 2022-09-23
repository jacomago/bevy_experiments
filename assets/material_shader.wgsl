
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // return material.color * textureSample(base_color_texture, base_color_sampler, uv);
    var alpha = ( 1.0) / 2.0;
    return return vec4<f32>(0.3, 0.2, 0.1, 1.0);}