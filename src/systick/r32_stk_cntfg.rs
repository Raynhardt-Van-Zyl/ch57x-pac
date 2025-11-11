#[doc = "Register `R32_STK_CNTFG` reader"]
pub type R = crate::R<R32StkCntfgSpec>;
#[doc = "Register `R32_STK_CNTFG` writer"]
pub type W = crate::W<R32StkCntfgSpec>;
#[doc = "Field `SWIE` reader - RW0,System soft interrupt enable"]
pub type SwieR = crate::BitReader;
#[doc = "Field `SWIE` writer - RW0,System soft interrupt enable"]
pub type SwieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTIF` reader - RW,Systick counter clear zero flag"]
pub type CntifR = crate::BitReader;
#[doc = "Field `CNTIF` writer - RW,Systick counter clear zero flag"]
pub type CntifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RW0,System soft interrupt enable"]
    #[inline(always)]
    pub fn swie(&self) -> SwieR {
        SwieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW,Systick counter clear zero flag"]
    #[inline(always)]
    pub fn cntif(&self) -> CntifR {
        CntifR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW0,System soft interrupt enable"]
    #[inline(always)]
    pub fn swie(&mut self) -> SwieW<'_, R32StkCntfgSpec> {
        SwieW::new(self, 0)
    }
    #[doc = "Bit 1 - RW,Systick counter clear zero flag"]
    #[inline(always)]
    pub fn cntif(&mut self) -> CntifW<'_, R32StkCntfgSpec> {
        CntifW::new(self, 1)
    }
}
#[doc = "Systick counter flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_stk_cntfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_stk_cntfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32StkCntfgSpec;
impl crate::RegisterSpec for R32StkCntfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_stk_cntfg::R`](R) reader structure"]
impl crate::Readable for R32StkCntfgSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_stk_cntfg::W`](W) writer structure"]
impl crate::Writable for R32StkCntfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_STK_CNTFG to value 0"]
impl crate::Resettable for R32StkCntfgSpec {}
