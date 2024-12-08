#![no_std]
// #![feature(unchecked_math)]

// #![no_main]

use com::commands::Command;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::pwm::SetDutyCycle;
use embedded_hal::spi::Polarity;

use crate::pid::PIDController;

pub mod pid;
pub mod stepper;

pub struct Arduino {}

pub struct DCMotor<EN, IN1, IN2, EncoderA, EncoderB> {
    en: EN,
    in1: IN1,
    in2: IN2,
    pub encoder_a: EncoderA,
    pub encoder_b: EncoderB,
    pub last_a: bool,
    pub last_b: bool,
    pub position: i32,
    speed: u8,
    polarity: Polarity,
    start_pos: i32,
    pub target_pos: i32,
    pub last_pos: i32,
    pub pid: PIDController,
}

impl<EN: SetDutyCycle, IN1: OutputPin, IN2: OutputPin, EncoderA: InputPin, EncoderB: InputPin>
    DCMotor<EN, IN1, IN2, EncoderA, EncoderB>
{
    pub fn new(
        en: EN,
        in1: IN1,
        in2: IN2,
        mut encoder_a: EncoderA,
        mut encoder_b: EncoderB,
    ) -> Self {
        let a = encoder_a.is_high().unwrap_or_else(|_| false);
        let b = encoder_b.is_high().unwrap_or_else(|_| false);
        DCMotor {
            en,
            in1,
            in2,
            encoder_a,
            encoder_b,
            last_a: a,
            last_b: b,
            position: 0,
            speed: 0,
            polarity: Polarity::IdleLow,
            start_pos: 0,
            target_pos: 0,
            last_pos: 0,
            pid: PIDController::new(35536 * 1, 0, 000, 16),
        }
    }

    pub fn stop(&mut self) {
        self.in1.set_low().expect("Failed to set low");
        self.in2.set_low().expect("Failed to set low");
    }

    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed;
        self.en
            .set_duty_cycle(speed as u16)
            .expect("Failed to set duty cycle");
    }

    pub fn set_polarity(&mut self, polarity: Polarity) {
        self.polarity = polarity;
    }

    pub fn switch_polarity(&mut self) {
        self.polarity = match self.polarity {
            Polarity::IdleLow => Polarity::IdleHigh,
            Polarity::IdleHigh => Polarity::IdleLow,
        };
        self.run();
    }

    pub fn run(&mut self) {
        if self.polarity == Polarity::IdleLow {
            self.in1.set_high().expect("Failed to set high");
            self.in2.set_low().expect("Failed to set low");
        } else {
            self.in1.set_low().expect("Failed to set low");
            self.in2.set_high().expect("Failed to set high");
        }
    }

    pub fn get_a_b(&mut self) -> (bool, bool) {
        let a = self.encoder_a.is_high().unwrap_or_else(|_| false);
        let b = self.encoder_b.is_high().unwrap_or_else(|_| false);
        (a, b)
    }

    pub fn count_pos(&mut self, a_b: (bool, bool)) -> i32 {
        let a = a_b.0;
        let b = a_b.1;
        // check if something has changed
        if a != self.last_a || b != self.last_b {
            if (a as i32 - self.last_a as i32) == 0 {
                // check if b has changed from 0 to 1 or vice versa
                self.position += 1
                    * if a { 1 } else { -1 }
                    * if (b as i32 - self.last_b as i32) == 1 {
                        1
                    } else {
                        -1
                    };
            } else {
                // no change in a
                self.position -= 1 * if b { 1 } else { -1 } * (a as i32 - self.last_a as i32);
            }
            self.last_a = a;
            self.last_b = b;
        }
        self.position
    }

    pub fn run_to_relative_pos(&mut self, target_pos: i32) {
        self.start_pos = self.position;
        self.target_pos = self.start_pos + target_pos;
        self.run();
    }

    pub fn run_to_relative_pos_step(&mut self) -> i32 {
        // get error
        let ab = self.get_a_b();
        let current_pos = self.count_pos(ab);
        if current_pos - self.last_pos == 0 {
            return 0;
        }
        // get error
        let error = current_pos - self.target_pos;
        let speed = self.pid.update(error, 66).abs().min(255).max(50);
        self.set_speed(speed as u8);
        // self.set_speed(0);
        // check if polarity is correct by looking if the delta between current_pos and self.last_pos in the same direction as the error
        let mut res = false;
        if current_pos - self.last_pos != 0
            && (current_pos - self.last_pos).signum() != error.signum()
        {
            // self.run();
        } else {
            self.switch_polarity();
            res = true;
        }
        self.last_pos = current_pos;
        error
    }
}

pub fn test() {}

#[macro_export]
macro_rules! send_command {
    ($serial:ident, $command:expr) => {{
        let encoded = $command.encode();
        let length = Command::length_c(&$command);
        // 5:
        // 0 50 0 0 0
        // 0 1  2 3 4 5
        // filter out all 0s
        // let encoded = encoded.iter().filter(|x| **x != 0).map(|x| *x).collect::<heapless::Vec<u8, length>>();
        //$seria// l.write(encoded.as_slice()).expect("Failed to write to port");
        let mut i = 0;
        for byte in encoded {
            // if byte == 0 {
            //     continue;
            // }
            if i >= length {
                break;
            }
            $serial.write(byte).expect("Failed to write to port");
            i += 1;
        }
    }};
}

#[macro_export]
macro_rules! receive_command {
    ($serial:ident) => {{
        let ty = nb::block!($serial.read()).unwrap();
        let total_length = Command::length(ty);
        // print received bytes
        // ufmt::uwriteln!($serial, "Received: {}\r", ty).unwrap();
        let mut buffer = [ty; 20];
        for i in 1..total_length {
            buffer[i] = nb::block!($serial.read()).unwrap();
        }
        Command::decode(&buffer).unwrap()
    }};
}

#[macro_export]
macro_rules! try_receive_command {
    ($serial:ident) => {{
        let ty = $serial.read();
        if ty.is_err() {
            None
        } else {
            let ty = ty.unwrap();
            let total_length = Command::length(ty);
            // print received bytes
            // ufmt::uwriteln!($serial, "Received: {}\r", ty).unwrap();
            let mut buffer = [ty; 20];
            for i in 1..total_length {
                buffer[i] = nb::block!($serial.read()).unwrap();
            }
            Some(Command::decode(&buffer).unwrap())
        }
    }};
}
