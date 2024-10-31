//! Blinks an LED
//!
//! The following wiring is assumed:
//! - LED => GPIO15
//! https://github.com/espressif/arduino-esp32/blob/d47771f2cc649c3cd52a3f6eb3d9b97c82005ffb/variants/XIAO_ESP32C6/pins_arduino.h#L13


#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output},
    prelude::*,
};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Set GPIO0 as an output, and set its state high initially.
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio15, Level::High);

    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(500);
        led.toggle();
        // or using `fugit` duration
        delay.delay(2.secs());
    }
}
