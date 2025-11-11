#[doc = "Register `R8_PLL_CONFIG` reader"]
pub type R = crate::R<R8PllConfigSpec>;
#[doc = "Register `R8_PLL_CONFIG` writer"]
pub type W = crate::W<R8PllConfigSpec>;
#[doc = "Field `RB_PLL_CFG_DAT` reader - RWA, PLL configure data"]
pub type RbPllCfgDatR = crate::FieldReader;
#[doc = "Field `RB_PLL_CFG_DAT` writer - RWA, PLL configure data"]
pub type RbPllCfgDatW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - RWA, PLL configure data"]
    #[inline(always)]
    pub fn rb_pll_cfg_dat(&self) -> RbPllCfgDatR {
        RbPllCfgDatR::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - RWA, PLL configure data"]
    #[inline(always)]
    pub fn rb_pll_cfg_dat(&mut self) -> RbPllCfgDatW<'_, R8PllConfigSpec> {
        RbPllCfgDatW::new(self, 0)
    }
}
#[doc = "RWA, PLL configuration control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pll_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pll_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8PllConfigSpec;
impl crate::RegisterSpec for R8PllConfigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pll_config::R`](R) reader structure"]
impl crate::Readable for R8PllConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pll_config::W`](W) writer structure"]
impl crate::Writable for R8PllConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PLL_CONFIG to value 0x4a"]
impl crate::Resettable for R8PllConfigSpec {
    const RESET_VALUE: u8 = 0x4a;
}
