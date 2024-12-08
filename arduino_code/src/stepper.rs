#![no_std]
#![no_main]

use core::time::Duration;
use embedded_hal::digital::OutputPin;

pub struct Stepper<P1, P2, P3, P4> {
    step_number: i32,         // Which step the motor is on
    direction: i32,           // Motor direction
    last_step_time: Duration, // Timestamp in microseconds of the last step taken
    number_of_steps: i32,     // Total number of steps for this motor
    step_delay: Duration,     // Delay between steps in microseconds
    motor_pin0: P1,           // Motor control pins
    motor_pin1: P2,
    motor_pin2: P3,
    motor_pin3: P4,
}

impl<P1: OutputPin, P2: OutputPin, P3: OutputPin, P4: OutputPin> Stepper<P1, P2, P3, P4> {
    // Constructor for four-wire stepper motor
    pub fn new_4_pins(
        number_of_steps: i32,
        motor_pin_0: P1,
        motor_pin_1: P2,
        motor_pin_2: P3,
        motor_pin_3: P4,
    ) -> Self {
        Stepper {
            step_number: 0,
            direction: 0,
            last_step_time: Duration::new(0, 0),
            number_of_steps,
            step_delay: Duration::new(0, 0),
            motor_pin0: motor_pin_0,
            motor_pin1: motor_pin_1,
            motor_pin2: motor_pin_2,
            motor_pin3: motor_pin_3,
        }
    }

    // Sets the speed in revs per minute
    pub fn set_speed(&mut self, what_speed: i32) {
        self.step_delay =
            Duration::from_micros((60 * 1_000_000 / self.number_of_steps / what_speed) as u64);
    }

    // Moves the motor steps_to_move steps. If the number is negative, the motor moves in the reverse direction.
    pub fn step(&mut self, steps_to_move: i32) {
        let mut steps_left = steps_to_move.abs();

        // Set the direction
        self.direction = if steps_to_move > 0 { 1 } else { 0 };

        while steps_left > 0 {
            // let now = micros();
            // wait self.step_delay
            arduino_hal::delay_us(self.step_delay.as_micros() as u32);

            // Move only if the appropriate delay has passed
            // if now - self.last_step_time >= self.step_delay {
            // self.last_step_time = now;

            // Increment or decrement the step number, depending on direction
            self.step_number = if self.direction == 1 {
                (self.step_number + 1) % self.number_of_steps
            } else {
                if self.step_number == 0 {
                    self.number_of_steps - 1
                } else {
                    self.step_number - 1
                }
            };

            // Decrement steps left
            steps_left -= 1;

            // Step the motor to the appropriate step
            self.step_motor(self.step_number % 4);
            // } else {
            //     // yield();
            // }
        }
        self.stop();
    }

    // Controls the motor's stepping for each pin configuration
    fn step_motor(&mut self, this_step: i32) {
        match this_step {
            0 => {
                digital_write(&mut self.motor_pin0, true);
                digital_write(&mut self.motor_pin1, false);
                digital_write(&mut self.motor_pin2, true);
                digital_write(&mut self.motor_pin3, false);
            }
            1 => {
                digital_write(&mut self.motor_pin0, false);
                digital_write(&mut self.motor_pin1, true);
                digital_write(&mut self.motor_pin2, true);
                digital_write(&mut self.motor_pin3, false);
            }
            2 => {
                digital_write(&mut self.motor_pin0, false);
                digital_write(&mut self.motor_pin1, true);
                digital_write(&mut self.motor_pin2, false);
                digital_write(&mut self.motor_pin3, true);
            }
            3 => {
                digital_write(&mut self.motor_pin0, true);
                digital_write(&mut self.motor_pin1, false);
                digital_write(&mut self.motor_pin2, false);
                digital_write(&mut self.motor_pin3, true);
            }
            _ => {}
        }
    }

    pub fn stop(&mut self) {
        digital_write(&mut self.motor_pin0, false);
        digital_write(&mut self.motor_pin1, false);
        digital_write(&mut self.motor_pin2, false);
        digital_write(&mut self.motor_pin3, false);
    }
}

fn digital_write<P: OutputPin>(p: &mut P, value: bool) {
    if value {
        p.set_high();
    } else {
        p.set_low();
    }
}
