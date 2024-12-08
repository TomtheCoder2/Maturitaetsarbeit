#![no_std]
#![no_main]

use arduino_hal::prelude::_embedded_hal_serial_Read;
use arduino_hal::simple_pwm::IntoPwmPin;
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::simple_pwm::Timer5Pwm;
use com::commands::Command;
use com::FixedPoint;
use nb::block;
use panic_halt as _;

use arduino_code::{receive_command, try_receive_command};
use arduino_code::{send_command, DCMotor};

// Import the heapless Vec

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Starting stepper receiver");
    // Initialize stepper motor control pins
    let mut in1 = pins.d40.into_output();
    let mut in2 = pins.d41.into_output();
    let mut in3 = pins.d42.into_output();
    let mut in4 = pins.d43.into_output();

    let mut stepper = arduino_code::stepper::Stepper::new_4_pins(200, in1, in2, in3, in4);
    // stepper.set_speed(50);
    // stepper.step(150);
    stepper.set_speed(100);
    stepper.step(-100);

    loop {
        if let Command::Start = receive_command!(serial) {
            // ufmt::uwriteln!(&mut serial, "Received start command");
            break;
        }
    }
    use arduino_hal::prelude::_embedded_hal_serial_Write;
    // ufmt::uwriteln!(&mut serial, "Starting...");
    send_command!(serial, Command::Start);
    let mut current_pos = 100;
    let mut target_pos = 100;

    loop {
        let c = try_receive_command!(serial);
        if let Some(command) = c {
            // send_command!(serial, Command::Start);
            match command {
                Command::Pos(pos) => {
                    target_pos = pos;
                    stepper.step(-(target_pos - current_pos));
                    current_pos = target_pos;
                    // send_command!(serial, Command::Pos(current_pos));
                }
                Command::Speed(speed) => {
                    stepper.set_speed(speed);
                }
                Command::Reset(d) => {
                    stepper.step(-d);
                    current_pos = 0;
                }
                Command::Stop => {
                    stepper.step(0);
                    // send_command!(serial, Command::Stop);
                    break;
                }
                _ => {}
            }
        }
        // if current_pos != target_pos {
        //     let diff = target_pos - current_pos;
        //     let steps = if diff.abs() > 10 { 16 } else { 4 } * diff.signum();
        //     stepper.step(-steps);
        //     current_pos += steps;
        // }
    }
    loop {}
}
