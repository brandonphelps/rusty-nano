#![allow(non_camel_case_types)]

pub type R = crate::R<u32, super::GENCTRL>;

pub type W = crate::W<u32, super::GENCTRL>;


#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum SRC_A {
    XOSC = 0,
    GCLKIN = 1,
    GCLKGEN1 = 2,
    OSCULP32K = 3,
    OSC32K = 4,
    XOSC32K = 5,
    OSC8M = 6,
    DFLL48M = 7,
    DPLL96M = 8,
}

impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}

pub type SRC_R = crate::R<u8, SRC_A>;

impl SRC_R {
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_A::XOSC),
            1 => Val(SRC_A::GCLKIN),
            2 => Val(SRC_A::GCLKGEN1),
            3 => Val(SRC_A::OSCULP32K),
            4 => Val(SRC_A::OSC32K),
            5 => Val(SRC_A::XOSC32K),
            6 => Val(SRC_A::OSC8M),
            7 => Val(SRC_A::DFLL48M),
            8 => Val(SRC_A::DPLL96M),
            i => Res(i),
        }
    }

    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SRC_A::XOSC
    }

    #[inline(always)]
    pub fn is_gclkin(&self) -> bool {
        *self == SRC_A::GCLKIN
    }

    #[inline(always)]
    pub fn is_gclkgen1(&self) -> bool {
        *self == SRC_A::GCLKGEN1
    }

    #[inline(always)]
    pub fn is_dfll48m(&self) -> bool {
        *self == SRC_A::DFLL48M
    }
}



pub type GENEN_R = crate::R<bool, bool>;



impl R {

    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }

    pub fn genen(&self) -> GENEN_R {
        GENEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}



