fn main() {
    let mut arduino_com = matura::arduino_com::ArduinoCom::new();
    let mut pos = 100;
    loop {
        arduino_com.send_string(&format!("{}", pos % 330));
        pos += 5;
        //     sleep 150 ms
        std::thread::sleep(std::time::Duration::from_millis(150));
    } // loop {}
}
