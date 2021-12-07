VSH          i  #include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

struct xlatMtlMain_out
{
    float4 gl_Position [[position]];
};

struct xlatMtlMain_in
{
    float4 a_position [[attribute(0)]];
};

vertex xlatMtlMain_out xlatMtlMain(xlatMtlMain_in in [[stage_in]])
{
    xlatMtlMain_out out = {};
    out.gl_Position = in.a_position;
    return out;
}

    