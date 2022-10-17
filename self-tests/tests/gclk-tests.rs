// #![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate arduino_nina;
use arduino_nina::blink_led;

extern crate defmt_rtt;
#[allow(unused)]
use defmt_rtt as _;

use arduino_nina::*;
use arduino_nina::console::interface::Write;

include!{"panic_handler.rs"}

#[defmt_test::tests]
mod tests {

    use super::*;

    #[test]
    fn gclk_read() {
        
        // can't find println! for some reason
        arduino_nina::print::_print(format_args!("{}", "Running gclk_read"));
        assert_eq!(1,1, "one != two");          
    }
}
