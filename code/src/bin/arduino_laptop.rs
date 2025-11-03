use std::thread;
use std::time::Duration;
use matura::arduino_com::ArduinoCom;

fn main() {
    let mut com = ArduinoCom::new();
    let positions = vec![500, 800, 200, 250, 280, 400, 180, 900, 300];
    let mut total_diff = 0;
    for pos in positions.iter() {
        com.send_string(&format!("{}", pos).to_string());
        thread::sleep(Duration::from_millis(1000));
        let actual_pos = com.get_pos() as i32;
        println!("sent {}, actual pos: {}, diff: {}", pos, actual_pos, actual_pos.abs_diff(*pos));
        total_diff += actual_pos.abs_diff(*pos);
        // wait until key pressed
        // println!("Press Enter to continue...");
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
    }
    let average = total_diff as f32 / positions.len() as f32; 
    // we have 1024 steps per 200mm
    let avg_mm = average * 200.0 / 1024.0;
    println!("Total difference: {}, avg: {}, {} mm", total_diff, average, avg_mm);
}