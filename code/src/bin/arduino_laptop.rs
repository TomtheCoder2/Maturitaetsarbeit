use std::thread;
use std::time::Duration;
use matura::arduino_com::ArduinoCom;

fn main() {
    let mut com = ArduinoCom::new();
    let positions = vec![500, 800, 200, 250, 280, 400, 180, 900, 300];
    let mut total_diff = 0;
    let mut total_time = 0.0;
    for pos in positions.iter() {
        com.send_string(&format!("{}", pos).to_string());
        // thread::sleep(Duration::from_millis(1000));
        let t0 = std::time::Instant::now();
        let mut actual_pos: i32 = 0;
        while actual_pos.abs_diff(*pos) > 5 && t0.elapsed().as_secs_f32() < 1. {
            actual_pos = com.get_pos() as i32;
            thread::sleep(Duration::from_millis(50));
        }
        let time_taken = t0.elapsed().as_secs_f32();
        let actual_pos = com.get_pos() as i32;
        println!("sent {}, actual pos: {}, diff: {} in {:.2}s", pos, actual_pos, actual_pos.abs_diff(*pos), time_taken);
        total_diff += actual_pos.abs_diff(*pos);
        total_time += time_taken;
        // wait until key pressed
        // println!("Press Enter to continue...");
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
    }
    let average = total_diff as f32 / positions.len() as f32; 
    // we have 1024 steps per 200mm
    let avg_mm = average * 200.0 / 1024.0;
    println!("Total difference: {}, avg: {}, {} mm", total_diff, average, avg_mm);
    println!("Total time: {:.2}s, avg time per move: {:.2}s", total_time, total_time / positions.len() as f32);
}