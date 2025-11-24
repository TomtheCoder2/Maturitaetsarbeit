use crate::arduino_com::ArduinoCom;
use crate::ball::{standard_selection, BallComp};
use crate::image_utils::{gpu_debayer, MetalContext};
use crate::live_feed::Command::*;
use crate::live_feed::{
    subtract_image, Command, Selection, FOLLOWBALL, FPS, IMAGE_BUFFER, IMAGE_BUFFER_UNDISTORTED,
    PAUSEPLAYER, PAUSESHOOTING, TIMING_OFFSET,
};
use cameleon::genapi::NodeStore;
use cameleon::u3v::{ControlHandle, StreamHandle};
use cameleon::Camera;
use cameleon_genapi::interface::INode;
use egui::ColorImage;
use std::io::Write;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Instant;

pub fn set_value(camera: &mut Camera<ControlHandle, StreamHandle>, name: String, value: f64) {
    let mut params_ctxt = camera.params_ctxt().unwrap();
    // Get `Gain` node of `GenApi`.
    // `GenApi SFNC` defines that `Gain` node should have `IFloat` interface,
    // so this conversion would be success if the camera follows that.
    // Some vendors may define `Gain` node as `IInteger`, in that case, use
    // `as_integer(&params_ctxt)` instead of `as_float(&params_ctxt).
    let exposure = params_ctxt
        .node(&*name)
        .unwrap()
        .as_float(&params_ctxt)
        .unwrap();

    // Get the current value of `Gain`.
    if exposure.is_readable(&mut params_ctxt).unwrap() {
        let value = exposure.value(&mut params_ctxt).unwrap();
        println!("{name}: {}", value);
    }

    // Set `0.1` to `Gain`.
    if exposure.is_writable(&mut params_ctxt).unwrap() {
        exposure.set_value(&mut params_ctxt, value).unwrap();
    } else {
        println!("{name} is not writable");
    }

    // Get the current value of `Gain`.
    // The float value may be truncated to valid value by the camera.
    if exposure.is_readable(&mut params_ctxt).unwrap() {
        let value = exposure.value(&mut params_ctxt).unwrap();
        println!("New {name} {}", value);
    }
}

pub fn set_value_i64(camera: &mut Camera<ControlHandle, StreamHandle>, name: String, value: i64) {
    let mut params_ctxt = camera.params_ctxt().unwrap();
    let exposure = params_ctxt
        .node(&*name)
        .unwrap()
        .as_integer(&params_ctxt)
        .unwrap();
    if exposure.is_readable(&mut params_ctxt).unwrap() {
        let value = exposure.value(&mut params_ctxt).unwrap();
        println!("{name}: {}", value);
    }
    if exposure.is_writable(&mut params_ctxt).unwrap() {
        exposure.set_value(&mut params_ctxt, value).unwrap();
    } else {
        println!("{name} is not writable");
    }
    if exposure.is_readable(&mut params_ctxt).unwrap() {
        let value = exposure.value(&mut params_ctxt).unwrap();
        println!("New {name} {}", value);
    }
}

pub fn set_enum_value(camera: &mut Camera<ControlHandle, StreamHandle>, name: &str, value: &str) {
    let mut params_ctxt = camera.params_ctxt().unwrap();
    let enum_node = params_ctxt
        .node(name)
        .unwrap()
        .as_enumeration(&params_ctxt)
        .unwrap();

    if enum_node.is_writable(&mut params_ctxt).unwrap() {
        if let Err(e) = enum_node.set_entry_by_symbolic(&mut params_ctxt, value) {
            println!("Failed to set {} to {}: {}", name, value, e);
        }
    } else {
        println!("{} is not writable", name);
    }

    if enum_node.is_readable(&mut params_ctxt).unwrap() {
        let current_value = enum_node.current_entry(&mut params_ctxt).unwrap();
        println!("New {}: {:?}", name, current_value);
    }
}

