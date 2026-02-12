#include <metal_stdlib>
using namespace metal;

struct QuadInstance {
    float4 bounds;    // x, y, width, height
    float4 color;     // r, g, b, a
};

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut vertex_main(
    uint vertex_id [[vertex_id]],
    uint instance_id [[instance_id]],
    constant float2 *vertices [[buffer(0)]],
    constant QuadInstance *instances [[buffer(1)]],
    constant float2 &viewport_size [[buffer(2)]]
) {
    float2 unit_pos = vertices[vertex_id];
    QuadInstance inst = instances[instance_id];

    // Scale unit quad to instance bounds
    float2 pos = inst.bounds.xy + unit_pos * inst.bounds.zw;

    // Device pixels → clip space [-1, 1]
    float2 clip = (pos / viewport_size) * 2.0 - 1.0;
    clip.y = -clip.y;  // Flip Y for Metal's coordinate system

    VertexOut out;
    out.position = float4(clip, 0.0, 1.0);
    out.color = inst.color;
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]]) {
    return in.color;
}
