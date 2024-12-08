#![no_std]
#![no_main]

use arduino_hal::prelude::_embedded_hal_serial_Read;
use arduino_hal::prelude::_embedded_hal_serial_Write;
use arduino_hal::simple_pwm::IntoPwmPin;
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::simple_pwm::Timer5Pwm;
use com::commands::Command;
use com::FixedPoint;
use nb::block;
use panic_halt as _;

use arduino_code::receive_command;
use arduino_code::{send_command, DCMotor};

// Import the heapless Vec

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello world!");
    loop {
        match receive_command!(serial) {
            Command::Stop => {
                break;
            }
            _ => {}
        }

        let mut buffer = [420i32; 1000];
        // ufmt::uwrite!(&mut serial, "{}", byte);
        serial.write(3u8).expect("Failed to write");
        // write length of buffer
        for byte in (buffer.len() as u32).to_le_bytes() {
            serial.write(byte).expect("Failed to write length");
        }
        for byte in buffer.iter() {
            if *byte == 0 {
                continue;
            }
            // ufmt::uwrite!(&mut serial, "{}", byte);
            // let data = byte.to_le_bytes();
            serial.write(*byte as u8).expect("Failed to write");
        }
    }
    // // let command = Command::Data(buffer);
    // let command = Command::Pos(43);
    // ufmt::uwriteln!(&mut serial, "creating array");
    // let mut buffer = [0u8; 4005];
    // ufmt::uwriteln!(&mut serial, "created array");
    // // let mut index = 0;
    // // match command {
    // //     Command::Pos(val) => {
    // //         buffer[index] = 0; // Identifier for Pos
    // //         index += 1;
    // //         buffer[index..index + 4].copy_from_slice(&val.to_le_bytes());
    // //         index += 4;
    // //     }
    // //     _ => {}
    // // }
    // let encoded = command.encode();
    // ufmt::uwriteln!(&mut serial, "encoded data: ");
    // for byte in encoded.iter() {
    //     if *byte == 0 {
    //         continue;
    //     }
    //     ufmt::uwriteln!(&mut serial, "{}", byte);
    // }
    loop {}
}
