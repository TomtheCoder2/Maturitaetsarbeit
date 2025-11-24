/// Converts BayerRG12p raw data (packed 12-bit, RGGB pattern) to RGB8 (8-bit per channel).
///
/// # Arguments
/// * `raw_data` - Byte slice containing the packed BayerRG12p data.
/// * `width` - Image width in pixels (must be even for standard packing).
/// * `height` - Image height in pixels.
/// * `reverse_bits` - If true, reverses nibble order in unpacking (for non-standard SDKs).
///
/// # Panics
/// Panics if dimensions invalid or data length mismatch.
///
/// # Notes
/// - Standard GenICam packing: 3 bytes/2 pixels (pix1: b0<<4 | b1>>4; pix2: (b1&0x0F)<<8 | b2).
/// - Clamps to 12-bit range (0-4095) to prevent overflow artifacts.
/// - Improved bilinear: Weighted diagonals (opposite ones averaged separately if needed) for less moiré.
/// - Debug: Prints first few unpacked pixels to console (remove for production).
/// - Output: Row-major RGB (r,g,b).
pub fn bayer_rg12p_to_rgb8(raw_data: &[u8], width: usize, height: usize, reverse_bits: bool) -> Vec<u8> {
    assert!(width % 2 == 0, "Width must be even");
    let num_pixels = width * height;
    let expected_len = (num_pixels * 3) / 2;
    assert_eq!(raw_data.len(), expected_len, "Data length mismatch: expected {} bytes", expected_len);

    // Step 1: Unpack with optional bit reversal.
    let mut bayer = vec![0u16; num_pixels];
    for y in 0..height {
        let row_bytes = (width * 3) / 2;
        let mut byte_idx = y * row_bytes;
        for x in (0..width).step_by(2) {
            if byte_idx + 2 >= raw_data.len() {
                panic!("Buffer overrun at y={}, x={}", y, x);
            }
            let b0 = raw_data[byte_idx] as u16;
            let b1 = raw_data[byte_idx + 1] as u16;
            let b2 = raw_data[byte_idx + 2] as u16;
            let (pix1, pix2) = if reverse_bits {
                // Alternative: Sometimes SDKs pack low bits first (pix1: (b1 & 0x0F)<<8 | b0; pix2: b2<<4 | (b1>>4))
                let pix1_alt = ((b1 & 0x0F) << 8) | b0;
                let pix2_alt = (b2 << 4) | (b1 >> 4);
                (pix1_alt, pix2_alt)
            } else {
                // Standard GenICam
                let pix1_std = (b0 << 4) | (b1 >> 4);
                let pix2_std = ((b1 & 0x0F) << 8) | b2;
                (pix1_std, pix2_std)
            };
            let val1 = pix1.clamp(0, 4095);  // Clamp to 12-bit
            let val2 = pix2.clamp(0, 4095);
            let idx1 = y * width + x;
            let idx2 = idx1 + 1;
            bayer[idx1] = val1;
            bayer[idx2] = val2;
            byte_idx += 3;

            // // Debug: Print first row's first few pixels (remove after testing)
            // if y == 0 && x < 4 {
            //     println!("Unpacked pixel ({},{}): {}, ({},{}): {}", x, y, val1, x+1, y, val2);
            // }
        }
    }

    // Step 2: Debayer with improved bilinear (weighted opposite diagonals for better color).
    let mut rgb = vec![0u8; num_pixels * 3];
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let is_r = (y % 2 == 0) && (x % 2 == 0);
            let is_b = (y % 2 == 1) && (x % 2 == 1);
            let is_g_h = !is_r && (x % 2 == 1);  // Horizontal G (even y, odd x)
            let is_g_v = !is_b && (x % 2 == 0);  // Vertical G (odd y, even x)

            let mut r: u16 = bayer[idx];
            let mut g: u16 = bayer[idx];
            let mut b: u16 = bayer[idx];

            if is_r {
                // R: r=raw, g=avg adj G, b=avg diag B (improve: avg UL/DR vs UR/DL if uneven)
                r = bayer[idx];
                g = avg_g_ortho(&bayer, x, y, width, height);
                b = avg_diag_b(&bayer, x, y, width, height, is_r);
            } else if is_b {
                // B: b=raw, g=avg adj G, r=avg diag R
                b = bayer[idx];
                g = avg_g_ortho(&bayer, x, y, width, height);
                r = avg_diag_b(&bayer, x, y, width, height, is_r);  // Reuse for R, swap logic
            } else if is_g_h {
                // Horiz G: g=raw, r=avg left/right R, b=avg up/down B
                g = bayer[idx];
                r = avg_horiz_even(&bayer, x, y, width, height);  // left/right: R
                b = avg_vert_odd(&bayer, x, y, width, height);    // up/down: B
            } else {  // is_g_v
                // Vert G: g=raw, b=avg left/right B, r=avg up/down R
                g = bayer[idx];
                b = avg_horiz_odd(&bayer, x, y, width, height);   // left/right: B
                r = avg_vert_even(&bayer, x, y, width, height);   // up/down: R
            }

            // Clamp and scale to 8-bit (>>4 truncates high 4 bits)
            let r8 = (r >> 4).clamp(0, 255) as u8;
            let g8 = (g >> 4).clamp(0, 255) as u8;
            let b8 = (b >> 4).clamp(0, 255) as u8;

            let rgb_idx = idx * 3;
            rgb[rgb_idx] = r8;
            rgb[rgb_idx + 1] = g8;
            rgb[rgb_idx + 2] = b8;
        }
    }

    rgb
}

