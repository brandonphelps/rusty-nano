#![allow(non_camel_case_types)]


pub type R = crate::R<u8, super::CTRL>;

pub type W = crate::W<u8, super::CTRL>;

impl crate::ResetValue for super::CTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}

pub type SWRST_R = crate::R<bool, bool>;
pub struct SWRST_W<'a> {
    w: &'a mut W,
}

impl <'a> SWRST_W<'a> {
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }

    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }

    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01)
            | ((value as u8) & 0x01);
        self.w
    }
}


impl R {
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
}

impl W {
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
}
