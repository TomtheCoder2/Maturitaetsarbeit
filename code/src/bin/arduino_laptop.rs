use matura::arduino_com::ArduinoCom;
use matura::plot::PlotApp;
use std::thread;
use std::time::Duration;

fn main() {
    let mut com = ArduinoCom::new();
    let positions = vec![500, 800, 200, 250, 280, 400, 180, 900, 300];
    let mut sleep_duration = Duration::from_millis(0);
    let n = 1;
    let mut total_diff = 0;
    let mut total_time = 0.0;
    let mut actual_positions = vec![];
    let mut target_positions = vec![];
    let mut pulse_widths: Vec<(f64, f64)> = vec![];
    let t00 = std::time::Instant::now();
    for _ in 0..n {
        for pos in positions.iter() {
            com.send_string(&format!("{}", pos).to_string());
            // thread::sleep(Duration::from_millis(1000));
            let t0 = std::time::Instant::now();
            let mut actual_pos: i32 = 0;
            com.sync();
            while actual_pos.abs_diff(*pos) > 5 && t0.elapsed().as_secs_f32() < 1. {
                actual_pos = com.get_pos_sync(true) as i32;
                let t = t00.elapsed().as_secs_f64();
                actual_positions.push((t, actual_pos as f64));
                target_positions.push((t, *pos as f64));
                // let pw = com.get_pulse_width();
                // pulse_widths.push((t, pw as f64));
                thread::sleep(Duration::from_millis(10));
            }
            let time_taken = t0.elapsed().as_secs_f64();
            let actual_pos = com.get_pos() as i32;
            println!(
                "sent {}, actual pos: {}, diff: {} in {:.2}s",
                pos,
                actual_pos,
                actual_pos.abs_diff(*pos),
                time_taken
            );
            total_diff += actual_pos.abs_diff(*pos);
            total_time += time_taken;
            // wait until key pressed
            // println!("Press Enter to continue...");
            // let mut input = String::new();
            // std::io::stdin().read_line(&mut input).unwrap();
            thread::sleep(sleep_duration);
        }
        sleep_duration += Duration::from_millis(200);
    }
    let average = total_diff as f32 / positions.len() as f32 / n as f32;
    // we have 1024 steps per 200mm
    let avg_mm = average * 200.0 / 1024.0;
    println!(
        "Total difference: {}, avg: {}, {} mm",
        total_diff, average, avg_mm
    );
    println!(
        "Total time: {:.2}s, avg time per move: {:.2}s",
        total_time,
        total_time / positions.len() as f64 / n as f64
    );
    let actual_positions: Vec<[f64; 2]> = actual_positions.iter().map(|p| [p.0, p.1]).collect();
    let target_positions: Vec<[f64; 2]> = target_positions.iter().map(|p| [p.0, p.1]).collect();
    let pulse_widths: Vec<[f64; 2]> = pulse_widths.iter().map(|p| [p.0, p.1]).collect();

    // plot_main(actual_positions, target_positions, pulse_widths);
}

fn plot_main(graph: Vec<[f64; 2]>, graph2: Vec<[f64; 2]>, graph3: Vec<[f64; 2]>) {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };
    // let graph: Vec<[f64; 2]> = vec![[0.0, 1.0], [2.0, 3.0], [3.0, 2.0]];
    // let graph2: Vec<[f64; 2]> = vec![];
    // let graph3: Vec<[f64; 2]> = vec![];

    eframe::run_native(
        "My egui App with a plot",
        options,
        Box::new(|_cc| {
            Ok(Box::new(PlotApp {
                insert_order: false,
                graph,
                graph2,
                graph3,
            }))
        }),
    )
    .unwrap()
}
