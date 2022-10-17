#![allow(non_camel_case_types)]

pub type R = crate::R<u16, super::CLKCTRL>;

pub type W = crate::W<u16, super::CLKCTRL>;

impl crate::ResetValue for super::CLKCTRL {
    type Type = u16;

    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}


#[repr(C)]
pub enum ID_A {
    DFLL48 = 0,
    SERCOM0_CORE = 0x14,
    SERCOM1_CORE = 0x15,
    SERCOM2_CORE = 0x16,
    SERCOM3_CORE = 0x17,
    SERCOM4_CORE = 0x18,
}

impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}



pub type ID_R = crate::R<u8, ID_A>;
impl ID_R {
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ID_A> {
        use crate::Variant::*;

        match self.bits {
            0 => Val(ID_A::DFLL48),
            _ => { todo!() }
        }
    }
}

pub struct ID_W<'a> {
    w: &'a mut W,
}

impl <'a> ID_W<'a> {
    pub fn variant(self, variant: ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }

    pub fn dfll48(self) -> &'a mut W {
        self.variant(ID_A::DFLL48)
    }

    
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}



#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GEN_A {
    #[doc = "0: Generic clock generator 0"]
    GCLK0 = 0,
    GCLK1 = 1,
    GCLK2 = 2,
    GCLK3 = 3,
    GCLK4 = 4,
    GCLK5 = 5,
    GCLK6 = 6,
    GCLK7 = 7,
    GCLK8 = 8,
}

impl From<GEN_A> for u8 {
    #[inline(always)]
    fn from(variant: GEN_A) -> Self {
        variant as _
    }
}

pub type GEN_R = crate::R<u8, GEN_A>;

impl GEN_R {

    pub fn variant(&self) -> crate::Variant<u8, GEN_A> {
        use crate::Variant::*;

        match self.bits {
            0 => Val(GEN_A::GCLK0),
            1 => Val(GEN_A::GCLK1),
            2 => Val(GEN_A::GCLK2),
            3 => Val(GEN_A::GCLK3),
            4 => Val(GEN_A::GCLK4),
            5 => Val(GEN_A::GCLK5),
            6 => Val(GEN_A::GCLK6),
            7 => Val(GEN_A::GCLK7),
            8 => Val(GEN_A::GCLK8),
            i => Res(i),
        }
    }
}


pub struct GEN_W<'a> {
    w: &'a mut W,
}

impl<'a> GEN_W<'a> {

    #[inline(always)]
    pub fn variant(self, variant: GEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }

    #[inline(always)]
    pub fn gclk0(self) -> &'a mut W {
        self.variant(GEN_A::GCLK0)
    }
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut W {
        self.variant(GEN_A::GCLK1)
    }
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut W {
        self.variant(GEN_A::GCLK2)
    }
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut W {
        self.variant(GEN_A::GCLK3)
    }
    
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut W {
        self.variant(GEN_A::GCLK4)
    }

    #[inline(always)]
    pub fn gclk5(self) -> &'a mut W {
        self.variant(GEN_A::GCLK5)
    }
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut W {
        self.variant(GEN_A::GCLK6)
    }
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut W {
        self.variant(GEN_A::GCLK7)
    }
    #[inline(always)]
    pub fn gclk8(self) -> &'a mut W {
        self.variant(GEN_A::GCLK8)
    }

    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}


pub type CLKEN_R = crate::R<bool, bool>;

pub struct CLKEN_W<'a> {
    w: &'a mut W
}

impl<'a> CLKEN_W<'a> {
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }

    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }

    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x1) << 14);
        self.w
    }
}

impl R {
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x3f)  as u8)
    }

    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }

    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 14) & 0x1) != 0)
    }
}

impl W {
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }

    #[inline(always)]
    pub fn gen(&mut self) -> GEN_W {
        GEN_W { w: self }
    }

    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self } 
    }
}