pub fn execute_command(camera: &mut Camera<ControlHandle, StreamHandle>, name: &str) {
    let mut params_ctxt = camera.params_ctxt().unwrap();
    let command_node = params_ctxt
        .node(name)
        .unwrap()
        .as_command(&params_ctxt)
        .unwrap();

    if command_node.is_writable(&mut params_ctxt).unwrap() {
        if let Err(e) = command_node.execute(&mut params_ctxt) {
            println!("Failed to execute command {}: {}", name, e);
        } else {
            println!("Executed command: {}", name);
        }
    } else {
        println!("Command {} is not executable", name);
    }
}

pub fn print_all_options(camera: &mut Camera<ControlHandle, StreamHandle>) {
    let mut options_file = std::fs::File::create("camera_options.txt").unwrap();
    let mut params_ctxt = camera.params_ctxt().unwrap();
    params_ctxt.node_store().visit_nodes(|node| {
        // Print debug info for each node (most implementations include the node name)
        // println!("{:?}", node);
        options_file
            .write_all(format!("{:#?}\n", node).as_bytes())
            .unwrap();

        // node: &NodeData: is an enum with enum types:
        // #[derive(Debug, Clone)]
        // pub enum NodeData {
        //     Node(Box<Node>),
        //     Category(Box<CategoryNode>),
        //     Integer(Box<IntegerNode>),
        //     IntReg(Box<IntRegNode>),
        //     MaskedIntReg(Box<MaskedIntRegNode>),
        //     Boolean(Box<BooleanNode>),
        //     Command(Box<CommandNode>),
        //     Enumeration(Box<EnumerationNode>),
        //     EnumEntry(Box<EnumEntryNode>),
        //     Float(Box<FloatNode>),
        //     FloatReg(Box<FloatRegNode>),
        //     String(Box<StringNode>),
        //     StringReg(Box<StringRegNode>),
        //     Register(Box<RegisterNode>),
        //     Converter(Box<ConverterNode>),
        //     IntConverter(Box<IntConverterNode>),
        //     SwissKnife(Box<SwissKnifeNode>),
        //     IntSwissKnife(Box<IntSwissKnifeNode>),
        //     Port(Box<PortNode>),
        //
        //     // TODO: Implement DCAM specific ndoes.
        //     ConfRom(()),
        //     TextDesc(()),
        //     IntKey(()),
        //     AdvFeatureLock(()),
        //     SmartFeature(()),
        use cameleon_genapi::store::NodeData::*;
        match node {
            // Node(_)
            // | Category(_)
            | Integer(_)
            // | IntReg(_)
            // | MaskedIntReg(_)
            // | Boolean(_)
            // | Command(_) | Enumeration(_) | EnumEntry(_) | Float(_) | FloatReg(_)
            // | String(_)
            // | StringReg(_)
            => {
                // println!("{:?}", node);
            }
            _ => {}
        }
        match node {
            Integer(i) => {
                let desc = i.node_base().description().unwrap_or("No description");
                let name = i.node_base().display_name();
                if desc.contains("pixels") {
                    println!("int: name: {:?}, Description: {:?}", name, desc);
                }
                // println!("{:?}", i.node_base().description().unwrap_or("No description"));
            }
            String(s) => {
                let desc = s.node_base().description().unwrap_or("No description");
                let name = s.node_base().display_name();
                if desc.to_ascii_lowercase().contains("format") {
                    println!("string: name: {:?}, Description: {:?}", name, desc);
                }
            }
            _ => {}
        }
    });
}

pub fn load_raw() -> ColorImage {
    let image = image::open("./raw.png").expect("Failed to open image");
    let rgb8_data = image.to_rgb8().into_raw();
    let width = image.width() as usize;
    let height = image.height() as usize;
    println!("Loaded raw image: {}x{}", width, height);
    create_color_image_from_rgb8(&rgb8_data, width, height)
}

fn create_color_image_from_rgb8(rgb8_data: &[u8], width: usize, height: usize) -> ColorImage {
    // Convert RGB8 to RGBA8 as expected by ColorImage
    let mut pixels = Vec::with_capacity(width * height * 4);

    for chunk in rgb8_data.chunks(3) {
        pixels.push(chunk[0]); // R
        pixels.push(chunk[1]); // G
        pixels.push(chunk[2]); // B
        pixels.push(255); // A (set alpha to 255 for full opacity)
    }

    ColorImage::from_rgba_unmultiplied([width, height], &pixels)
}

