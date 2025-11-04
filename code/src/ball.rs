use image::io::Reader as ImageReader;
use image::Pixel;
use image::{DynamicImage, GenericImage, RgbImage};
use image::{GenericImageView, ImageBuffer, Rgb, Rgba};
use nalgebra::{clamp, Vector2};
use std::fmt::{Display, Formatter};
use std::sync::atomic::AtomicI32;

pub static MAGNITUE_DIFF: AtomicI32 = AtomicI32::new(20);

pub struct Ball {
    x: u32,
    y: u32,
    radius: f32,
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!($($arg)*);
        }
    }
}

pub type SelectionFn = dyn Fn(u8, u8, u8) -> bool;

pub fn standard_selection(r: u8, g: u8, b: u8) -> bool {
    (r as i32 + g as i32 + b as i32) > 128
}

pub fn read_image(
    file_name: String,
    out_dir: String,
    ball_comp: BallComp,
) -> Result<Vec<Ball>, Box<dyn std::error::Error>> {
    let t1 = std::time::Instant::now();
    let img = ImageReader::open(file_name.clone())?.decode()?;
    const SCALING: u32 = 1;
    let new_width = img.width() / SCALING;
    let new_height = img.height() / SCALING;
    // let mut output = ImageBuffer::new(img.width() / SCALING + 1, img.height() / SCALING + 1);
    let mut output = ImageBuffer::new(img.width(), img.height());
    output.put_pixel(0u32, 0u32, Rgba([0u8, 0, 0, 0]));
    // ImageBuffer::new((img.width() as f32 * scaling_factor) as u32, (img.height() as f32 * scaling_factor) as u32);

    #[derive(Debug)]
    struct BallRaw {
        sum_x: u64,
        sum_y: u64,
        count: u64,
        max_x: u32,
        max_y: u32,
        min_x: u32,
        min_y: u32,
    }
    // let mut new_output = ImageBuffer::new(img.width() / SCALING + 1, img.height() / SCALING + 1);
    let mut balls_raw: Vec<BallRaw> = Vec::new();
    // let pixels = img.pixels().collect::<Vec<_>>();
    debug!("Image read in {} ms", t1.elapsed().as_millis());
    debug!("Image dimensions: {:?}", img.dimensions());
    let t1 = std::time::Instant::now();
    let mut counter = 0;
    // debug!("pixels: {}", img.pixels().count());
    for (_i, pixel) in img.pixels().enumerate() {
        if counter == SCALING - 1 {
            // let pixel = pixels[i];
            // debug!("pixel: {:?}", pixel);
            let rgb = pixel.2.channels();
            let (r, g, b, a) = (rgb[0], rgb[1], rgb[2], rgb[3]);
            // if r > 50 {
            // println!("({}, {}): {:?}", pixel.0, pixel.1, rgb);
            // }
            let x = pixel.0 / SCALING;
            let y = pixel.1 / SCALING;
            output.put_pixel(x * SCALING, y * SCALING, Rgba([r, g, b, a / 2]));
            // keep the white pixels
            if (r as i32 + g as i32 + b as i32) > 3 * 30 {
                output.put_pixel(x * SCALING, y * SCALING, Rgba([r, g, b, a]));
                // output.put_pixel(x, y, Rgba([r, g, b, a]));
                // go through all balls, check if the pixel is close to any of them (within 2 pixels)
                let mut found = false;
                for ball in balls_raw.iter_mut() {
                    const MAX_DISTANCE: u32 = 2;
                    if x as i32 >= ball.min_x as i32 - MAX_DISTANCE as i32
                        && x <= ball.max_x + MAX_DISTANCE
                        && y as i32 >= ball.min_y as i32 - MAX_DISTANCE as i32
                        && y <= ball.max_y + MAX_DISTANCE
                    {
                        ball.sum_x += x as u64;
                        ball.sum_y += y as u64;
                        ball.count += 1;
                        if x < ball.min_x {
                            ball.min_x = x;
                        }
                        if x > ball.max_x {
                            ball.max_x = x;
                        }
                        if y < ball.min_y {
                            ball.min_y = y;
                        }
                        if y > ball.max_y {
                            ball.max_y = y;
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    balls_raw.push(BallRaw {
                        sum_x: x as u64,
                        sum_y: y as u64,
                        count: 1,
                        max_x: x,
                        max_y: y,
                        min_x: x,
                        min_y: y,
                    });
                }
            } else {
                // output.put_pixel(x, y, Rgba([r, g, b, a / 2]));
            }
            counter = 0;
        } else {
            counter += 1;
        }
    }
    // filter out the balls that are too small
    balls_raw.retain(|ball| ball.count > 2);
    debug!("Image processed in {:?}", t1.elapsed());
    // debug!("balls: {:?}, len: {}", balls, balls.len());
    impl Display for Ball {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Ball: ({}, {}), radius: {}", self.x, self.y, self.radius)
        }
    }
    let mut balls = Vec::new();
    for (_i, pixel) in img.pixels().enumerate() {
        let rgb = pixel.2.channels();
        let (r, g, b, a) = (rgb[0], rgb[1], rgb[2], rgb[3]);
        let x = pixel.0;
        let y = pixel.1;
        // output.put_pixel(x, y, Rgba([r, g, b, a / 2]));
    }
    // crate lines with the centers of the balls that are between 25% and 75% of the image
    for ball in balls_raw.iter() {
        let x = (ball.sum_x / ball.count) as u32;
        let y = (ball.sum_y / ball.count) as u32;
        // if x > NEW_WIDTH / 3
        //     && x < NEW_WIDTH * 3 / 4
        //     && y > NEW_HEIGHT / 3
        //     && y < NEW_HEIGHT * 3 / 4
        // {
        // todo: check if this is even right
        let radius = (((ball.max_x - ball.min_x).pow(2) + (ball.max_y - ball.min_y).pow(2)) as f32)
            .sqrt()
            / 2.0;
        // check that the radius is between 10 and 40
        if radius < 15.0 || radius > 40.0 {
            continue;
        }
        // check that the width and height are about equal to the radius
        if ((ball.max_x - ball.min_x) as f32) < radius * 1.
            || ((ball.max_y - ball.min_y) as f32) < radius * 1.
        {
            continue;
        }
        for i in 0..output.width() {
            // dont show the lines at radius*2
            if (i as i32 - (x * SCALING) as i32).abs() < (radius * SCALING as f32) as i32 * 3 {
                continue;
            }
            output.put_pixel(i, y * SCALING, Rgba([255, 0, 0, 255]));
        }
        for i in 0..output.height() {
            if (i as i32 - (y * SCALING) as i32).abs() < (radius * SCALING as f32) as i32 * 3 {
                continue;
            }
            output.put_pixel(x * SCALING, i, Rgba([255, 0, 0, 255]));
        }
        // make a circle around the ball
        fn draw_circle(
            output: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
            x: u32,
            y: u32,
            radius: f32,
            color: [u8; 4],
        ) {
            for i in 0..360 {
                let x = ((x * SCALING) as f32
                    + radius * SCALING as f32 * (i as f32).to_radians().cos())
                    as u32;
                let y = ((y * SCALING) as f32
                    + radius * SCALING as f32 * (i as f32).to_radians().sin())
                    as u32;
                if x > output.width() - 1 || y > output.height() - 1 {
                    continue;
                }
                output.put_pixel(x, y, Rgba(color));
            }
        }
        draw_circle(&mut output, x, y, radius, [255, 0, 0, 0]);
        // debug!(
        //     "center: ({}, {}) normal coords: ({}, {})",
        //     x,
        //     y,
        //     x * SCALING,
        //     y * SCALING
        // );
        balls.push(Ball {
            x: x * SCALING,
            y: y * SCALING,
            radius: radius * SCALING as f32,
        });

        // println!("Ball at 1086: {:?}, 134", ball_comp.intersection_x(1086.));
        if let Some(mid_point) = ball_comp.intersection_x(1031.) {
            let mid_point = mid_point.0;
            let x = mid_point.x as u32;
            let y = mid_point.y as u32;
            draw_circle(&mut output, x, y, radius, [255, 255, 0, 0]);
        }
        for ball in &ball_comp.positions {
            let x = ball.0.x as u32;
            let y = ball.0.y as u32;
            draw_circle(&mut output, x, y, radius, [0, 255, 0, 0]);
        }
        // draw a line from the ball to the edge with  ball_comp.velocity
        let max_iter = 1000;
        let mut iter = 0;
        let start_x = x;
        let mut x = x as i32;
        let m = ball_comp.velocity.y as f32 / ball_comp.velocity.x as f32;
        // println!(
        //     "m: {}, vel: {}, {}",
        //     m, ball_comp.velocity.x, ball_comp.velocity.y
        // );
        let add: i32 = if ball_comp.velocity.x > 0. { 1 } else { -1 };
        while (x > 0 && x < output.width() as i32) && iter < max_iter {
            x += add;
            if x < 0 {
                continue;
            }
            let line_y = (y as f32 + m * (x - start_x as i32) as f32) as u32;
            iter += 1;
            if x > output.width() as i32 - 1 || line_y > output.height() - 1 {
                continue;
            }
            // println!("putting pixel at ({},{})", x, y);
            output.put_pixel(x as u32, line_y, Rgba([255, 0, 255, 0]));
        }
        // }
    }
    debug!("Image filtered in {} ms", t1.elapsed().as_millis());
    let t1 = std::time::Instant::now();
    let out_file = format!(
        "{}/{}",
        out_dir,
        file_name.split("\\").last().unwrap().replace("png", "jpg")
    );
    debug!("out_file: {}", out_file);
    output.save(out_file)?;
    debug!("Image saved in {} ms", t1.elapsed().as_millis());
    // print all the balls
    for ball in balls.iter() {
        debug!("{}", ball);
    }
    Ok(balls)
}

pub fn find_ball(
    img: &[u8],
    width: u32,
    height: u32,
    ball_comp: &mut BallComp,
    time: f32,
    selection_fn: &dyn Fn(u8, u8, u8) -> bool,
    min_radius: f32,
    max_radius: f32,
) -> (i32, i32, f32) {
    const SCALING: u32 = 1;
    #[derive(Debug)]
    struct BallRaw {
        sum_x: u64,
        sum_y: u64,
        count: u64,
        max_x: u32,
        max_y: u32,
        min_x: u32,
        min_y: u32,
    }
    // let mut new_output = ImageBuffer::new(img.width() / SCALING + 1, img.height() / SCALING + 1);
    let mut balls_raw: Vec<BallRaw> = Vec::new();
    // let pixels = img.pixels().collect::<Vec<_>>();
    let t1 = std::time::Instant::now();
    let mut counter = 0;
    // debug!("pixels: {}", img.pixels().count());
    for i in 0..width * height {
        if counter == SCALING - 1 {
            // let pixel = pixels[i];
            // debug!("pixel: {:?}", pixel);
            let r = img[i as usize * 3];
            let g = img[i as usize * 3 + 1];
            let b = img[i as usize * 3 + 2];
            // if r > 50 {
            // println!("({}, {}): {:?}", pixel.0, pixel.1, rgb);
            // }
            let x = i % width;
            let y = i / width;
            // keep the white pixels
            // if (r as i32 + g as i32 + b as i32) > 3 * 50 {
            if selection_fn(r, g, b) {
                // output.put_pixel(x, y, Rgba([r, g, b, a]));
                // go through all balls, check if the pixel is close to any of them (within 2 pixels)
                let mut found = false;
                for ball in balls_raw.iter_mut() {
                    const MAX_DISTANCE: u32 = 1;
                    if x as i32 >= ball.min_x as i32 - MAX_DISTANCE as i32
                        && x <= ball.max_x + MAX_DISTANCE
                        && y as i32 >= ball.min_y as i32 - MAX_DISTANCE as i32
                        && y <= ball.max_y + MAX_DISTANCE
                    {
                        ball.sum_x += x as u64;
                        ball.sum_y += y as u64;
                        ball.count += 1;
                        if x < ball.min_x {
                            ball.min_x = x;
                        }
                        if x > ball.max_x {
                            ball.max_x = x;
                        }
                        if y < ball.min_y {
                            ball.min_y = y;
                        }
                        if y > ball.max_y {
                            ball.max_y = y;
                        }
                        found = true;
                        // let radius = ((ball.MAX_X - ball.MIN_X).pow(2)
                        //     + (ball.MAX_Y - ball.MIN_Y).pow(2) as f32)
                        //     .sqrt()
                        //     / 2.;
                        // let min: u32 = 18;
                        // let max: u32 = 25;
                        // // check that the width and height are about equal to the radius
                        // if ((ball.MAX_X - ball.MIN_X) as f32).powi(2) * 2 < radius
                        //     || ((ball.MAX_Y - ball.MIN_Y) as f32).powi(2) * 2 < radius
                        // {
                        //     continue;
                        // }
                        // if radius < min as f32 * 2. || radius > max as f32 * 2 {
                        //     // return this ball
                        //     return (
                        //         ball.sum_x as i32 / ball.count as i32,
                        //         ball.sum_y as i32 / ball.count as i32,
                        //     );
                        // }
                        break;
                    }
                }
                if !found {
                    balls_raw.push(BallRaw {
                        sum_x: x as u64,
                        sum_y: y as u64,
                        count: 1,
                        max_x: x,
                        max_y: y,
                        min_x: x,
                        min_y: y,
                    });
                }
            } else {
                // output.put_pixel(x, y, Rgba([r, g, b, a / 2]));
            }
            counter = 0;
        } else {
            counter += 1;
        }
    }
    // println!("ball length: {}", balls_raw.len());
    // filter out the balls that are too small
    balls_raw.retain(|ball| ball.count > 2);
    let mut balls = Vec::new();
    // crate lines with the centers of the balls that are between 25% and 75% of the image
    for ball in balls_raw.iter() {
        let x = (ball.sum_x / ball.count) as u32;
        let y = (ball.sum_y / ball.count) as u32;
        // if x > NEW_WIDTH / 3
        //     && x < NEW_WIDTH * 3 / 4
        //     && y > NEW_HEIGHT / 3
        //     && y < NEW_HEIGHT * 3 / 4
        // {
        // todo: check if this is even right
        let radius = (((ball.max_x - ball.min_x).pow(2) + (ball.max_y - ball.min_y).pow(2)) as f32)
            .sqrt()
            / 2.0;
        // check that the radius is between 10 and 20
        if radius < min_radius || radius > max_radius {
            continue;
        }
        // check that the width and height are about equal to the radius
        if ((ball.max_x - ball.min_x) as f32) < radius * 1.
            || ((ball.max_y - ball.min_y) as f32) < radius * 1.
        {
            continue;
        }
        balls.push(Ball {
            x: x * SCALING,
            y: y * SCALING,
            radius: radius as f32 * SCALING as f32,
        });
        // }
    }
    let ball: (i32, i32, f32) = if balls.len() > 1 {
        // get the ball with the radius thats nearest to 20
        let mut min_diff = 1000.0;
        let mut min_diff_index = 0;
        for (i, ball) in balls.iter().enumerate() {
            let diff = (ball.radius - 15.0).abs();
            if diff < min_diff {
                min_diff = diff;
                min_diff_index = i;
            }
        }
        (
            balls[min_diff_index].x as i32,
            balls[min_diff_index].y as i32,
            balls[min_diff_index].radius,
        )
    } else if balls.len() == 1 {
        (balls[0].x as i32, balls[0].y as i32, balls[0].radius)
    } else {
        (-1, -1, -1.0)
    };
    let ball_vec2 = Vec2::new(ball.0 as f32, ball.1 as f32);
    ball_comp.update(ball_vec2, time);
    (ball.0, ball.1, ball.2)
}
pub const SCALING: u32 = 1;
// make a circle around the ball
pub fn draw_circle(output: &mut DynamicImage, x: u32, y: u32, radius: f32, color: [u8; 4]) {
    for i in 0..360 {
        let x =
            ((x * SCALING) as f32 + radius * SCALING as f32 * (i as f32).to_radians().cos()) as u32;
        let y =
            ((y * SCALING) as f32 + radius * SCALING as f32 * (i as f32).to_radians().sin()) as u32;
        if x > output.width() - 1 || y > output.height() - 1 {
            continue;
        }
        output.put_pixel(x, y, Rgba(color));
    }
}

pub fn read_image_vis(
    input: &mut DynamicImage,
    output: &mut DynamicImage,
    ball_comp: &mut BallComp,
    time: f32,
    selection_fn: &SelectionFn,
    min_radius: f32,
    max_radius: f32,
    overlay_ball_prediction: bool,
) -> (i32, i32, f32) {
    // convert to u8 rgb vector
    let pixels = input.to_rgb8().into_raw();
    assert_eq!(
        pixels.len(),
        output.width() as usize * output.height() as usize * 3
    );

    let ball = find_ball(
        &pixels,
        output.width(),
        output.height(),
        ball_comp,
        time,
        selection_fn,
        min_radius,
        max_radius,
    );
    // println!("ball: {:?}", ball);
    if ball.0 == -1 {
        return ball;
    }
    let x = ball.0 as u32;
    let y = ball.1 as u32;
    let radius = ball.2;

    for i in 0..output.width() {
        // dont show the lines at radius*2
        if (i as i32 - (x * SCALING) as i32).abs() < (radius * SCALING as f32) as i32 * 3 {
            continue;
        }
        output.put_pixel(i, y * SCALING, Rgba([255, 0, 0, 255]));
        output.put_pixel(
            i,
            clamp(y as i32 * SCALING as i32 + 1, 0, output.height() as i32 - 1) as u32,
            Rgba([255, 0, 0, 255]),
        );
        output.put_pixel(
            i,
            clamp(y as i32 * SCALING as i32 - 1, 0, output.height() as i32 - 1) as u32,
            Rgba([255, 0, 0, 255]),
        );
    }
    for i in 0..output.height() {
        if (i as i32 - (y * SCALING) as i32).abs() < (radius * SCALING as f32) as i32 * 3 {
            continue;
        }
        output.put_pixel(x * SCALING, i, Rgba([255, 0, 0, 255]));
        output.put_pixel(
            clamp(x as i32 * SCALING as i32 + 1, 0, output.width() as i32 - 1) as u32,
            i,
            Rgba([255, 0, 0, 255]),
        );
        output.put_pixel(
            clamp(x as i32 * SCALING as i32 - 1, 0, output.width() as i32 - 1) as u32,
            i,
            Rgba([255, 0, 0, 255]),
        );
    }
    draw_circle(output, x, y, radius, [255, 0, 0, 0]);
    draw_circle(output, x, y, radius + 1., [255, 0, 0, 0]);

    // println!("Ball at 1086: {:?}, 134", ball_comp.intersection_x(1086.));
    if let Some(mid_point) = ball_comp.intersection_x(44.0) {
        if ball_comp.velocity.x < 0. {
            let mid_point = mid_point.0;
            let x = mid_point.x as u32;
            let y = mid_point.y as u32;
            if overlay_ball_prediction && ball_comp.velocity.magnitude() > 10. {
                draw_circle(output, x, y, radius, [255, 255, 0, 0]);
                draw_circle(output, x, y, radius + 1., [255, 255, 0, 0]);
            }
            // draw_circle(output, x, y, radius, [255, 255, 0, 0]);}
        }
    }
    for ball in &ball_comp.positions {
        let x = ball.0.x as u32;
        let y = ball.0.y as u32;
        if overlay_ball_prediction && ball_comp.velocity.magnitude() > 10. {
            draw_circle(output, x, y, radius, [0, 255, 0, 0]);
        }
    }
    // draw a line from the ball to the edge with  ball_comp.velocity
    let max_iter = 1000;
    let mut iter = 0;
    let start_x = x;
    let mut x = x as i32;
    let m = ball_comp.velocity.y as f32 / ball_comp.velocity.x as f32;
    // println!(
    //     "m: {}, vel: {}, {}",
    //     m, ball_comp.velocity.x, ball_comp.velocity.y
    // );
    if overlay_ball_prediction && ball_comp.velocity.magnitude() > 10. {
        let add: i32 = if ball_comp.velocity.x > 0. { 1 } else { -1 };
        while (x > 0 && x < output.width() as i32) && iter < max_iter {
            x += add;
            if x < 0 {
                continue;
            }
            let line_y = (y as f32 + m * (x - start_x as i32) as f32) as u32;
            iter += 1;
            if x > output.width() as i32 - 1 || line_y > output.height() - 1 {
                continue;
            }
            // println!("putting pixel at ({},{})", x, y);
            output.put_pixel(x as u32, line_y, Rgba([255, 0, 255, 0]));
            output.put_pixel(
                x as u32,
                clamp(line_y as i32 + 1, 0, output.height() as i32 - 1) as u32,
                Rgba([255, 0, 255, 0]),
            );
            output.put_pixel(
                x as u32,
                clamp(line_y as i32 - 1, 0, output.height() as i32 - 1) as u32,
                Rgba([255, 0, 255, 0]),
            );
        }
    }
    ball
}

pub type Vec2 = Vector2<f32>;

/// Function to perform linear regression and compute the velocity of the ball
pub fn compute_velocity(positions: &Vec<(Vector2<f32>, f32)>) -> Vector2<f32> {
    let mut sum_xy = Vector2::new(0.0, 0.0); // Sum of position*time
    let mut sum_x = Vector2::new(0.0, 0.0); // Sum of positions
    let mut sum_y = 0.0; // Sum of time
    let mut sum_yy = 0.0; // Sum of time^2
    let n = positions.len() as f32; // Number of samples

    for &(pos, time) in positions.iter() {
        sum_xy += pos * time;
        sum_x += pos;
        sum_y += time;
        sum_yy += time * time;
    }

    // Compute the velocity using linear regression formula: v = (n * Σxy - Σx * Σy) / (n * Σyy - Σy^2)
    let velocity = (n * sum_xy - sum_x * sum_y) / (n * sum_yy - sum_y * sum_y);
    velocity
}

#[derive(Debug, Clone)]
pub struct BallComp {
    pub velocity: Vec2,
    // real world position though
    // (position, time in seconds)
    pub positions: Vec<(Vec2, f32)>,
    pub position: Vec2,
}

impl Default for BallComp {
    fn default() -> Self {
        BallComp {
            velocity: Vec2::new(0.0, 0.0),
            positions: vec![(Vec2::new(0.0, 0.0), -1. / 149.)],
            position: Vec2::new(0.0, 0.0),
        }
    }
}

impl BallComp {
    pub fn new() -> Self {
        BallComp::default()
    }

    pub fn predict_position(&self, dt: f32) -> Vec2 {
        self.position + self.velocity * dt
    }

    /// Check if the ball is still moving with the same velocity (direction and magnitude, but magnitude can be less because of drag), else change the velocity
    pub fn check_velocity(&mut self, ball0: Vec2, ball1: Vec2, dt: f32) {
        let new_velocity = (ball1 - ball0) / dt;
        let diff = new_velocity - self.velocity;
        if diff.magnitude() > 0.1 {
            println!(
                "velocity changed, old: {:?}, new: {:?}",
                self.velocity, new_velocity
            );
            self.velocity = (new_velocity + self.velocity) / 2.0;
            self.position = ball1;
        }
    }

    pub fn update(&mut self, ball: Vec2, time: f32) {
        // lets chech if the ball is still moving with the same velocity in the same direction
        let prediction = self.predict_position(time - self.positions.last().unwrap().1);
        let diff = prediction - ball;
        // println!(
        //     "prediction: {:?}, ball: {:?}, diff: {:?}",
        //     prediction, ball, diff
        // );
        if diff.magnitude() / self.velocity.magnitude() * 2000. > MAGNITUE_DIFF.load(std::sync::atomic::Ordering::Relaxed) as f32 {
            let diff = self.positions.last().unwrap().0 - ball;
            let new_velocity = diff / (self.positions.last().unwrap().1 - time);
            // println!(
            // "velocity changed, old: {:?}, new: {:?}",
            // self.velocity, new_velocity
            // );
            self.velocity = new_velocity;
            self.positions.clear();
            self.positions.push((ball, time));
        } else {
            self.positions.push((ball, time));
            // max 10 samples
            if self.positions.len() > 200 {
                self.positions.remove(0);
            }
            // println!("Length of positions: {}", self.positions.len());
            self.velocity = compute_velocity(&self.positions);
        }

        self.position = ball;
    }

    // Compute the intersection with the line x (vertical line)
    // returns the coordinates of the intersection and the time it will take to reach that point
    pub fn intersection_x(&self, x: f32) -> Option<(Vec2, f32)> {
        if self.velocity.x == 0.0 {
            return None;
        }
        let t = (x - self.position.x) / self.velocity.x;
        Some((self.position + self.velocity * t, t))
    }
}
