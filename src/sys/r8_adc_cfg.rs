#[doc = "Register `R8_ADC_CFG` reader"]
pub type R = crate::R<R8AdcCfgSpec>;
#[doc = "Register `R8_ADC_CFG` writer"]
pub type W = crate::W<R8AdcCfgSpec>;
#[doc = "Field `RB_ADC_POWER_ON` reader - RW, ADC power control: 0=power down, 1=power on"]
pub type RbAdcPowerOnR = crate::BitReader;
#[doc = "Field `RB_ADC_POWER_ON` writer - RW, ADC power control: 0=power down, 1=power on"]
pub type RbAdcPowerOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ADC_BUF_EN` reader - RW, ADC input buffer enable"]
pub type RbAdcBufEnR = crate::BitReader;
#[doc = "Field `RB_ADC_BUF_EN` writer - RW, ADC input buffer enable"]
pub type RbAdcBufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ADC_DIFF_EN` reader - RW, ADC input channel mode: 0=single-end, 1=differnetial"]
pub type RbAdcDiffEnR = crate::BitReader;
#[doc = "Field `RB_ADC_DIFF_EN` writer - RW, ADC input channel mode: 0=single-end, 1=differnetial"]
pub type RbAdcDiffEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ADC_OFS_TEST` reader - RW, enable ADC offset test mode: 0=normal mode, 1=short port4 to test offset"]
pub type RbAdcOfsTestR = crate::BitReader;
#[doc = "Field `RB_ADC_OFS_TEST` writer - RW, enable ADC offset test mode: 0=normal mode, 1=short port4 to test offset"]
pub type RbAdcOfsTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ADC_PGA_GAIN` reader - RW, set ADC input PGA gain: 00=-12dB, 01=-6dB, 10=0dB, 11=6dB"]
pub type RbAdcPgaGainR = crate::FieldReader;
#[doc = "Field `RB_ADC_PGA_GAIN` writer - RW, set ADC input PGA gain: 00=-12dB, 01=-6dB, 10=0dB, 11=6dB"]
pub type RbAdcPgaGainW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_ADC_CLK_DIV` reader - RW, select ADC clock frequency: 00=3.2MHz, 01=2.67MHz, 10=5.33MHz, 11=4MHz"]
pub type RbAdcClkDivR = crate::FieldReader;
#[doc = "Field `RB_ADC_CLK_DIV` writer - RW, select ADC clock frequency: 00=3.2MHz, 01=2.67MHz, 10=5.33MHz, 11=4MHz"]
pub type RbAdcClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - RW, ADC power control: 0=power down, 1=power on"]
    #[inline(always)]
    pub fn rb_adc_power_on(&self) -> RbAdcPowerOnR {
        RbAdcPowerOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW, ADC input buffer enable"]
    #[inline(always)]
    pub fn rb_adc_buf_en(&self) -> RbAdcBufEnR {
        RbAdcBufEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RW, ADC input channel mode: 0=single-end, 1=differnetial"]
    #[inline(always)]
    pub fn rb_adc_diff_en(&self) -> RbAdcDiffEnR {
        RbAdcDiffEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RW, enable ADC offset test mode: 0=normal mode, 1=short port4 to test offset"]
    #[inline(always)]
    pub fn rb_adc_ofs_test(&self) -> RbAdcOfsTestR {
        RbAdcOfsTestR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - RW, set ADC input PGA gain: 00=-12dB, 01=-6dB, 10=0dB, 11=6dB"]
    #[inline(always)]
    pub fn rb_adc_pga_gain(&self) -> RbAdcPgaGainR {
        RbAdcPgaGainR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - RW, select ADC clock frequency: 00=3.2MHz, 01=2.67MHz, 10=5.33MHz, 11=4MHz"]
    #[inline(always)]
    pub fn rb_adc_clk_div(&self) -> RbAdcClkDivR {
        RbAdcClkDivR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - RW, ADC power control: 0=power down, 1=power on"]
    #[inline(always)]
    pub fn rb_adc_power_on(&mut self) -> RbAdcPowerOnW<'_, R8AdcCfgSpec> {
        RbAdcPowerOnW::new(self, 0)
    }
    #[doc = "Bit 1 - RW, ADC input buffer enable"]
    #[inline(always)]
    pub fn rb_adc_buf_en(&mut self) -> RbAdcBufEnW<'_, R8AdcCfgSpec> {
        RbAdcBufEnW::new(self, 1)
    }
    #[doc = "Bit 2 - RW, ADC input channel mode: 0=single-end, 1=differnetial"]
    #[inline(always)]
    pub fn rb_adc_diff_en(&mut self) -> RbAdcDiffEnW<'_, R8AdcCfgSpec> {
        RbAdcDiffEnW::new(self, 2)
    }
    #[doc = "Bit 3 - RW, enable ADC offset test mode: 0=normal mode, 1=short port4 to test offset"]
    #[inline(always)]
    pub fn rb_adc_ofs_test(&mut self) -> RbAdcOfsTestW<'_, R8AdcCfgSpec> {
        RbAdcOfsTestW::new(self, 3)
    }
    #[doc = "Bits 4:5 - RW, set ADC input PGA gain: 00=-12dB, 01=-6dB, 10=0dB, 11=6dB"]
    #[inline(always)]
    pub fn rb_adc_pga_gain(&mut self) -> RbAdcPgaGainW<'_, R8AdcCfgSpec> {
        RbAdcPgaGainW::new(self, 4)
    }
    #[doc = "Bits 6:7 - RW, select ADC clock frequency: 00=3.2MHz, 01=2.67MHz, 10=5.33MHz, 11=4MHz"]
    #[inline(always)]
    pub fn rb_adc_clk_div(&mut self) -> RbAdcClkDivW<'_, R8AdcCfgSpec> {
        RbAdcClkDivW::new(self, 6)
    }
}
#[doc = "RW, ADC configure\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8AdcCfgSpec;
impl crate::RegisterSpec for R8AdcCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_adc_cfg::R`](R) reader structure"]
impl crate::Readable for R8AdcCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_adc_cfg::W`](W) writer structure"]
impl crate::Writable for R8AdcCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_ADC_CFG to value 0xa0"]
impl crate::Resettable for R8AdcCfgSpec {
    const RESET_VALUE: u8 = 0xa0;
}
