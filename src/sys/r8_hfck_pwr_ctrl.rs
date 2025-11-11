#[doc = "Register `R8_HFCK_PWR_CTRL` reader"]
pub type R = crate::R<R8HfckPwrCtrlSpec>;
#[doc = "Register `R8_HFCK_PWR_CTRL` writer"]
pub type W = crate::W<R8HfckPwrCtrlSpec>;
#[doc = "Field `RB_CLK_XT32M_PON` reader - RWA, external 32MHz oscillator power control: 0=power down, 1-power on"]
pub type RbClkXt32mPonR = crate::BitReader;
#[doc = "Field `RB_CLK_XT32M_PON` writer - RWA, external 32MHz oscillator power control: 0=power down, 1-power on"]
pub type RbClkXt32mPonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_CLK_PLL_PON` reader - RWA, PLL power control: 0=power down, 1-power on"]
pub type RbClkPllPonR = crate::BitReader;
#[doc = "Field `RB_CLK_PLL_PON` writer - RWA, PLL power control: 0=power down, 1-power on"]
pub type RbClkPllPonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - RWA, external 32MHz oscillator power control: 0=power down, 1-power on"]
    #[inline(always)]
    pub fn rb_clk_xt32m_pon(&self) -> RbClkXt32mPonR {
        RbClkXt32mPonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RWA, PLL power control: 0=power down, 1-power on"]
    #[inline(always)]
    pub fn rb_clk_pll_pon(&self) -> RbClkPllPonR {
        RbClkPllPonR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RWA, external 32MHz oscillator power control: 0=power down, 1-power on"]
    #[inline(always)]
    pub fn rb_clk_xt32m_pon(&mut self) -> RbClkXt32mPonW<'_, R8HfckPwrCtrlSpec> {
        RbClkXt32mPonW::new(self, 2)
    }
    #[doc = "Bit 4 - RWA, PLL power control: 0=power down, 1-power on"]
    #[inline(always)]
    pub fn rb_clk_pll_pon(&mut self) -> RbClkPllPonW<'_, R8HfckPwrCtrlSpec> {
        RbClkPllPonW::new(self, 4)
    }
}
#[doc = "RWA, high frequency clock module power control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hfck_pwr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hfck_pwr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8HfckPwrCtrlSpec;
impl crate::RegisterSpec for R8HfckPwrCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_hfck_pwr_ctrl::R`](R) reader structure"]
impl crate::Readable for R8HfckPwrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_hfck_pwr_ctrl::W`](W) writer structure"]
impl crate::Writable for R8HfckPwrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_HFCK_PWR_CTRL to value 0x04"]
impl crate::Resettable for R8HfckPwrCtrlSpec {
    const RESET_VALUE: u8 = 0x04;
}
