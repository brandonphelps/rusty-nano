

use core::panic::PanicInfo;


#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
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
