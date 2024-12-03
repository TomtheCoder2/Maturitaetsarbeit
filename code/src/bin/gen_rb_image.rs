use image::Rgb;

fn main() {
    let width = 728;
    let height = 544;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let hue = (x as f32 / width as f32) * 360.0;
        let saturation = 1.0;
        let value = y as f32 / height as f32;

        let rgb = hsv_to_rgb(hue, saturation, value);
        *pixel = Rgb([rgb.0, rgb.1, rgb.2]);
    }

    imgbuf.save("rainbow.png").unwrap();

    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = match h {
            h if h < 60.0 => (c, x, 0.0),
            h if h < 120.0 => (x, c, 0.0),
            h if h < 180.0 => (0.0, c, x),
            h if h < 240.0 => (0.0, x, c),
            h if h < 300.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}
