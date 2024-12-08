#![no_std]
#![no_main]

extern crate panic_halt;

use core::hash::Hasher;

use arduino_hal::prelude::*;
use arduino_hal::{default_serial, Peripherals};

#[arduino_hal::entry]
fn main() -> ! {
    // wait 2seconds
    // arduino_hal::delay_ms(2000);
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();
    // wait for 'a' to start
    loop {
        if let Ok(byte) = block!(serial.read()) {
            if byte == b'a' {
                break;
            }
        }
    }
    ufmt::uwriteln!(&mut serial, "Hello!");
    loop {
        led.set_low();
        // ufmt::uwriteln!(&mut serial, "Waiting for start command...").unwrap();
        if let Ok(byte) = serial.read() {
            if byte != b'a' {
                break;
            }
            led.set_high();
            // ufmt::uwriteln!(&mut serial, "Received: {}", byte).unwrap();
            // arduino_hal::delay_ms(5000);
            // Create an array of 100 integers
            let data: [u8; 100] = [0; 100];
            for i in 0..100 {
                ufmt::uwriteln!(&mut serial, "{}", i).unwrap();
                // serial.write(&[i as u8]);
                // match (&(i)) {
                //     ( __0   ) => {
                //         use ufmt::UnstableDoAsFormatter   as _;
                //         (&mut serial).do_as_formatter(|f| {
                //             // ufmt::uDisplay::fmt(__0, f)?;
                //             // f.write_str("\n")?;
                //             f.write_u8(i as u8);
                //             core::result::Result::Ok(())
                //         })
                //     }
                // };
            }

            // Indicate that data transmission is complete
            ufmt::uwriteln!(&mut serial, "END").unwrap();
            led.set_high();
        }
    }

    loop {}
}
