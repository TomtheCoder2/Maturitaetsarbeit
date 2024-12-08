#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello world!");

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d13.into_output();

    let number: i32 = 65;
    ufmt::uwriteln!(&mut serial, "Number: {}", number);
    if number > 64 {
        ufmt::uwriteln!(&mut serial, "Number is greater than 64");
    } else {
        ufmt::uwriteln!(&mut serial, "Number is less than 64");
    }

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
