#![no_std]
#![no_main]
#![feature(format_args_nl)]

use core::fmt;

use arduino_nano33iot as bsp;
use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;

use bsp::entry;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

mod console;
mod generic;
mod sercom0;
mod syncronization;
mod timer;
pub(crate) mod ring_buffer;

use generic::{Readable, Reg, ResetValue, Writeable, R, W};

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    // todo: handle result.
    console::console().write_fmt(args);
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        _print(format_args_nl!($($arg)*));
    })
}

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

#[entry]
fn main() -> ! {
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

    /*
    let spi = bsp::spi_master(
        &mut clocks,
        MegaHertz(16),
        peripherals.SERCOM1,
        &mut peripherals.PM,
        pins.led_sck,
        pins.mosi,
        pins.miso);
    */

    let mut uart = bsp::uart(
        &mut clocks,
        9600.hz(),
        peripherals.SERCOM5,
        &mut peripherals.PM,
        pins.rx,
        pins.tx,
    );

    timer::timer().delay(1000);
    // unsafe { uart.write_data(65); }
    timer::timer().delay(1000);

    let usart = unsafe { sercom0::SERCOM5::ptr().as_ref().unwrap().usart() };

    console::console_init(uart);

    let mut uuart = sercom0::Uart::new();
    // uuart.disable();
    // timer::timer().delay(200);
    // uuart.enable();

    // println!("Custom write");
    for i in 33u16..124u16 {
        uuart.write_char(i);
        // timer::timer().delay(200);
    }
//    println!("After write");

    let mut i = 0;
    while i < 10 {
        timer::timer().delay(200);
        led.set_low().unwrap();
        timer::timer().delay(200);
        // println!("Hello World from within arduino rust with globals");
        uuart.write_char(65);
        uuart.write_char(b'\n' as u16);
        led.set_high().unwrap();

        /*
        for ch in 33..127 {
            uart.write(ch).unwrap();
            //delay.delay_ms(500u16);
        }
         */
        i += 1;
    }

    loop {
        timer::timer().delay(200);
        led.set_low().unwrap();
        uuart.maintain();
        timer::timer().delay(200);
        led.set_high().unwrap();
    }
}
