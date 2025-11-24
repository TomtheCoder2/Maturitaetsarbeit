#include <metal_stdlib>
using namespace metal;

// Struct for globals (passed as buffer(2))
struct Globals {
    uint width;
    uint height;
};

// Helper: Get Bayer value at (x,y) with edge clamping
uint16_t get_bayer(device const uint16_t* bayer_buffer, uint x, uint y, uint width, uint height) {
    if (x >= width || y >= height) return 0u;
    uint idx = y * width + x;
    // Bounds check
    if (idx >= width * height) return 0u;
    return bayer_buffer[idx];
}

// Avg orthogonal neighbors (for G from R/B positions)
uint16_t avg_ortho(device const uint16_t* bayer_buffer, uint x, uint y, uint width, uint height) {
    uint32_t sum = 0u;
    uint cnt = 0u;
    if (x > 0u) { sum += get_bayer(bayer_buffer, x - 1, y, width, height); cnt++; }
    if (x + 1 < width) { sum += get_bayer(bayer_buffer, x + 1, y, width, height); cnt++; }
    if (y > 0u) { sum += get_bayer(bayer_buffer, x, y - 1, width, height); cnt++; }
    if (y + 1 < height) { sum += get_bayer(bayer_buffer, x, y + 1, width, height); cnt++; }
    return cnt > 0u ? static_cast<uint16_t>(sum / cnt) : 0u;
}

// Avg diagonals (for B from R or R from B)
uint16_t avg_diagonals(device const uint16_t* bayer_buffer, uint x, uint y, uint width, uint height) {
    uint32_t sum_ul_dr = 0u, sum_ur_dl = 0u;
    uint cnt1 = 0u, cnt2 = 0u;

    // UL & DR pair
    if (x >= 1u && y >= 1u) { uint16_t v = get_bayer(bayer_buffer, x - 1, y - 1, width, height); sum_ul_dr += v; cnt1++; }
    if (x + 1 < width && y + 1 < height) { uint16_t v = get_bayer(bayer_buffer, x + 1, y + 1, width, height); sum_ul_dr += v; cnt1++; }
    // UR & DL pair
    if (x + 1 < width && y >= 1u) { uint16_t v = get_bayer(bayer_buffer, x + 1, y - 1, width, height); sum_ur_dl += v; cnt2++; }
    if (x >= 1u && y + 1 < height) { uint16_t v = get_bayer(bayer_buffer, x - 1, y + 1, width, height); sum_ur_dl += v; cnt2++; }

    uint16_t avg1 = cnt1 > 0u ? static_cast<uint16_t>(sum_ul_dr / cnt1) : 0u;
    uint16_t avg2 = cnt2 > 0u ? static_cast<uint16_t>(sum_ur_dl / cnt2) : 0u;
    return (avg1 + avg2) / 2u;
}

// Avg horizontal for R positions (even x, for horiz G)
uint16_t avg_horiz_r(device const uint16_t* bayer_buffer, uint x, uint y, uint width, uint height) {
    uint32_t sum = 0u; uint cnt = 0u;
    if (x > 0u) { sum += get_bayer(bayer_buffer, x - 1, y, width, height); cnt++; }
    if (x + 1 < width) { sum += get_bayer(bayer_buffer, x + 1, y, width, height); cnt++; }
    return cnt > 0u ? static_cast<uint16_t>(sum / cnt) : 0u;
}

// Avg horizontal for B positions (odd x, for vert G)
uint16_t avg_horiz_b(device const uint16_t* bayer_buffer, uint x, uint y, uint width, uint height) {
    uint32_t sum = 0u; uint cnt = 0u;
    if (x > 0u) { sum += get_bayer(bayer_buffer, x - 1, y, width, height); cnt++; }
    if (x + 1 < width) { sum += get_bayer(bayer_buffer, x + 1, y, width, height); cnt++; }
    return cnt > 0u ? static_cast<uint16_t>(sum / cnt) : 0u;
}

