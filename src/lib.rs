#![no_std]
#![feature(format_args_nl)]

use core::fmt;

use arduino_nano33iot as bsp;
use bsp::hal;

use bsp::entry;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

mod timer;
mod syncronization;

pub fn blink_led() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led: bsp::Led = pins.led_sck.into();
    let delay = Delay::new(core.SYST, &mut clocks);
    timer::set_timer(delay);
    
    loop {
        timer::timer().delay(200);
        led.set_low().unwrap();
        timer::timer().delay(200);
        led.set_high().unwrap();
    }
}
