#[doc = "Register `R8_CK32K_CONFIG` reader"]
pub type R = crate::R<R8Ck32kConfigSpec>;
#[doc = "Register `R8_CK32K_CONFIG` writer"]
pub type W = crate::W<R8Ck32kConfigSpec>;
#[doc = "Field `RB_CLK_XT32K_PON` reader - RWA, external 32KHz oscillator power on"]
pub type RbClkXt32kPonR = crate::BitReader;
#[doc = "Field `RB_CLK_XT32K_PON` writer - RWA, external 32KHz oscillator power on"]
pub type RbClkXt32kPonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_CLK_INT32K_PON` reader - RWA, internal 32KHz oscillator power on"]
pub type RbClkInt32kPonR = crate::BitReader;
#[doc = "Field `RB_CLK_INT32K_PON` writer - RWA, internal 32KHz oscillator power on"]
pub type RbClkInt32kPonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_CLK_OSC32K_XT` reader - RWA, 32KHz oscillator source selection: 0=RC, 1=XT"]
pub type RbClkOsc32kXtR = crate::BitReader;
#[doc = "Field `RB_CLK_OSC32K_XT` writer - RWA, 32KHz oscillator source selection: 0=RC, 1=XT"]
pub type RbClkOsc32kXtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_32K_CLK_PIN` reader - RO, 32KHz oscillator clock pin status"]
pub type Rb32kClkPinR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RWA, external 32KHz oscillator power on"]
    #[inline(always)]
    pub fn rb_clk_xt32k_pon(&self) -> RbClkXt32kPonR {
        RbClkXt32kPonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RWA, internal 32KHz oscillator power on"]
    #[inline(always)]
    pub fn rb_clk_int32k_pon(&self) -> RbClkInt32kPonR {
        RbClkInt32kPonR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RWA, 32KHz oscillator source selection: 0=RC, 1=XT"]
    #[inline(always)]
    pub fn rb_clk_osc32k_xt(&self) -> RbClkOsc32kXtR {
        RbClkOsc32kXtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, 32KHz oscillator clock pin status"]
    #[inline(always)]
    pub fn rb_32k_clk_pin(&self) -> Rb32kClkPinR {
        Rb32kClkPinR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RWA, external 32KHz oscillator power on"]
    #[inline(always)]
    pub fn rb_clk_xt32k_pon(&mut self) -> RbClkXt32kPonW<'_, R8Ck32kConfigSpec> {
        RbClkXt32kPonW::new(self, 0)
    }
    #[doc = "Bit 1 - RWA, internal 32KHz oscillator power on"]
    #[inline(always)]
    pub fn rb_clk_int32k_pon(&mut self) -> RbClkInt32kPonW<'_, R8Ck32kConfigSpec> {
        RbClkInt32kPonW::new(self, 1)
    }
    #[doc = "Bit 2 - RWA, 32KHz oscillator source selection: 0=RC, 1=XT"]
    #[inline(always)]
    pub fn rb_clk_osc32k_xt(&mut self) -> RbClkOsc32kXtW<'_, R8Ck32kConfigSpec> {
        RbClkOsc32kXtW::new(self, 2)
    }
}
#[doc = "RWA, 32KHz oscillator configure\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_ck32k_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_ck32k_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Ck32kConfigSpec;
impl crate::RegisterSpec for R8Ck32kConfigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_ck32k_config::R`](R) reader structure"]
impl crate::Readable for R8Ck32kConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_ck32k_config::W`](W) writer structure"]
impl crate::Writable for R8Ck32kConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_CK32K_CONFIG to value 0x02"]
impl crate::Resettable for R8Ck32kConfigSpec {
    const RESET_VALUE: u8 = 0x02;
}
