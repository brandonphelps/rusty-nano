// using arduino nano 33 iot and MCP9808 report out current temperature. 
// overall goal is minimize the amount of external libraries used to 
// provide an understanding of lower level systems and provide
// hal api design. 

#![no_std]
#![no_main]
#![feature(format_args_nl)]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;

use arduino_nina::{Peripherals, CorePeripherals, GenericClockController, bsp, Delay};
use arduino_nina::entry;

use arduino_nina::{timer, sercom0, console};
use arduino_nina::*;

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

    // used to do uart configuration.
    let uart = bsp::uart(
        &mut clocks,
        9600.hz(),
        peripherals.SERCOM5,
        &mut peripherals.PM,
        pins.rx,
        pins.tx,
    );

    timer::timer().delay(1000);
    timer::timer().delay(1000);
    console::console_init(uart);

    let mut uuart = sercom0::Uart::new();
    for i in 33u16..124u16 {
        uuart.write_char(i).unwrap();
    }

    let mut i = 0;
    while i < 10 {
        timer::timer().delay(200);
        led.set_low().unwrap();
        timer::timer().delay(200);
        // println!("Hello World from within arduino rust with globals");
        uuart.write_char(65).unwrap();
        uuart.write_char(b'\n' as u16).unwrap();
        led.set_high().unwrap();
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
