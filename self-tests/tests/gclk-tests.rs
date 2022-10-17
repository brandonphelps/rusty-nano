// #![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate arduino_nina;
use arduino_nina::blink_led;

extern crate defmt_rtt;
#[allow(unused)]
use defmt_rtt as _;

use arduino_nina::console::interface::Write;
use arduino_nina::*;

include! {"panic_handler.rs"}

#[defmt_test::tests]
mod tests {

    use super::*;

    #[init]
    fn test_setup() {
        // current code to setup the uart via the provided bsp.
        let mut peripherals = Peripherals::take().unwrap();
        let core = CorePeripherals::take().unwrap();
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );

        let pins = bsp::Pins::new(peripherals.PORT);
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
    }

    #[test]
    fn gclk_read() {
        let gclk_p = unsafe { gclk::GCLK::ptr() };

        let gclock_en = unsafe { gclk_p.as_ref().unwrap().genctrl.read().genen().bits() };
        let gclock_source = unsafe { gclk_p.as_ref().unwrap().genctrl.read().src() };

        // can't find println! for some reason
        arduino_nina::print::_print(format_args!("{}", "Running gclk_read"));
        arduino_nina::print::_print(format_args!("{}: {}\n", "GCLK_EN", gclock_en));
        arduino_nina::print::_print(format_args!("{}: {}\n", "source bits", gclock_source.bits()));
        arduino_nina::print::_print(format_args!("{}: {}\n", "is xosc", gclock_source.is_xosc()));
        arduino_nina::print::_print(format_args!(
            "{}: {}\n",
            "is gclkin",
            gclock_source.is_gclkin()
        ));
        arduino_nina::print::_print(format_args!(
            "{}: {}\n",
            "is gclkgen1",
            gclock_source.is_gclkgen1()
        ));
        arduino_nina::print::_print(format_args!(
            "{}: {}\n",
            "is gclkgen1",
            gclock_source.is_dfll48m()
        ));
        assert_eq!(1, 1, "one != two");
    }
}
