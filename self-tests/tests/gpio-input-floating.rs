
#![deny(warnings)]
#![no_std]
#![no_main]


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[defmt_test::tests]
mod tests {

    #[test]
    fn ground_is_low() {
        assert!(false);          
    }
}
