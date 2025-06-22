use std::thread::sleep;
use std::time::Duration;
use matura::arduino_com::ArduinoCom;

// Example main function for testing
fn main() {
    let mut arduino_com = ArduinoCom::new(); // This will now include the 3-second boot delay

    // Keep using the simple "Ping!" Arduino sketch for initial testing to confirm stability
    // Arduino sketch:
    // void setup() { Serial.begin(57600); while (!Serial); Serial.println("Arduino: Ready!"); }
    // void loop() { Serial.println("Arduino: Ping!"); delay(1000); }

    println!("Starting continuous read from Arduino...");
    loop {
        // match arduino_com.read_line() {
        //     s if !s.is_empty() => println!("Received: '{}'", s),
        //     _ => {
        //         println!("Received an empty line (likely only \\r\\n was sent)");
        //     }
        // }
        arduino_com.sync();
        // You might want a small sleep here in the loop if the Arduino sends very slowly
        // to avoid constant timeouts. Not strictly necessary with 1-sec ping.
        sleep(Duration::from_millis(1000));
    }

    let mut arduino_com = matura::arduino_com::ArduinoCom::new();
    let mut pos = 100;
    arduino_com.sync();
    loop {
        arduino_com.send_string(&format!("{}", pos % 330));
        pos += 5;
        //     sleep 150 ms
        std::thread::sleep(std::time::Duration::from_millis(150));
    } // loop {}
}
