#include <metal_stdlib>
using namespace metal;

kernel void add_arrays(device float *in1 [[buffer(0)]],
                       device float *in2 [[buffer(1)]],
                       device float *out [[buffer(2)]],
                       uint id [[thread_position_in_grid]]) {
    out[id] = in1[id] + in2[id];
}