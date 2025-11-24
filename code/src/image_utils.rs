use std::slice;

/// Converts unpacked BayerRG12 raw data (12-bit values in 16-bit words, RGGB pattern) to RGB8 (8-bit per channel).
///
/// # Arguments
/// * `raw_data` - Byte slice containing the unpacked BayerRG12 data (length must be width * height * 2).
/// * `width` - Image width in pixels.
/// * `height` - Image height in pixels.
///
/// # Panics
/// Panics if the input data length does not match the expected size (width * height * 2).
///
/// # Notes
/// - Assumes data is big-endian or little-endian u16? Rust slice assumes host endian; for USB Vision, typically little-endian.
///   If needed, use byteorder crate for explicit endianness (not included here).
/// - Uses a simple bilinear interpolation debayering algorithm.
/// - Scales 12-bit values to 8-bit by right-shifting 4 bits (simple truncation).
/// - Output is a flat Vec<u8> with RGB pixels in row-major order (r, g, b per pixel).
/// - Based on your buffer size (792064 = 728 * 544 * 2), this handles unpacked 16-bit format.
/// - If data is packed (1.5 bpp), adjust by slicing only the first (width * height * 3 / 2) bytes and use the previous version.
pub fn bayer_rg12_to_rgb8(raw_data: &[u8], width: usize, height: usize) -> Vec<u8> {
    let expected_len = width * height * 2;
    assert_eq!(raw_data.len(), expected_len, "Input data length mismatch: expected unpacked 16-bit BayerRG12");

    // Step 1: Unpack 16-bit Bayer values into a flat u16 vector (RGGB pattern).
    // Assumes little-endian (common for USB); if big-endian, swap bytes or use byteorder.
    let bayer: Vec<u16> = unsafe {
        std::slice::from_raw_parts(raw_data.as_ptr() as *const u16, width * height)
    }.iter()
        .map(|&v| v & 0x0FFF)  // Mask to 12 bits if high bits have junk, but typically they are 0
        .collect();

    // Step 2: Debayer to RGB using bilinear interpolation.
    let mut rgb = vec![0u8; width * height * 3];
    for y in 0..height {
        for x in 0..width {
            let bayer_idx = y * width + x;
            let is_r_pos = (y % 2 == 0) && (x % 2 == 0);
            let is_b_pos = (y % 2 == 1) && (x % 2 == 1);
            // G pos is the else case.

            let mut red: u16 = 0;
            let mut green: u16 = 0;
            let mut blue: u16 = 0;

            if is_r_pos {
                // R position (even, even): R = raw, G = avg adjacent G, B = avg diagonal B
                red = bayer[bayer_idx];
                // Avg G (left, right, up, down)
                let mut sum_g = 0u32;
                let mut cnt_g = 0u32;
                if x > 0 { sum_g += bayer[y * width + x - 1] as u32; cnt_g += 1; }
                if x + 1 < width { sum_g += bayer[y * width + x + 1] as u32; cnt_g += 1; }
                if y > 0 { sum_g += bayer[(y - 1) * width + x] as u32; cnt_g += 1; }
                if y + 1 < height { sum_g += bayer[(y + 1) * width + x] as u32; cnt_g += 1; }
                green = if cnt_g > 0 { (sum_g / cnt_g) as u16 } else { 0 };
                // Avg B (diagonals: up-left, up-right, down-left, down-right; these are B positions for R)
                let mut sum_b = 0u32;
                let mut cnt_b = 0u32;
                for dy in [-1i32, 1i32] {
                    for dx in [-1i32, 1i32] {
                        let nx = (x as i32) + dx;
                        let ny = (y as i32) + dy;
                        if nx >= 0 && nx < (width as i32) && ny >= 0 && ny < (height as i32) {
                            sum_b += bayer[(ny as usize) * width + (nx as usize)] as u32;
                            cnt_b += 1;
                        }
                    }
                }
                blue = if cnt_b > 0 { (sum_b / cnt_b) as u16 } else { 0 };
            } else if is_b_pos {
                // B position (odd, odd): B = raw, G = avg adjacent G, R = avg diagonal R
                blue = bayer[bayer_idx];
                // Avg G (same as above)
                let mut sum_g = 0u32;
                let mut cnt_g = 0u32;
                if x > 0 { sum_g += bayer[y * width + x - 1] as u32; cnt_g += 1; }
                if x + 1 < width { sum_g += bayer[y * width + x + 1] as u32; cnt_g += 1; }
                if y > 0 { sum_g += bayer[(y - 1) * width + x] as u32; cnt_g += 1; }
                if y + 1 < height { sum_g += bayer[(y + 1) * width + x] as u32; cnt_g += 1; }
                green = if cnt_g > 0 { (sum_g / cnt_g) as u16 } else { 0 };
                // Avg R (diagonals: same positions, which are R for B)
                let mut sum_r = 0u32;
                let mut cnt_r = 0u32;
                for dy in [-1i32, 1i32] {
                    for dx in [-1i32, 1i32] {
                        let nx = (x as i32) + dx;
                        let ny = (y as i32) + dy;
                        if nx >= 0 && nx < (width as i32) && ny >= 0 && ny < (height as i32) {
                            sum_r += bayer[(ny as usize) * width + (nx as usize)] as u32;
                            cnt_r += 1;
                        }
                    }
                }
                red = if cnt_r > 0 { (sum_r / cnt_r) as u16 } else { 0 };
            } else {
                // G position
                green = bayer[bayer_idx];
                let is_horiz_g = (x % 2 == 1) && (y % 2 == 0);  // Odd x, even y: horizontal G line
                if is_horiz_g {
                    // R from horizontal neighbors (even x)
                    let mut sum_r = 0u32;
                    let mut cnt_r = 0u32;
                    if x > 0 { sum_r += bayer[y * width + x - 1] as u32; cnt_r += 1; }
                    if x + 1 < width { sum_r += bayer[y * width + x + 1] as u32; cnt_r += 1; }
                    red = if cnt_r > 0 { (sum_r / cnt_r) as u16 } else { 0 };
                    // B from vertical neighbors (odd y, same x = odd; but for horiz G, vertical are B? Wait, vertical neighbors are odd y, odd x? No.
                    // Correction: for horiz G (y even, x odd), up/down (y odd, x odd) are B positions.
                    let mut sum_b = 0u32;
                    let mut cnt_b = 0u32;
                    if y > 0 { sum_b += bayer[(y - 1) * width + x] as u32; cnt_b += 1; }
                    if y + 1 < height { sum_b += bayer[(y + 1) * width + x] as u32; cnt_b += 1; }
                    blue = if cnt_b > 0 { (sum_b / cnt_b) as u16 } else { 0 };
                } else {
                    // Vertical G (even x, odd y): B from horizontal neighbors (odd x), R from vertical (even y, even x)
                    let mut sum_b = 0u32;
                    let mut cnt_b = 0u32;
                    if x > 0 { sum_b += bayer[y * width + x - 1] as u32; cnt_b += 1; }
                    if x + 1 < width { sum_b += bayer[y * width + x + 1] as u32; cnt_b += 1; }
                    blue = if cnt_b > 0 { (sum_b / cnt_b) as u16 } else { 0 };
                    let mut sum_r = 0u32;
                    let mut cnt_r = 0u32;
                    if y > 0 { sum_r += bayer[(y - 1) * width + x] as u32; cnt_r += 1; }
                    if y + 1 < height { sum_r += bayer[(y + 1) * width + x] as u32; cnt_r += 1; }
                    red = if cnt_r > 0 { (sum_r / cnt_r) as u16 } else { 0 };
                }
            }

            // Scale to 8-bit and store (row-major RGB)
            let rgb_idx = (y * width + x) * 3;
            rgb[rgb_idx] = (red >> 4) as u8;
            rgb[rgb_idx + 1] = (green >> 4) as u8;
            rgb[rgb_idx + 2] = (blue >> 4) as u8;
        }
    }

    rgb
}

pub fn save_rgb8_image(p0: &String, p1: &Vec<u8>, p2: usize, p3: usize) -> Result<(), std::io::Error> {
    use image::{ImageBuffer, Rgb};
    let img_buffer: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(p2 as u32, p3 as u32, p1.clone())
        .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create image buffer"))?;
    img_buffer.save(p0).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}
