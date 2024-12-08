#![no_std]
#![no_main]
use arduino_code::stepper;
use arduino_hal::delay_ms;
use arduino_hal::prelude::*;
use embedded_hal::digital::OutputPin;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Initialize stepper motor control pins
    let mut in1 = pins.d40.into_output();
    let mut in2 = pins.d41.into_output();
    let mut in3 = pins.d42.into_output();
    let mut in4 = pins.d43.into_output();

    let mut stepper = arduino_code::stepper::Stepper::new_4_pins(200, in1, in2, in3, in4);
    stepper.set_speed(100);

    // drive 50 forward then 50 backward
    loop {
        stepper.step(50);
        delay_ms(1000);
        stepper.step(-50);
        delay_ms(1000);
    }
}
