
// #![deny(warnings)]
#![no_std]
#![no_main]


use core::panic::PanicInfo;

extern crate arduino_nina;
use arduino_nina::blink_led;
extern crate defmt_rtt;
use defmt_rtt as _;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    blink_led()
}

struct State {
}


#[defmt_test::tests]
mod tests {
    use super::State;
    #[init]
    fn init() -> State {
        let p = arduino_nina::sercom0::Uart::new();

        State { }
    }

    #[test]
    fn ground_is_low() {
        assert!(true);          
    }
}
