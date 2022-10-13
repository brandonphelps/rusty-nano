pub type R = crate::R<u32, super::CTRLA>;
pub type W = crate::W<u32, super::CTRLA>;

pub struct ENABLE_W<'a> {
    w: &'a mut W,
}

impl<'a> ENABLE_W<'a> {
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}

impl W {
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
