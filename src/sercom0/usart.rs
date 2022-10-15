#![allow(non_camel_case_types)]

pub type CTRLA = crate::Reg<u32, _CTRLA>;
pub struct _CTRLA;

impl crate::Readable for CTRLA {}

impl crate::Writeable for CTRLA {}

pub type CTRLB = crate::Reg<u32, _CTRLB>;
pub struct _CTRLB;
impl crate::Readable for CTRLB {}
impl crate::Writeable for CTRLB {}

pub type BAUD = crate::Reg<u16, _BAUD>;
pub struct _BAUD;
impl crate::Readable for BAUD {}
impl crate::Writeable for BAUD {}

pub type BAUD_FRAC_MODE = crate::Reg<u16, _BAUD_FRAC_MODE>;
pub struct _BAUD_FRAC_MODE;
impl crate::Readable for BAUD_FRAC_MODE {}
impl crate::Writeable for BAUD_FRAC_MODE {}

pub type RXPL = crate::Reg<u8, _RXPL>;
pub struct _RXPL;
impl crate::Readable for RXPL {}
impl crate::Writeable for RXPL {}

pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
pub struct _INTENCLR;
impl crate::Readable for INTENCLR {}
impl crate::Writeable for INTENCLR {}

pub type INTENSET = crate::Reg<u8, _INTENSET>;
pub struct _INTENSET;
impl crate::Readable for INTENSET {}
impl crate::Writeable for INTENSET {}

pub type INTFLAG = crate::Reg<u8, _INTFLAG>;
pub struct _INTFLAG;
impl crate::Readable for INTFLAG {}
impl crate::Writeable for INTFLAG {}

pub mod intflag;

pub type STATUS = crate::Reg<u16, _STATUS>;
pub struct _STATUS;
impl crate::Readable for STATUS {}
impl crate::Writeable for STATUS {}

pub mod status;

pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
pub struct _SYNCBUSY;
impl crate::Readable for SYNCBUSY {}
impl crate::Writeable for SYNCBUSY {}

pub type DATA = crate::Reg<u16, _DATA>;
pub struct _DATA;
impl crate::Readable for DATA {}
impl crate::Writeable for DATA {}

pub mod data;

pub type DBGCTRL = crate::Reg<u8, _DBGCTRL>;
pub struct _DBGCTRL;
impl crate::Readable for DBGCTRL {}
impl crate::Writeable for DBGCTRL {}

pub mod ctrla;
