#[doc = "Register `R8_TKEY_COUNT` reader"]
pub type R = crate::R<R8TkeyCountSpec>;
#[doc = "Register `R8_TKEY_COUNT` writer"]
pub type W = crate::W<R8TkeyCountSpec>;
#[doc = "Field `RB_TKEY_CHARG_CNT` reader - RW, Touchkey charge count"]
pub type RbTkeyChargCntR = crate::FieldReader;
#[doc = "Field `RB_TKEY_CHARG_CNT` writer - RW, Touchkey charge count"]
pub type RbTkeyChargCntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RB_TKEY_DISCH_CNT` reader - RW, Touchkey discharge count"]
pub type RbTkeyDischCntR = crate::FieldReader;
#[doc = "Field `RB_TKEY_DISCH_CNT` writer - RW, Touchkey discharge count"]
pub type RbTkeyDischCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - RW, Touchkey charge count"]
    #[inline(always)]
    pub fn rb_tkey_charg_cnt(&self) -> RbTkeyChargCntR {
        RbTkeyChargCntR::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - RW, Touchkey discharge count"]
    #[inline(always)]
    pub fn rb_tkey_disch_cnt(&self) -> RbTkeyDischCntR {
        RbTkeyDischCntR::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:4 - RW, Touchkey charge count"]
    #[inline(always)]
    pub fn rb_tkey_charg_cnt(&mut self) -> RbTkeyChargCntW<'_, R8TkeyCountSpec> {
        RbTkeyChargCntW::new(self, 0)
    }
    #[doc = "Bits 5:7 - RW, Touchkey discharge count"]
    #[inline(always)]
    pub fn rb_tkey_disch_cnt(&mut self) -> RbTkeyDischCntW<'_, R8TkeyCountSpec> {
        RbTkeyDischCntW::new(self, 5)
    }
}
#[doc = "RW, Touchkey charge and discharge count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tkey_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tkey_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8TkeyCountSpec;
impl crate::RegisterSpec for R8TkeyCountSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_tkey_count::R`](R) reader structure"]
impl crate::Readable for R8TkeyCountSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_tkey_count::W`](W) writer structure"]
impl crate::Writable for R8TkeyCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_TKEY_COUNT to value 0"]
impl crate::Resettable for R8TkeyCountSpec {}