// Helper: Avg orthogonal G (left/right/up/down)
fn avg_g_ortho(bayer: &[u16], x: usize, y: usize, width: usize, height: usize) -> u16 {
    let mut sum = 0u32;
    let mut cnt = 0u32;
    if x > 0 { sum += bayer[y * width + x - 1] as u32; cnt += 1; }
    if x + 1 < width { sum += bayer[y * width + x + 1] as u32; cnt += 1; }
    if y > 0 { sum += bayer[(y - 1) * width + x] as u32; cnt += 1; }
    if y + 1 < height { sum += bayer[(y + 1) * width + x] as u32; cnt += 1; }
    if cnt > 0 { (sum / cnt) as u16 } else { 0 }
}

// Helper: Avg diagonal B for R pos (or R for B pos via param)
fn avg_diag_b(bayer: &[u16], x: usize, y: usize, width: usize, height: usize, is_r: bool) -> u16 {
    let mut sum_ul_dr = 0u32;  // Upper-left + down-right
    let mut sum_ur_dl = 0u32;  // Upper-right + down-left
    let mut cnt1 = 0u32;
    let mut cnt2 = 0u32;

    // UL
    if x >= 1 && y >= 1 {
        let val = bayer[(y - 1) * width + (x - 1)] as u32;
        sum_ul_dr += val;
        cnt1 += 1;
    }
    // DR
    if x + 1 < width && y + 1 < height {
        let val = bayer[(y + 1) * width + (x + 1)] as u32;
        sum_ul_dr += val;
        cnt1 += 1;
    }
    // UR
    if x + 1 < width && y >= 1 {
        let val = bayer[(y - 1) * width + (x + 1)] as u32;
        sum_ur_dl += val;
        cnt2 += 1;
    }
    // DL
    if x >= 1 && y + 1 < height {
        let val = bayer[(y + 1) * width + (x - 1)] as u32;
        sum_ur_dl += val;
        cnt2 += 1;
    }

    let avg1 = if cnt1 > 0 { (sum_ul_dr / cnt1) as u16 } else { 0 };
    let avg2 = if cnt2 > 0 { (sum_ur_dl / cnt2) as u16 } else { 0 };
    (avg1 + avg2) / 2  // Average the two diagonal pairs for smoother result
}

// Horiz even: Avg left/right (R positions)
fn avg_horiz_even(bayer: &[u16], x: usize, y: usize, width: usize, _height: usize) -> u16 {
    let mut sum = 0u32;
    let mut cnt = 0u32;
    if x > 0 { sum += bayer[y * width + x - 1] as u32; cnt += 1; }
    if x + 1 < width { sum += bayer[y * width + x + 1] as u32; cnt += 1; }
    if cnt > 0 { (sum / cnt).clamp(0, 4095) as u16 } else { 0 }
}

// Horiz odd: Avg left/right (B positions)
fn avg_horiz_odd(bayer: &[u16], x: usize, y: usize, width: usize, _height: usize) -> u16 {
    let mut sum = 0u32;
    let mut cnt = 0u32;
    if x > 0 { sum += bayer[y * width + x - 1] as u32; cnt += 1; }
    if x + 1 < width { sum += bayer[y * width + x + 1] as u32; cnt += 1; }
    if cnt > 0 { (sum / cnt).clamp(0, 4095) as u16 } else { 0 }
}

// Vert even: Avg up/down (R positions)
fn avg_vert_even(bayer: &[u16], x: usize, y: usize, width: usize, height: usize) -> u16 {
    let mut sum = 0u32;
    let mut cnt = 0u32;
    if y > 0 { sum += bayer[(y - 1) * width + x] as u32; cnt += 1; }
    if y + 1 < height { sum += bayer[(y + 1) * width + x] as u32; cnt += 1; }
    if cnt > 0 { (sum / cnt).clamp(0, 4095) as u16 } else { 0 }
}

// Vert odd: Avg up/down (B positions)
fn avg_vert_odd(bayer: &[u16], x: usize, y: usize, width: usize, height: usize) -> u16 {
    let mut sum = 0u32;
    let mut cnt = 0u32;
    if y > 0 { sum += bayer[(y - 1) * width + x] as u32; cnt += 1; }
    if y + 1 < height { sum += bayer[(y + 1) * width + x] as u32; cnt += 1; }
    if cnt > 0 { (sum / cnt).clamp(0, 4095) as u16 } else { 0 }
}
pub fn save_rgb8_image(p0: &String, p1: &Vec<u8>, p2: usize, p3: usize) -> Result<(), std::io::Error> {
    use image::{ImageBuffer, Rgb};
    let img_buffer: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(p2 as u32, p3 as u32, p1.clone())
        .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create image buffer"))?;
    img_buffer.save(p0).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}
