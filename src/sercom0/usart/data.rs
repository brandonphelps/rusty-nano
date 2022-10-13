pub type R = crate::R<u16, super::DATA>;
pub type W = crate::W<u16, super::DATA>;

impl crate::ResetValue for super::DATA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}

pub type DATA_R = crate::R<u16, u16>;

pub struct DATA_W<'a> {
    w: &'a mut W,
}

impl<'a> DATA_W<'a> {
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01FF) | ((value as u16) & 0x01FF);
        self.w
    }
}

impl R {
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x01FF) as u16)
    }
}

impl W {
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
