
#[repr(C)]
pub struct RegisterBlock { 

    pub ctrl: CTRL,

    pub status: STATUS,

    pub clkctrl: CLKCTRL,
    
    pub genctrl: GENCTRL,
    
    pub gendiv: GENDIV,
}

pub type CTRL = crate::Reg<u8, _CTRL>;


pub struct _CTRL;
impl crate::Readable for CTRL {}
impl crate::Writeable for CTRL {}
pub mod ctrl;


pub type STATUS = crate::Reg<u8, _STATUS>;

pub struct _STATUS;

impl crate::Readable for STATUS {}

pub mod status;


pub type CLKCTRL = crate::Reg<u16, _CLKCTRL>;
pub struct _CLKCTRL;

impl crate::Readable for CLKCTRL {}
impl crate::Writeable for CLKCTRL {}

pub mod clkctrl;

pub type GENCTRL = crate::Reg<u32, _GENCTRL>;
pub struct _GENCTRL;

impl crate::Readable for GENCTRL {}
impl crate::Writeable for GENCTRL {}

pub mod genctrl;

pub type GENDIV = crate::Reg<u32, _GENDIV>;
pub struct _GENDIV;

impl crate::Readable for GENDIV {}
impl crate::Writeable for GENDIV {}

pub mod gendiv;