pub struct CamThread {
    // todo: migrate all variables here
    player_x_coord: i32,
}

pub const WIDTH: u32 = 728;
pub const HEIGHT: u32 = 544;
pub const LEFT_MARGIN: i32 = 100;
pub const RIGHT_MARGIN: i32 = 100;
pub const TOP_MARGIN: i32 = 0;
pub const BOTTOM_MARGIN: i32 = 28;

pub const MIN_X: i32 = -LEFT_MARGIN;
pub const MAX_X: i32 = WIDTH as i32 + RIGHT_MARGIN;
pub const MIN_Y: i32 = -TOP_MARGIN;
pub const MAX_Y: i32 = HEIGHT as i32 + BOTTOM_MARGIN;
pub const NEW_WIDTH: u32 = (MAX_X - MIN_X) as u32;
pub const NEW_HEIGHT: u32 = (MAX_Y - MIN_Y) as u32;

impl CamThread {
    pub fn new() -> Self {
        Self { player_x_coord: 44 }
    }

    pub fn start(mut self) -> Sender<Command> {
        // Enumerates all cameras connected to the host.
        let mut cameras = cameleon::u3v::enumerate_cameras().unwrap();
        let (tx, rx): (Sender<Command>, Receiver<Command>) = mpsc::channel();
        if cameras.is_empty() {
            println!("no camera found");
            return tx;
        }

        let mut camera = cameras.pop().unwrap();

        // Opens the camera.
        camera.open().unwrap();
        // Loads `GenApi` context. This is necessary for streaming
        camera.load_context().unwrap();

        // let mut params_ctxt = camera.params_ctxt().unwrap();
        // params_ctxt.node_store().visit_nodes(|f|
        //         // IntSwissKnife(IntSwissKnifeNode { attr_base: NodeAttributeBase { id: NodeId(2247), name_space: Custom, merge_priority: Mid, expose_static: None }, elem_base: NodeElementBase { tooltip: None, description: None, display_name: None, visibility: Beginner, docu_url: None, is_deprecated: false, event_id: None, p_is_implemented: None, p_is_available: None, p_is_locked: None, p_block_polling: None, imposed_access_mode: RW, p_errors: [], p_alias: None, p_cast_alias: None, p_invalidators: [] }, streamable: false, p_variables: [NamedValue { name: "CORRECTIONSELECTORINDEX", value: NodeId(1365) }], constants: [], expressions: [], formula: Formula { expr: BinOp { kind: Mul, lhs: BinOp { kind: Add, lhs: Integer(386), rhs: Ident("CORRECTIONSELECTORINDEX") }, rhs: Integer(4) } }, unit: None, representation: PureNumber })
        //         match f {
        //             Node::IntSwissKnife(node) => {
        //                 println!("{:#?}", node);
        //             }
        //             _ => {}
        //         });
        // println!("{:?}\n", f));
        set_value(&mut camera, "ExposureTime".to_string(), 1. * 1e6 / 30.0);
        // set_value(&mut camera, "AcquisitionFrameRate".to_string(), 300.0);
        // get_value(&mut camera, "DeviceLinkThroughputLimitMode".to_string());

        execute_command(&mut camera, "AcquisitionStop");
        set_enum_value(&mut camera, "PixelFormat", "BayerRG12p");
        execute_command(&mut camera, "AcquisitionStart");

        // Start streaming. Channel capacity is set to 3.
        let payload_rx = camera.start_streaming(3).unwrap();

        let t0_first_thread = std::time::Instant::now();
        // let t0_second_thread = std::time::Instant::now();

        thread::spawn(move || {
            let t0 = t0_first_thread;
            let mut t0_delta = std::time::Instant::now();
            let mut frame_counter = 0;
            let mut last_fps = [0.0; 100];
            let mut paused = false;
            let metal_context = MetalContext::new().expect("Metal init failed");

            // undistort image

            println!("old width: {}, old height: {}", WIDTH, HEIGHT);
            println!("new width: {}, new height: {}", NEW_WIDTH, NEW_HEIGHT);
            let precompute = crate::gen_table(WIDTH, HEIGHT, NEW_WIDTH, NEW_HEIGHT, MIN_X, MIN_Y);
            // let rl_comp = matura::compute_rl_coords::RLCompute::new();

            let raw = load_raw();
            // we want to convert it to rgb from rgba, so we delete every 4th element
            let raw_image = raw
                .as_raw()
                .to_vec()
                .chunks(4)
                .map(|x| x[0..3].to_vec())
                .flatten()
                .collect::<Vec<u8>>();
            let mut raw_image = raw_image.as_slice();
            let mut raw_image_owned_boxed_slice: Box<[u8]>; // Declared in outer scope to ensure longevity.

            let mut last_command = Instant::now();
            let mut arduino_com = crate::arduino_com::ArduinoCom::new();

            let mut buffer = crate::live_feed::IMAGE_BUFFER_UNDISTORTED.lock().unwrap();
            buffer.0 = NEW_WIDTH;
            buffer.1 = NEW_HEIGHT;
            drop(buffer);

            let mut undistorted_image = vec![0u8; (NEW_WIDTH * NEW_HEIGHT * 3) as usize];
            let mut ball_comp = BallComp::new();
            let mut shoot_time = 0.;
            // whether the ball has already been shot at time shoot_time
            let mut shot = true;
            let mut time_since_catch = Instant::now();
            let mut moved_to_center = true;

            let mut player_calibration_positions = vec![];

            let mut selection_fn: Box<dyn Fn(u8, u8, u8) -> bool> = Box::new(standard_selection);
            let mut min_radius @ mut max_radius = 0.;

            // functions
            const _MIN_MOTOR: i32 = 0;
            const _MAX_MOTOR: i32 = 400;
            let mut min_pixel = 226;
            let mut max_pixel = 350;
            fn move_y(
                y: f32,
                arduino_com: &mut ArduinoCom,
                last_command: &mut Instant,
                min_pixel: i32,
                max_pixel: i32,
            ) {
                // *player_target = o_y as i32;
                // let y = rl_comp.transform_point((x, o_y)).1 + player_0 as f32;
                // println!("oy: {o_y}, y: {y}, 450 - y: {}", 450. - y);
                //
                // motor, pixel y
                // 0,   350
                // 330, 226
                //
                // pixel y, motor
                // 226, 330
                // 350, 0
                //
                // A(226, 330) B(350, 0)
                // y = mx + b
                // m = (y2 - y1) / (x2 - x1)
                // b = y1 - m * x1
                // m = (0 - 330) / (350 - 226) = -330 / 124 = -2.6612903226
                // b = 330 - (-2.66 * 226) = 330 + 600.76 = 931.4516129076
                // y = -2.66 * x + 930.76
                // motor = y - 350
                // println!("y: {y}, min: {MIN_PIXEL}, max: {MAX_PIXEL}");
                if y > min_pixel as f32
                    && y < max_pixel as f32
                    && last_command.elapsed().as_secs_f32() > 0.05
                {
                    // convert from pixel y to motor
                    // let m = (MIN_MOTOR - MAX_MOTOR) as f32 / (MAX_PIXEL - MIN_PIXEL) as f32;
                    // let b = MAX_MOTOR as f32 - m * MIN_PIXEL as f32;
                    // let x = m * y + b;

                    // new formula to convert from pixel y to motor
                    // first convert y to f64, because the polynomial fit is done with f64 and it needs to be very precise
                    let y = y as f64;
                    // cnc shield:
                    let x = -0.0000609070 * y.powi(3)
                        + 0.0577279513 * y.powi(2)
                        + -11.0721181144 * y.powi(1)
                        + 242.7078559049;
                    // rs485:
                    // let x = 0.0001587715 * y.powi(3) + -0.1268654294 * y.powi(2) + 30.4151445206 * y.powi(1) + -1892.1674459350;

                    let x = x as i32;
                    // println!("sending: y: {x}");
                    let paused_player = PAUSEPLAYER.load(Ordering::Relaxed);
                    if !paused_player {
                        // println!("in sending: y: {x}");
                        arduino_com.send_string(&format!("{}", x as i32));
                        *last_command = Instant::now();
                    }
                }
            }
            fn move_center(
                arduino_com: &mut ArduinoCom,
                last_command: &mut Instant,
                min_pixel: i32,
                max_pixel: i32,
            ) {
                if !PAUSEPLAYER.load(Ordering::Relaxed) {
                    let y = (min_pixel + max_pixel) as f32 / 2.;
                    if last_command.elapsed().as_secs_f32() > 0.05 {
                        // println!("Moving to center");
                        // arduino_com.send_string(&format!("{}", 212 as i32));
                        // arduino_com.send_string(&"check 10".to_string());
                        move_y(y, arduino_com, last_command, min_pixel, max_pixel);
                        *last_command = Instant::now();
                    }
                }
            }

            loop {
                if let Ok(message) = rx.try_recv() {
                    match message {
                        Start => {
                            paused = false;
                        }
                        Exposure(value) => {
                            set_value(&mut camera, "ExposureTime".to_string(), value);
                        }
                        Pause => {
                            paused = true;
                        }
                        Reset => {
                            arduino_com.send_string("full_reset");
                            let mut output = "".to_string();
                            while !output.starts_with("end") {
                                output = arduino_com.read_line();
                                // println!("f{:?}f", output.chars().collect::<Vec<char>>());
                                println!("{}", output);
                            }
                            println!("Finished full reset!");
                        }
                        ResetDC => {
                            arduino_com.send_string("reset_dc");
                        }
                        Stop => {
                            break;
                        }
                        ReloadRaw => {
                            let raw = load_raw();
                            raw_image_owned_boxed_slice = raw
                                .as_raw()
                                .to_vec()
                                .chunks(4)
                                .map(|x| x[0..3].to_vec())
                                .flatten()
                                .collect::<Vec<u8>>()
                                .into_boxed_slice();
                            raw_image = &raw_image_owned_boxed_slice;
                            // let raw_image1 = raw_image1.as_slice();
                            // raw_image = raw_image1.clone();
                        }
                        MoveCenter => {
                            move_center(&mut arduino_com, &mut last_command, min_pixel, max_pixel);
                            moved_to_center = true;
                        }
                        Shoot => {
                            arduino_com.send_string("S");
                        }
                        PlayerCalibration(input_pos) => {
                            if input_pos == -1 {
                                // println!("Player calibration started");
                                // arduino_com.send_string("full_reset");
                                // let mut output = "".to_string();
                                // while !output.starts_with("end") {
                                //     output = arduino_com.read_line();
                                //     // println!("f{:?}f", output.chars//().collect::<Vec<char>>());
                                //     println!("o: {}", output);
                                // }
                                // println!("Finished full reset!");
                                arduino_com.sync();
                            } else {
                                //     arduino_com.send_string("check 20");
                                //     std::thread::sleep(std::time::Duration::from_secs(2));
                                // arduino_com.read_everything();
                                arduino_com.sync();
                                arduino_com.send_string("I");
                                // sleep for 500 ms
                                std::thread::sleep(std::time::Duration::from_millis(10));
                                let output = arduino_com.read_line();
                                println!("o: {}", output);
                                // output format:    Pos: 32
                                let pos = output
                                    .split_whitespace()
                                    .nth(1)
                                    .unwrap()
                                    .parse::<f32>()
                                    .unwrap();
                                println!("pos: {}", pos);
                                player_calibration_positions.push(pos as i32);
                                println!("Sending pos: {}", input_pos);
                                arduino_com.send_string(&format!("{}", input_pos));
                            }
                        }
                        FinishPlayerCalibration(positions) => {
                            println!("Player calibration finished");
                            println!(
                                "player_calibration_positions: {:?}, len: {}\npositions: {:?}, len: {}",
                                player_calibration_positions,
                                player_calibration_positions.len(),
                                positions,
                                positions.len()
                            );
                            println!(
                                "positions = [{}]",
                                positions
                                    .iter()
                                    .enumerate()
                                    .map(|x| format!(
                                        "[{}, {}]",
                                        player_calibration_positions[x.0], x.1
                                    ))
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            );
                            player_calibration_positions.clear();
                        }
                        Command::SelectionFn {
                            selection_type,
                            r,
                            g,
                            b,
                            sum,
                        } => {
                            selection_fn = match selection_type {
                                Selection::Separation => {
                                    // inputs for r g b
                                    let r = r; // Clone current values of self.r, self.g, self.b
                                    let g = g;
                                    let b = b;

                                    // Create a closure with `'static` lifetime
                                    Box::new(move |r_in, g_in, b_in| {
                                        r_in > r && g_in > g && b_in > b
                                    })
                                }
                                Selection::Addition => {
                                    // Create a closure with `'static` lifetime
                                    Box::new(move |r_in, g_in, b_in| {
                                        r_in as i32 + g_in as i32 + b_in as i32 > sum
                                    })
                                }
                            };
                        }
                        Radius {
                            min_radius: min,
                            max_radius: max,
                        } => {
                            min_radius = min;
                            max_radius = max;
                        }
                        BallPixel {
                            min_pixel: min,
                            max_pixel: max,
                        } => {
                            min_pixel = min;
                            max_pixel = max;
                        }
                        PlayerXCoord(coord) => {
                            self.player_x_coord = coord;
                        }
                    }
                }
                if paused {
                    continue;
                }
                frame_counter += 1;
                // show rolling fps average
                let elapsed = t0.elapsed();
                // let fps = frame_counter as f64 / elapsed.as_secs_f64();
                let delta = t0_delta.elapsed();
                // println!("delta: {:?}, fps: {:.3}", delta, 1. / delta.as_secs_f64());
                t0_delta = std::time::Instant::now();
                if frame_counter % 1000 == 0 {
                    // println!("avg fps: {:.2}", fps);
                }
                // let fps = 1.0 / delta.as_secs_f64()
                let fps = delta.as_secs_f64();
                last_fps[(frame_counter % last_fps.len()) as usize] = fps;
                *FPS.lock().unwrap() = last_fps.iter().sum::<f64>() / last_fps.len() as f64;
                let payload = match payload_rx.recv_blocking() {
                    Ok(payload) => payload,
                    Err(e) => {
                        println!(
                            "[{}]Payload receive error: {e}",
                            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
                        );
                        continue;
                    }
                };
                // println!(
                //     "payload received! block_id: {:?}, timestamp: {:?}",
                //     payload.id(),
                //     payload.timestamp()
                // );
                // let mut current_player_pos;
                if let Some(image_info) = payload.image_info() {
                    // println!("{:?}\n", image_info);
                    // let width = image_info.width;
                    // let height = image_info.height;

                    let image = payload.image();
                    if let Some(image_rg12) = image {
                        let image = &metal_context.debayer(
                            image_rg12,
                            image_info.width as usize,
                            image_info.height as usize,
                            true,
                        );
                        crate::undistort_image_table(
                            image,
                            &mut undistorted_image,
                            &precompute,
                            NEW_WIDTH,
                            NEW_HEIGHT,
                        );
                        let undistorted_clone = undistorted_image.clone();
                        let ball_comp_t0 = std::time::Instant::now();
                        // todo rever this
                        subtract_image(&mut undistorted_image, raw_image);
                        // let u_image = DynamicImage::ImageRgb8(
                        // ImageBuffer::from_raw(NEW_WIDTH, NEW_HEIGHT, undistorted_image.clone())
                        // .unwrap(),
                        // );
                        // u_image.save("undistorted_image.png");
                        // println!("subtracted image: {:?}", undistorted_image);
                        // std::process::exit(0);
                        let ball = crate::ball::find_ball(
                            undistorted_image.as_slice(),
                            NEW_WIDTH as u32,
                            NEW_HEIGHT as u32,
                            &mut ball_comp,
                            elapsed.as_secs_f32(),
                            &selection_fn,
                            min_radius,
                            max_radius,
                        );
                        let elapsed_ball_comp = ball_comp_t0.elapsed();
                        if elapsed_ball_comp.as_secs_f32() * 1000.0 > 5. {
                            println!(
                                "warning: ball_comp: {:.2}ms",
                                elapsed_ball_comp.as_secs_f32() * 1000.0
                            );
                        }

                        if ball_comp.velocity.x < 0.0 && ball_comp.velocity.magnitude() > 20. {
                            // the ball goes towards the goal
                            let intersection = ball_comp.intersection_x(self.player_x_coord as f32);
                            if let Some(intersection) = intersection {
                                // println!("intersection: {:?}", intersection);
                                let t = intersection.1;
                                let prepone = TIMING_OFFSET.load(Ordering::Relaxed); // default 0.25s
                                if t > prepone && ball_comp.velocity.magnitude() > 50.0 {
                                    // println!("t: {}, v: {}, t0: {}, t0 + t: {}", t, ball_comp.velocity.magnitude(), t0.elapsed().as_secs_f32(), t0.elapsed().as_secs_f32() + t);
                                    shoot_time = t0.elapsed().as_secs_f32() + t - prepone;
                                    shot = false;
                                }
                                // println!("t: {t}, shoot_time: {}", shoot_time);
                                let intersection = intersection.0;
                                if !FOLLOWBALL.load(Ordering::Relaxed) {
                                    move_y(
                                        intersection.y,
                                        &mut arduino_com,
                                        &mut last_command,
                                        min_pixel,
                                        max_pixel,
                                    );

                                    time_since_catch = Instant::now();
                                    moved_to_center = false;
                                }
                            } else if time_since_catch.elapsed().as_secs_f32() > 0.2
                                && !moved_to_center
                            {
                                if !FOLLOWBALL.load(Ordering::Relaxed) {
                                    move_center(
                                        &mut arduino_com,
                                        &mut last_command,
                                        min_pixel,
                                        max_pixel,
                                    );
                                }
                                moved_to_center = true;
                            }
                        } else if time_since_catch.elapsed().as_secs_f32() > 0.2 && !moved_to_center
                        {
                            if !FOLLOWBALL.load(Ordering::Relaxed) {
                                move_center(
                                    &mut arduino_com,
                                    &mut last_command,
                                    min_pixel,
                                    max_pixel,
                                );
                            }
                            moved_to_center = true;
                        }

                        // shoot
                        if !shot
                            && t0.elapsed().as_secs_f32() > shoot_time
                            && !PAUSESHOOTING.load(Ordering::Relaxed)
                            && !PAUSEPLAYER.load(Ordering::Relaxed)
                        {
                            arduino_com.send_string("S");
                            // println!("Shot!");
                            shot = true;
                        }

                        let y = ball.1 as f32;
                        if y > min_pixel as f32
                            && y < max_pixel as f32
                            && last_command.elapsed().as_secs_f32() > 0.05
                        {
                            // convert from pixel y to motor
                            // let m = (MIN_MOTOR - MAX_MOTOR) as f32 / (MAX_PIXEL - MIN_PIXEL) as f32;
                            // let b = MAX_MOTOR as f32 - m * MIN_PIXEL as f32;
                            // let x = m * y + b;
                            // println!("sending: y: {y}");
                            // arduino_com.send_string(&format!("{}", x as i32));
                            // last_command = Instant::now();
                        }
                        if FOLLOWBALL.load(Ordering::Relaxed) {
                            move_y(
                                ball.1 as f32,
                                &mut arduino_com,
                                &mut last_command,
                                min_pixel,
                                max_pixel,
                            );
                        }
                        if frame_counter % 1 == 0 {
                            // save to IMAGE_BUFFER
                            let mut buffer = IMAGE_BUFFER.lock().unwrap();
                            buffer.clear();
                            buffer.extend_from_slice(image);
                            drop(buffer);

                            let mut buffer = IMAGE_BUFFER_UNDISTORTED.lock().unwrap();
                            buffer.2.clear();
                            buffer.2.extend_from_slice(&undistorted_clone);
                        }
                    }
                }
            }
        });

        tx
    }
}
