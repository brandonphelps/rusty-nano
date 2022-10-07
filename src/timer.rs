use crate::syncronization::interface::Mutex;
use crate::syncronization::NullLock;

use arduino_nano33iot as bsp;
use bsp::hal;
use hal::prelude::*;

struct DelayTimerInner {
    value: NullLock<super::hal::delay::Delay>,
}

impl DelayTimerInner {
    fn delay(&self, v: u32) {
        self.value.lock(|inner| inner.delay_ms(v));
    }
}

struct DelayTimer {
    inner: Option<DelayTimerInner>,
}

impl DelayTimer {
    unsafe fn set_timer(&mut self, delay: super::Delay) {
        self.inner = Some(DelayTimerInner { value: NullLock::new(delay) });
    }
}

pub trait Timer {
    fn delay(&self, v: u32);
}

impl Timer for DelayTimer {
    fn delay(&self, time: u32) {
        match self.inner {
            Some(ref v) => {
                v.delay(time);
            },
            None => { }
        }
    }
}

static mut GLOBAL_TIMER: DelayTimer =  DelayTimer { inner: None };

pub fn timer() -> &'static mut dyn Timer {
    unsafe { &mut GLOBAL_TIMER }
}

pub fn set_timer(delay: super::Delay) {
    unsafe { GLOBAL_TIMER.set_timer(delay) }
}
