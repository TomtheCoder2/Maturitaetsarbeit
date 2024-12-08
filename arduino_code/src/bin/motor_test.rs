#![no_std]
#![no_main]

use arduino_hal::simple_pwm::IntoPwmPin;
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::simple_pwm::Timer5Pwm;
use panic_halt as _;

// // Motor A connections
// int enA = 48;
// int in1 = 50;
// int in2 = 51;
// // Motor B connections
// int enB = 49;
// int in3 = 52;
// int in4 = 53;
//
// void setup() {
// 	// Set all the motor control pins to outputs
// 	pinMode(enA, OUTPUT);
// 	pinMode(enB, OUTPUT);
// 	pinMode(in1, OUTPUT);
// 	pinMode(in2, OUTPUT);
// 	pinMode(in3, OUTPUT);
// 	pinMode(in4, OUTPUT);
//
// 	// Turn off motors - Initial state
// 	digitalWrite(in1, LOW);
// 	digitalWrite(in2, LOW);
// 	digitalWrite(in3, LOW);
// 	digitalWrite(in4, LOW);
//   Serial.begin(57600);
//   Serial.print("test");
// }
//
// void loop() {
// 	directionControl();
// 	delay(1000);
// 	speedControl();
// 	delay(1000);
// }
//
// // This function lets you control spinning direction of motors
// void directionControl() {
// 	// Set motors to maximum speed
// 	// For PWM maximum possible values are 0 to 255
// 	analogWrite(enA, 255);
// 	analogWrite(enB, 255);
//
// 	// Turn on motor A & B
// 	digitalWrite(in1, HIGH);
// 	digitalWrite(in2, LOW);
// 	digitalWrite(in3, HIGH);
// 	digitalWrite(in4, LOW);
// 	delay(2000);
//
// 	// Now change motor directions
// 	digitalWrite(in1, LOW);
// 	digitalWrite(in2, HIGH);
// 	digitalWrite(in3, LOW);
// 	digitalWrite(in4, HIGH);
// 	delay(2000);
//
// 	// Turn off motors
// 	digitalWrite(in1, LOW);
// 	digitalWrite(in2, LOW);
// 	digitalWrite(in3, LOW);
// 	digitalWrite(in4, LOW);
// }
//
// // This function lets you control speed of the motors
// void speedControl() {
// 	// Turn on motors
// 	digitalWrite(in1, LOW);
// 	digitalWrite(in2, HIGH);
// 	digitalWrite(in3, LOW);
// 	digitalWrite(in4, HIGH);
//
// 	// Accelerate from zero to maximum speed
// 	for (int i = 0; i < 256; i++) {
// 		analogWrite(enA, i);
// 		analogWrite(enB, i);
// 		delay(20);
// 	}
//
// 	// Decelerate from maximum speed to zero
// 	for (int i = 255; i >= 0; --i) {
// 		analogWrite(enA, i);
// 		analogWrite(enB, i);
// 		delay(20);
// 	}
//
// 	// Now turn off motors
// 	digitalWrite(in1, LOW);
// 	digitalWrite(in2, LOW);
// 	digitalWrite(in3, LOW);
// 	digitalWrite(in4, LOW);
// }

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
    // Turn off motors - Initial state
    in1.set_low();
    in2.set_low();
    in3.set_low();
    in4.set_low();
    let mut i = 0;

    ufmt::uwriteln!(&mut serial, "peripherals");
    // Get access to the device peripherals
    ufmt::uwriteln!(&mut serial, "peripherals");
    let mut pwm_timer5 = Timer5Pwm::new(dp.TC5, Prescaler::Prescale64);

    ufmt::uwriteln!(&mut serial, "pins");
    // let pins_copy = arduino_hal::pins!(dp);
    let mut en_a = pins.d44.into_output().into_pwm(&mut pwm_timer5);
    en_a.enable();
    let mut en_b = pins.d46.into_output().into_pwm(&mut pwm_timer5);
    en_b.enable();
    loop {
        // Set motors to maximum speed
        // For PWM maximum possible values are 0 to 255
        // ufmt::uwriteln!(&mut serial, "setting a and b high");
        en_a.set_duty(255);
        en_b.set_duty(255);
        // ufmt::uwriteln!(&mut serial, "set a and b high");
        // Turn on motor A & B
        in1.set_high();
        in2.set_low();
        in3.set_high();
        in4.set_low();
        arduino_hal::delay_ms(2000);
        // Now change motor directions
        in1.set_low();
        in2.set_high();
        in3.set_low();
        in4.set_high();
        arduino_hal::delay_ms(2000);
        // Turn off motors
        in1.set_low();
        in2.set_low();
        in3.set_low();
        in4.set_low();
        // Turn on motors
        in1.set_low();
        in2.set_high();
        in3.set_low();
        in4.set_high();
        // // Accelerate from zero to maximum speed
        for i in 0..256 {
            en_a.set_duty(i as u8);
            en_b.set_duty(i as u8);
            // ufmt::uwriteln!(&mut serial, "i: {}", i);
            arduino_hal::delay_ms(20);
        }
        // Decelerate from maximum speed to zero
        for i in (0..256).rev() {
            en_a.set_duty(i as u8);
            en_b.set_duty(i as u8);
            arduino_hal::delay_ms(20);
        }
        // Now turn off motors
        in1.set_low();
        in2.set_low();
        in3.set_low();
        in4.set_low();
    }
}

//     /*
//      * For examples (and inspiration), head to
//      *
//      *     https://github.com/Rahix/avr-hal/tree/main/examples
//      *
//      * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
//      * for a different board can be adapted for yours.  The Arduino Uno currently has the most
//      * examples available.
//      */
//
//     let mut led = pins.d13.into_output();
//
//     loop {
//         led.toggle();
//         arduino_hal::delay_ms(1000);
//     }
// }
