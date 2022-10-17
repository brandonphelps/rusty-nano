#![no_std]
#![feature(format_args_nl)]

#![deny(warnings)]

pub use arduino_nano33iot as bsp;
use bsp::hal;



pub use bsp::entry;

pub use hal::clock::GenericClockController;
pub use hal::delay::Delay;
pub use hal::pac::{CorePeripherals, Peripherals};
pub use hal::prelude::*;

pub mod console;
mod generic;
pub mod sercom0;
pub mod gclk;
mod syncronization;
pub mod timer;
pub mod ring_buffer;
pub mod print;

use generic::{Readable, Reg, ResetValue, Writeable, R, W, Variant};


pub use sercom0::Uart;

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


