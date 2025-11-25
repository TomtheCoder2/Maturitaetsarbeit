use matura::arduino_com::ArduinoCom;
use matura::plot::PlotApp;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::thread;
use std::time::Duration;

fn main() {
    let mut com = ArduinoCom::new();
    let mut positions = vec![800, 200, 250, 280, 400, 180, 900, 300];
    // insert a 500 before every element
    for i in (0..positions.len()).rev() {
        positions.insert(i, 500);
    }
    positions.push(500);
    let mut sleep_duration = Duration::from_millis(0);
    let n = 2;
    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(err) => {
            println!("Error initializing readline editor: {:?}", err);
            return;
        }
    };
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        // get pid from command line
        // ask for pid vars
        println!("input pid variables (P I D), or 'exit' to quit:");
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                if line.trim() == "exit" {
                    break;
                }
                let vars: Vec<&str> = line.trim().split_whitespace().collect();
                if vars.len() != 3 {
                    println!("Please input 3 values for P I D");
                    continue;
                }
                macro_rules! parse_var {
                    ($var:expr) => {
                        match $var.parse() {
                            Ok(val) => val,
                            Err(_) => {
                                println!("Invalid value for PID variable");
                                continue;
                            }
                        }
                    };
                }
                let pid = (
                    parse_var!(vars[0]),
                    parse_var!(vars[1]),
                    parse_var!(vars[2]),
                );
                println!("Setting PID to: P={}, I={}, D={}", pid.0, pid.1, pid.2);
                com.set_pid(pid.0, pid.1, pid.2);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }

        sleep_duration = Duration::from_millis(0);
        let mut total_diff = 0;
        let mut total_time = 0.0;
        let mut actual_positions = vec![];
        // let mut target_positions = vec![];
        // let mut pulse_widths: Vec<(f64, f64)> = vec![];
        let t00 = std::time::Instant::now();
        for _ in 0..n {
            for pos in positions.iter() {
                com.sync();
                com.send_string(&format!("{}", pos).to_string());
                // thread::sleep(Duration::from_millis(1000));
                // let t0 = std::time::Instant::now();
                // let mut actual_pos: i32 = 0;
                // com.sync();
                // while actual_pos.abs_diff(*pos) > 5 && t0.elapsed().as_secs_f32() < 1. {
                //     actual_pos = com.get_pos_sync(true) as i32;
                //     let t = t00.elapsed().as_secs_f64();
                //     actual_positions.push((t, actual_pos as f64));
                //     target_positions.push((t, *pos as f64));
                //     // let pw = com.get_pulse_width();
                //     // pulse_widths.push((t, pw as f64));
                //     thread::sleep(Duration::from_millis(5));
                // }
                let line = com.read_line();
                // should be t: 400 for 400ms
                // println!("line: {:?}", line);
                let time_taken = line
                    .trim()
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<f64>()
                    .unwrap()
                    / 1000.0;
                let counter = line
                    .trim()
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                let actual_pos = com.get_pos_sync(true) as i32;
                actual_positions.push((t00.elapsed().as_secs_f64(), actual_pos as f64));
                let actual_pos = com.get_pos() as i32;
                println!(
                    "sent {}, actual pos: {}, diff: {} in {:.3}s with {} steps, ie. actual avg delay: {:.3}us",
                    pos,
                    actual_pos,
                    actual_pos.abs_diff(*pos),
                    time_taken,
                    counter,
                    time_taken * 1_000_000.0 / counter as f64
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
        // let target_positions: Vec<[f64; 2]> = target_positions.iter().map(|p| [p.0, p.1]).collect();
        // let pulse_widths: Vec<[f64; 2]> = pulse_widths.iter().map(|p| [p.0, p.1]).collect();

        // plot_main(actual_positions, target_positions, pulse_widths);
    }
    rl.save_history("history.txt").expect("TODO: panic message");
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
