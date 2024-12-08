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

use arduino_code::receive_command;
use arduino_code::{send_command, DCMotor};

// Import the heapless Vec

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello world!");
    // let mut en_a = pins.d44.into_output();
    let mut in1 = pins.d50.into_output();
    let mut in2 = pins.d51.into_output();
    // let mut en_b = pins.d46.into_output();
    let mut in3 = pins.d52.into_output();
    let mut in4 = pins.d53.into_output();
    let mut led = pins.d13.into_output();

    // encoder stuff for first motor
    let mut encoder_a = pins.d24.into_pull_up_input();
    let mut encoder_b = pins.d26.into_pull_up_input();
    // Turn off motors - Initial state
    in1.set_low();
    in2.set_low();
    in3.set_low();
    in4.set_low();
    let mut i = 0;

    // ufmt::uwriteln!(&mut serial, "peripherals");
    // Get access to the device peripherals
    ufmt::uwriteln!(&mut serial, "peripherals");
    let mut pwm_timer5 = Timer5Pwm::new(dp.TC5, Prescaler::Prescale64);

    ufmt::uwriteln!(&mut serial, "pins");
    // let pins_copy = arduino_hal::pins!(dp);
    let mut en_a = pins.d44.into_output().into_pwm(&mut pwm_timer5);
    en_a.enable();
    let mut en_b = pins.d46.into_output().into_pwm(&mut pwm_timer5);
    en_b.enable();
    let mut current_pos = 0;
    let mut last_a = encoder_a.is_high();
    let mut last_b = encoder_b.is_high();
    let mut dc_motor = DCMotor::new(en_a, in1, in2, encoder_a, encoder_b);
    dc_motor.run();
    let mut i: u64 = 0;
    let mut pos_changes = 0;
    let mut last_pos = 0;
    dc_motor.run_to_relative_pos(640);

    led.set_low();
    loop {
        if let Command::Start = receive_command!(serial) {
            // ufmt::uwriteln!(&mut serial, "Received start command");
            break;
        }
    }
    use arduino_hal::prelude::_embedded_hal_serial_Write;
    // ufmt::uwriteln!(&mut serial, "Starting...");
    send_command!(serial, Command::Start);
    led.set_high();
    // let mut positions = [0; 1000];
    let mut last_pos: i32 = 0;
    let mut pos_i = -1;
    // dc_motor.set_speed(0);
    dc_motor.set_speed(255);
    let mut current_target_pos: i32 = 640;
    let mut target_pos_counter = 0;
    loop {
        if (last_pos - current_target_pos).abs() < 10 {
            if target_pos_counter > 100 {
                current_target_pos = -current_target_pos;
                dc_motor.run_to_relative_pos(current_target_pos);
                dc_motor.set_speed(255);
                target_pos_counter = 0;
            } else {
                target_pos_counter += 1;
            }
        } else {
            target_pos_counter = 0;
        }
        if pos_i == 1000 || pos_i == -1 {
            // // send_command!(serial, Command::Data(positions));
            // let encoded = Command::Data(positions).encode();
            // // filter out all 0s
            // // let encoded = encoded.iter().filter(|x| **x != 0).map(|x| *x).collect::<heapless::Vec<u8, length>>();
            // //$seria// l.write(encoded.as_slice()).expect("Failed to write to port");
            // for byte in encoded {
            //     if byte == 0 {
            //         continue;
            //     }
            //     // serial.write(byte).expect("Failed to write to port");
            // }
            serial.write_byte(3);
            1000u32
                .to_le_bytes()
                .iter()
                .for_each(|f| serial.write_byte(*f));
            pos_i = 0;
        }
        let error = dc_motor.run_to_relative_pos_step();

        let pos = dc_motor.position;
        if pos != last_pos {
            // positions[pos_i as usize] = pos;
            last_pos = pos;
            (pos as i32)
                .to_le_bytes()
                .iter()
                .for_each(|f| serial.write_byte(*f));
            pos_i += 1;
        }
        i += 1;
    }
    ufmt::uwriteln!(&mut serial, "Stopping motor: {}", pos_changes);
    dc_motor.stop();
    loop {}
}
