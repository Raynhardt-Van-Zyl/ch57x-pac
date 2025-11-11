#[doc = "Register `R8_SLP_CLK_OFF1` reader"]
pub type R = crate::R<R8SlpClkOff1Spec>;
#[doc = "Register `R8_SLP_CLK_OFF1` writer"]
pub type W = crate::W<R8SlpClkOff1Spec>;
#[doc = "Field `RB_SLP_CLK_SPI0` reader - RWA, close SPI0 clock"]
pub type RbSlpClkSpi0R = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_SPI0` writer - RWA, close SPI0 clock"]
pub type RbSlpClkSpi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_PWMX` reader - RWA, close PWMx clock"]
pub type RbSlpClkPwmxR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_PWMX` writer - RWA, close PWMx clock"]
pub type RbSlpClkPwmxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_USB` reader - RWA, close USB clock"]
pub type RbSlpClkUsbR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_USB` writer - RWA, close USB clock"]
pub type RbSlpClkUsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_BLE` reader - RWA, close BLE clock"]
pub type RbSlpClkBleR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_BLE` writer - RWA, close BLE clock"]
pub type RbSlpClkBleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RWA, close SPI0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_spi0(&self) -> RbSlpClkSpi0R {
        RbSlpClkSpi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - RWA, close PWMx clock"]
    #[inline(always)]
    pub fn rb_slp_clk_pwmx(&self) -> RbSlpClkPwmxR {
        RbSlpClkPwmxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RWA, close USB clock"]
    #[inline(always)]
    pub fn rb_slp_clk_usb(&self) -> RbSlpClkUsbR {
        RbSlpClkUsbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - RWA, close BLE clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ble(&self) -> RbSlpClkBleR {
        RbSlpClkBleR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RWA, close SPI0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_spi0(&mut self) -> RbSlpClkSpi0W<'_, R8SlpClkOff1Spec> {
        RbSlpClkSpi0W::new(self, 0)
    }
    #[doc = "Bit 2 - RWA, close PWMx clock"]
    #[inline(always)]
    pub fn rb_slp_clk_pwmx(&mut self) -> RbSlpClkPwmxW<'_, R8SlpClkOff1Spec> {
        RbSlpClkPwmxW::new(self, 2)
    }
    #[doc = "Bit 4 - RWA, close USB clock"]
    #[inline(always)]
    pub fn rb_slp_clk_usb(&mut self) -> RbSlpClkUsbW<'_, R8SlpClkOff1Spec> {
        RbSlpClkUsbW::new(self, 4)
    }
    #[doc = "Bit 7 - RWA, close BLE clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ble(&mut self) -> RbSlpClkBleW<'_, R8SlpClkOff1Spec> {
        RbSlpClkBleW::new(self, 7)
    }
}
#[doc = "RWA, sleep clock off control byte 1, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_clk_off1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_clk_off1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8SlpClkOff1Spec;
impl crate::RegisterSpec for R8SlpClkOff1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_slp_clk_off1::R`](R) reader structure"]
impl crate::Readable for R8SlpClkOff1Spec {}
#[doc = "`write(|w| ..)` method takes [`r8_slp_clk_off1::W`](W) writer structure"]
impl crate::Writable for R8SlpClkOff1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SLP_CLK_OFF1 to value 0"]
impl crate::Resettable for R8SlpClkOff1Spec {}
