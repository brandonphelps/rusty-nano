
// #![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]


use core::panic::PanicInfo;

extern crate arduino_nina;
use arduino_nina::blink_led;
extern crate defmt_rtt;
use defmt_rtt as _;

use arduino_nina::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // todo: make it so we don't need to pull in all this extra stuff 
    // i.e configure it your self.
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

    arduino_nina::console::console_init(uart);

    //core::fmt::Write::write_fmt(&mut uart, *info.message().unwrap());
    if let Some(a) = info.message() {
        arduino_nina::print::_print(*a);
    }
    

    // attempt to get a uart handle for printing.
    let mut uart = arduino_nina::sercom0::Uart::new();

    if let Some(payload) = info.message() {
        if let Some(msg) = payload.as_str() {
            for i in msg.as_bytes().iter() {
                uart.write_char(*i as u16);
            }    
        } else { 
            for i in b"Failed to get payload string\n".iter() {
                uart.write_char(*i as u16);
            }
        }
    } else 
    {
        for i in b"Failed to determine panic reason\n".iter() {
            uart.write_char(*i as u16);
        }
    }
    blink_led()
}

struct State {
}


#[defmt_test::tests]
mod tests {
    use super::State;
    

    #[test]
    fn ground_is_low() {
        assert_eq!(1,2, "one != two");          
    }
}
