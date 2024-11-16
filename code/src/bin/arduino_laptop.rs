fn main() {
    let mut arduino_com = matura::arduino_com::ArduinoCom::new();
    arduino_com.send_string("100");
    // loop {}
}
