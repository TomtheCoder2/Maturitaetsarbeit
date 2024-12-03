fn main() {
    let mut com = matura::arduino_com::ArduinoCom::new();
    let mut i = 0;
    let t0 = std::time::Instant::now();
    loop {
        com.send_string("10");
        i += 1;
        if i % 1000 == 0 {
            let t1 = std::time::Instant::now();
            let elapsed = t1.duration_since(t0);
            println!("Elapsed: {:?}, per iteration: {:?}", elapsed, elapsed / i);
        }
    }
}