// Avg vertical for R positions (even y, for vert G)
uint16_t avg_vert_r(device const uint16_t* bayer_buffer, uint x, uint y, uint width, uint height) {
    uint32_t sum = 0u; uint cnt = 0u;
    if (y > 0u) { sum += get_bayer(bayer_buffer, x, y - 1, width, height); cnt++; }
    if (y + 1 < height) { sum += get_bayer(bayer_buffer, x, y + 1, width, height); cnt++; }
    return cnt > 0u ? static_cast<uint16_t>(sum / cnt) : 0u;
}

// Avg vertical for B positions (odd y, for horiz G)
uint16_t avg_vert_b(device const uint16_t* bayer_buffer, uint x, uint y, uint width, uint height) {
    uint32_t sum = 0u; uint cnt = 0u;
    if (y > 0u) { sum += get_bayer(bayer_buffer, x, y - 1, width, height); cnt++; }
    if (y + 1 < height) { sum += get_bayer(bayer_buffer, x, y + 1, width, height); cnt++; }
    return cnt > 0u ? static_cast<uint16_t>(sum / cnt) : 0u;
}

// Pixel processing (called from kernel)
void process_pixel(device const uint16_t* bayer_buffer, device uint8_t* rgb_buffer,
                   constant Globals& globals, uint x, uint y) {
    uint width = globals.width;
    uint height = globals.height;
    uint idx = y * width + x;
    if (x >= width || y >= height) return;

    bool is_r = (y % 2u == 0u) && (x % 2u == 0u);
    bool is_b = (y % 2u == 1u) && (x % 2u == 1u);
    bool is_g_h = (y % 2u == 0u) && (x % 2u == 1u);  // Horizontal G
    bool is_g_v = (y % 2u == 1u) && (x % 2u == 0u);  // Vertical G

    uint16_t r_val = 0u, g_val = 0u, b_val = 0u;
    if (is_r) {
        r_val = get_bayer(bayer_buffer, x, y, width, height);
        g_val = avg_ortho(bayer_buffer, x, y, width, height);
        b_val = avg_diagonals(bayer_buffer, x, y, width, height);
    } else if (is_b) {
        b_val = get_bayer(bayer_buffer, x, y, width, height);
        g_val = avg_ortho(bayer_buffer, x, y, width, height);
        r_val = avg_diagonals(bayer_buffer, x, y, width, height);
    } else if (is_g_h) {
        g_val = get_bayer(bayer_buffer, x, y, width, height);
        r_val = avg_horiz_r(bayer_buffer, x, y, width, height);
        b_val = avg_vert_b(bayer_buffer, x, y, width, height);
    } else {  // is_g_v
        g_val = get_bayer(bayer_buffer, x, y, width, height);
        b_val = avg_horiz_b(bayer_buffer, x, y, width, height);
        r_val = avg_vert_r(bayer_buffer, x, y, width, height);
    }

    // Scale 12-bit to 8-bit with resolved min (explicit cast to uint)
    uint shifted_r = static_cast<uint>(r_val >> 4u);
    uint8_t r8 = static_cast<uint8_t>(min(shifted_r, 255u));
    uint shifted_g = static_cast<uint>(g_val >> 4u);
    uint8_t g8 = static_cast<uint8_t>(min(shifted_g, 255u));
    uint shifted_b = static_cast<uint>(b_val >> 4u);
    uint8_t b8 = static_cast<uint8_t>(min(shifted_b, 255u));

    // Write RGB (row-major)
    uint out_idx = idx * 3u;
    rgb_buffer[out_idx] = r8;
    rgb_buffer[out_idx + 1] = g8;
    rgb_buffer[out_idx + 2] = b8;
}

// Kernel: Launched by Rust
kernel void debayer_kernel(device const uint16_t* bayer_buffer [[buffer(0)]],
                           device uint8_t* rgb_buffer [[buffer(1)]],
                           constant Globals& globals [[buffer(2)]],
                           uint2 tid [[thread_position_in_grid]],
                           uint2 dtid [[threads_per_threadgroup]]) {
    process_pixel(bayer_buffer, rgb_buffer, globals, tid.x, tid.y);
}