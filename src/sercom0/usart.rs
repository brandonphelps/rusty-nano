use core::marker;

use crate::_print;

trait Readable {}
trait Writeable {} 

pub struct Reg<U, REG> {
    register: vcell::VolatileCell<U>,
    _marker: marker::PhantomData<REG>,
}

impl<U, REG> Reg<U, REG>
where
    // Self: Readable,
    U: Copy,
{
    pub fn read(&self) -> crate::R<U, Self> {
        crate::println!("D: {:?}", core::ptr::addr_of!(self) as u32);
        crate::R {
            bits: self.register.get(),
            _reg: marker::PhantomData,
        }
    }
}


pub type CTRLA = crate::Reg<u32, _CTRLA>;
pub struct _CTRLA;

impl Readable for CTRLA {}

impl Writeable for CTRLA {} 

pub type CTRLB = crate::Reg<u32, _CTRLB>;
pub struct _CTRLB;
impl Readable for CTRLB {}
impl Writeable for CTRLB {}


pub type BAUD = crate::Reg<u16, _BAUD>;
pub struct _BAUD;
impl Readable for BAUD {}
impl Writeable for BAUD {}

pub type BAUD_FRAC_MODE = crate::Reg<u16, _BAUD_FRAC_MODE>;
pub struct _BAUD_FRAC_MODE;
impl Readable for BAUD_FRAC_MODE {}
impl Writeable for BAUD_FRAC_MODE {}

pub type RXPL = crate::Reg<u8, _RXPL>;
pub struct _RXPL;
impl Readable for RXPL {}
impl Writeable for RXPL {}

pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
pub struct _INTENCLR;
impl Readable for INTENCLR {}
impl Writeable for INTENCLR {}

pub type INTENSET = crate::Reg<u8, _INTENSET>;
pub struct _INTENSET;
impl Readable for INTENSET {}
impl Writeable for INTENSET {}

pub type INTFLAG = crate::Reg<u8, _INTFLAG>;
pub struct _INTFLAG;
impl Readable for INTFLAG {}
impl Writeable for INTFLAG {}

pub type STATUS = crate::Reg<u16, _STATUS>;
pub struct _STATUS;
impl Readable for STATUS {}
impl Writeable for STATUS {}

pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
pub struct _SYNCBUSY;
impl Readable for SYNCBUSY {}
impl Writeable for SYNCBUSY {}

pub type DATA = crate::Reg<u16, _DATA>;
pub struct _DATA;
impl Readable for DATA {}
impl Writeable for DATA {}

pub type DBGCTRL = crate::Reg<u8, _DBGCTRL>;
pub struct _DBGCTRL;
impl Readable for DBGCTRL {}
impl Writeable for DBGCTRL {}

pub mod ctrla;
