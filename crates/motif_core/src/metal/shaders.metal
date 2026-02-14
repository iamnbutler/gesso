#include <metal_stdlib>
using namespace metal;

struct QuadInstance {
    float4 bounds;        // x, y, width, height
    float4 color;         // r, g, b, a (background)
    float4 border_color;  // r, g, b, a
    float4 border_widths; // top, right, bottom, left
};

struct VertexOut {
    float4 position [[position]];
    float4 color;
    float4 border_color;
    float4 border_widths;
    float2 quad_size;     // width, height in pixels
    float2 local_pos;     // position within quad in pixels
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
    out.border_color = inst.border_color;
    out.border_widths = inst.border_widths;
    out.quad_size = inst.bounds.zw;
    out.local_pos = unit_pos * inst.bounds.zw;
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]]) {
    // Check if pixel is in border region
    float2 pos = in.local_pos;
    float2 size = in.quad_size;

    // Distance from each edge
    float dist_top = pos.y;
    float dist_bottom = size.y - pos.y;
    float dist_left = pos.x;
    float dist_right = size.x - pos.x;

    // Border widths: top, right, bottom, left
    bool in_border = dist_top < in.border_widths.x ||
                     dist_right < in.border_widths.y ||
                     dist_bottom < in.border_widths.z ||
                     dist_left < in.border_widths.w;

    // Use border color if in border region and border has alpha
    if (in_border && in.border_color.a > 0.0) {
        return in.border_color;
    }
    return in.color;
}
