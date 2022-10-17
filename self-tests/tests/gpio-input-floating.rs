
// #![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate arduino_nina;
use arduino_nina::blink_led;

extern crate defmt_rtt;
use defmt_rtt as _;

use arduino_nina::*;

use arduino_nina::console::interface::Write;

include!{"panic_handler.rs"}

struct State {
}


#[defmt_test::tests]
mod tests {
    use super::State;
    
    // commented out since test filtering doesn't work? 
    // #[test]
    // fn ground_is_low() {
    //     arduino_nina::print::_print(format_args!("{}", "Running ground is low"));
    //     assert_eq!(1,1, "one != two");          
    // }
}
