use core::marker::PhantomData;
use core::ops::Deref;


pub struct SERCOM0 {
    _marker: PhantomData<*const ()>,
}

impl SERCOM0 {
    pub const fn ptr() -> *const RegisterBlock {
        0x4200_0800 as *const _
    }
}

pub struct SERCOM5 {
    _marker: PhantomData<*const ()>,
}

impl SERCOM5 {
    pub const fn ptr() -> *const RegisterBlock {
        0x42001C00 as *const _
    }
}

impl Deref for SERCOM5 {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM5::ptr() }
    }
}

impl Deref for SERCOM0 {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM0::ptr() }
    }
}

pub struct RegisterBlock {
    _reserved_0_spi: [u8; 49usize],
}

impl RegisterBlock {
    pub fn usart(&self) -> &USART {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART) }
    }
}

#[repr(C)]
pub struct USART {                 
    pub ctrla: self::usart::CTRLA, // 0x0
    pub ctrlb: self::usart::CTRLB, // 0x4
    _reserved2: [u8; 4usize],      // 0x8
    _reserved_2_baud: [u8; 2usize], // 0xC
    pub rxpl: self::usart::RXPL,   // 0xE
    _reserved4: [u8; 5usize],      // 0xF
    pub intenclr: self::usart::INTENCLR, // 0x14
    _reserved5: [u8; 1usize],            // 0x15
    pub intenset: self::usart::INTENSET, // 0x16
    _reserved6: [u8; 1usize],            // 0x17
    pub intflag: self::usart::INTFLAG,   // 0x18
    _reserved7: [u8; 1usize],            // 0x19
    pub status: self::usart::STATUS,     // 0x1A
    pub syncbusy: self::usart::SYNCBUSY, // 0x1C
    _reserved9: [u8; 8usize],            // 0x20
    pub data: self::usart::DATA,         // 0x28
    _reserved10: [u8; 6usize],           // 0x2A
    pub dbgctrl: self::usart::DBGCTRL,   // 0x30
}

pub struct Uart {
    
}

impl Uart {
    pub fn new() -> Self {
        Self { } 
    }

    pub fn get_something(&self) -> u32 {
        todo!()
    }
}


pub mod usart;


#[cfg(test)]
mod tests {

    use std::mem::size_of;

    #[test]
    fn test_size() {
        
    }
}
