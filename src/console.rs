use crate::{sercom0, syncronization::interface::Mutex};

use arduino_nano33iot as bsp;
use bsp::hal;
use hal::prelude::*;

use core::fmt;
pub mod interface {
    use core::fmt;

    pub trait Write {
        fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result;
        fn write_char(&mut self, c: char);
        fn flush(&self);
    }

    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }
    }

    pub trait All: Write + Statistics {}
}

use crate::syncronization::NullLock;

struct UartInner {
    value: NullLock<super::bsp::Uart>,
}

impl UartInner {
    fn write_char(&mut self, c: char) {
        // todo: handle write result.
        self.value.lock(|inner| inner.write(c as u8));
    }
}

impl fmt::Write for UartInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {

            self.write_char(c);
            
            let usart = sercom0::Uart::new();            
            // wait till the dre bit is empty.
            while !usart.usart.intflag.read().txc().bit() {}
            // crate::timer::timer().delay(100);
        }
        Ok(())
    }
}

struct NinaUart {
    inner: Option<UartInner>,
}

impl NinaUart {
    const unsafe fn new() -> Self {
        Self { inner: None }
    }

    unsafe fn set_uart(&mut self, uart: super::bsp::Uart) {
        self.inner = Some(UartInner {
            value: NullLock::new(uart),
        });
    }
}

impl interface::Write for NinaUart {
    fn write_char(&mut self, c: char) {
        match self.inner {
            Some(ref mut v) => {
                v.write_char(c);
            }
            None => {}
        }
    }

    fn write_fmt(&mut self, args: core::fmt::Arguments) -> fmt::Result {
        match self.inner {
            Some(ref mut v) => fmt::Write::write_fmt(v, args),
            None => Ok(()),
        }
    }

    fn flush(&self) {
        todo!()
    }
}

impl interface::Statistics for NinaUart {}
impl interface::All for NinaUart {}

// todo: coudl this mut be removed?
static mut CONSOLE_UART: NinaUart = unsafe { NinaUart::new() };

pub fn console() -> &'static mut dyn interface::All {
    unsafe { &mut CONSOLE_UART }
}

pub fn console_init(uart: super::bsp::Uart) {
    unsafe {
        CONSOLE_UART.set_uart(uart);
    }
}
