pub type R = crate::R<u8, super::INTFLAG>;
pub type W = crate::W<u8, super::INTFLAG>;

pub type TXC_R = crate::R<bool, bool>;

pub type DRE_R = crate::R<bool, bool>;

impl R {
    #[inline(always)]
    pub fn dre(&self) -> DRE_R {
        DRE_R::new((self.bits & 0x01) != 0)
    }

    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 0x02) != 0)
    }
}
