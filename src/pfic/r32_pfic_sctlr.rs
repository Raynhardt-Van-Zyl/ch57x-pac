#[doc = "Register `R32_PFIC_SCTLR` reader"]
pub type R = crate::R<R32PficSctlrSpec>;
#[doc = "Register `R32_PFIC_SCTLR` writer"]
pub type W = crate::W<R32PficSctlrSpec>;
#[doc = "Field `SLEEPONEXIT` reader - RW,SLEEPONEXIT"]
pub type SleeponexitR = crate::BitReader;
#[doc = "Field `SLEEPONEXIT` writer - RW,SLEEPONEXIT"]
pub type SleeponexitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEPDEEP` reader - RW,SLEEPDEEP"]
pub type SleepdeepR = crate::BitReader;
#[doc = "Field `SLEEPDEEP` writer - RW,SLEEPDEEP"]
pub type SleepdeepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFITOWFE` reader - RW,WFITOWFE"]
pub type WfitowfeR = crate::BitReader;
#[doc = "Field `WFITOWFE` writer - RW,WFITOWFE"]
pub type WfitowfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEVONPEND` reader - RW,SEVONPEND"]
pub type SevonpendR = crate::BitReader;
#[doc = "Field `SEVONPEND` writer - RW,SEVONPEND"]
pub type SevonpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETEVENT` reader - WO,SETEVENT"]
pub type SeteventR = crate::BitReader;
#[doc = "Field `SETEVENT` writer - WO,SETEVENT"]
pub type SeteventW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - RW,SLEEPONEXIT"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SleeponexitR {
        SleeponexitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RW,SLEEPDEEP"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SleepdeepR {
        SleepdeepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RW,WFITOWFE"]
    #[inline(always)]
    pub fn wfitowfe(&self) -> WfitowfeR {
        WfitowfeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RW,SEVONPEND"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SevonpendR {
        SevonpendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WO,SETEVENT"]
    #[inline(always)]
    pub fn setevent(&self) -> SeteventR {
        SeteventR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RW,SLEEPONEXIT"]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> SleeponexitW<'_, R32PficSctlrSpec> {
        SleeponexitW::new(self, 1)
    }
    #[doc = "Bit 2 - RW,SLEEPDEEP"]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> SleepdeepW<'_, R32PficSctlrSpec> {
        SleepdeepW::new(self, 2)
    }
    #[doc = "Bit 3 - RW,WFITOWFE"]
    #[inline(always)]
    pub fn wfitowfe(&mut self) -> WfitowfeW<'_, R32PficSctlrSpec> {
        WfitowfeW::new(self, 3)
    }
    #[doc = "Bit 4 - RW,SEVONPEND"]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> SevonpendW<'_, R32PficSctlrSpec> {
        SevonpendW::new(self, 4)
    }
    #[doc = "Bit 5 - WO,SETEVENT"]
    #[inline(always)]
    pub fn setevent(&mut self) -> SeteventW<'_, R32PficSctlrSpec> {
        SeteventW::new(self, 5)
    }
}
#[doc = "System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_sctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_sctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficSctlrSpec;
impl crate::RegisterSpec for R32PficSctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_sctlr::R`](R) reader structure"]
impl crate::Readable for R32PficSctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_sctlr::W`](W) writer structure"]
impl crate::Writable for R32PficSctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_SCTLR to value 0"]
impl crate::Resettable for R32PficSctlrSpec {}
