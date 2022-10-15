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
