#[doc = "Register `R16_CLK_SYS_CFG` reader"]
pub type R = crate::R<R16ClkSysCfgSpec>;
#[doc = "Register `R16_CLK_SYS_CFG` writer"]
pub type W = crate::W<R16ClkSysCfgSpec>;
#[doc = "Field `RB_CLK_PLL_DIV` reader - RWA, output clock divider from PLL or CK32M"]
pub type RbClkPllDivR = crate::FieldReader;
#[doc = "Field `RB_CLK_PLL_DIV` writer - RWA, output clock divider from PLL or CK32M"]
pub type RbClkPllDivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RB_CLK_SYS_MOD` reader - RWA, system clock source mode: 00=divided from 32MHz, 01=divided from PLL-480MHz, 10=directly from 32MHz, 11=directly from 32KHz"]
pub type RbClkSysModR = crate::FieldReader;
#[doc = "Field `RB_CLK_SYS_MOD` writer - RWA, system clock source mode: 00=divided from 32MHz, 01=divided from PLL-480MHz, 10=directly from 32MHz, 11=directly from 32KHz"]
pub type RbClkSysModW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - RWA, output clock divider from PLL or CK32M"]
    #[inline(always)]
    pub fn rb_clk_pll_div(&self) -> RbClkPllDivR {
        RbClkPllDivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - RWA, system clock source mode: 00=divided from 32MHz, 01=divided from PLL-480MHz, 10=directly from 32MHz, 11=directly from 32KHz"]
    #[inline(always)]
    pub fn rb_clk_sys_mod(&self) -> RbClkSysModR {
        RbClkSysModR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - RWA, output clock divider from PLL or CK32M"]
    #[inline(always)]
    pub fn rb_clk_pll_div(&mut self) -> RbClkPllDivW<'_, R16ClkSysCfgSpec> {
        RbClkPllDivW::new(self, 0)
    }
    #[doc = "Bits 6:7 - RWA, system clock source mode: 00=divided from 32MHz, 01=divided from PLL-480MHz, 10=directly from 32MHz, 11=directly from 32KHz"]
    #[inline(always)]
    pub fn rb_clk_sys_mod(&mut self) -> RbClkSysModW<'_, R16ClkSysCfgSpec> {
        RbClkSysModW::new(self, 6)
    }
}
#[doc = "RWA, system clock configuration, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_clk_sys_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_clk_sys_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16ClkSysCfgSpec;
impl crate::RegisterSpec for R16ClkSysCfgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_clk_sys_cfg::R`](R) reader structure"]
impl crate::Readable for R16ClkSysCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_clk_sys_cfg::W`](W) writer structure"]
impl crate::Writable for R16ClkSysCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_CLK_SYS_CFG to value 0x05"]
impl crate::Resettable for R16ClkSysCfgSpec {
    const RESET_VALUE: u16 = 0x05;
}
