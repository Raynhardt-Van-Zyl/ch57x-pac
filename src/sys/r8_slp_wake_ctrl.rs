#[doc = "Register `R8_SLP_WAKE_CTRL` reader"]
pub type R = crate::R<R8SlpWakeCtrlSpec>;
#[doc = "Register `R8_SLP_WAKE_CTRL` writer"]
pub type W = crate::W<R8SlpWakeCtrlSpec>;
#[doc = "Field `RB_SLP_USB_WAKE` reader - RWA, enable USB waking"]
pub type RbSlpUsbWakeR = crate::BitReader;
#[doc = "Field `RB_SLP_USB_WAKE` writer - RWA, enable USB waking"]
pub type RbSlpUsbWakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_RTC_WAKE` reader - RWA, enable RTC waking"]
pub type RbSlpRtcWakeR = crate::BitReader;
#[doc = "Field `RB_SLP_RTC_WAKE` writer - RWA, enable RTC waking"]
pub type RbSlpRtcWakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_GPIO_WAKE` reader - RWA, enable GPIO waking"]
pub type RbSlpGpioWakeR = crate::BitReader;
#[doc = "Field `RB_SLP_GPIO_WAKE` writer - RWA, enable GPIO waking"]
pub type RbSlpGpioWakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_BAT_WAKE` reader - RWA, enable BAT waking"]
pub type RbSlpBatWakeR = crate::BitReader;
#[doc = "Field `RB_SLP_BAT_WAKE` writer - RWA, enable BAT waking"]
pub type RbSlpBatWakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_WAKE_EV_MODE` reader - RWA, event wakeup mode"]
pub type RbWakeEvModeR = crate::BitReader;
#[doc = "Field `RB_WAKE_EV_MODE` writer - RWA, event wakeup mode"]
pub type RbWakeEvModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_WAKE_DELAY` reader - RWA, wakeup delay"]
pub type RbWakeDelayR = crate::BitReader;
#[doc = "Field `RB_WAKE_DELAY` writer - RWA, wakeup delay"]
pub type RbWakeDelayW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RWA, enable USB waking"]
    #[inline(always)]
    pub fn rb_slp_usb_wake(&self) -> RbSlpUsbWakeR {
        RbSlpUsbWakeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - RWA, enable RTC waking"]
    #[inline(always)]
    pub fn rb_slp_rtc_wake(&self) -> RbSlpRtcWakeR {
        RbSlpRtcWakeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RWA, enable GPIO waking"]
    #[inline(always)]
    pub fn rb_slp_gpio_wake(&self) -> RbSlpGpioWakeR {
        RbSlpGpioWakeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RWA, enable BAT waking"]
    #[inline(always)]
    pub fn rb_slp_bat_wake(&self) -> RbSlpBatWakeR {
        RbSlpBatWakeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RWA, event wakeup mode"]
    #[inline(always)]
    pub fn rb_wake_ev_mode(&self) -> RbWakeEvModeR {
        RbWakeEvModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RWA, wakeup delay"]
    #[inline(always)]
    pub fn rb_wake_delay(&self) -> RbWakeDelayR {
        RbWakeDelayR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RWA, enable USB waking"]
    #[inline(always)]
    pub fn rb_slp_usb_wake(&mut self) -> RbSlpUsbWakeW<'_, R8SlpWakeCtrlSpec> {
        RbSlpUsbWakeW::new(self, 0)
    }
    #[doc = "Bit 3 - RWA, enable RTC waking"]
    #[inline(always)]
    pub fn rb_slp_rtc_wake(&mut self) -> RbSlpRtcWakeW<'_, R8SlpWakeCtrlSpec> {
        RbSlpRtcWakeW::new(self, 3)
    }
    #[doc = "Bit 4 - RWA, enable GPIO waking"]
    #[inline(always)]
    pub fn rb_slp_gpio_wake(&mut self) -> RbSlpGpioWakeW<'_, R8SlpWakeCtrlSpec> {
        RbSlpGpioWakeW::new(self, 4)
    }
    #[doc = "Bit 5 - RWA, enable BAT waking"]
    #[inline(always)]
    pub fn rb_slp_bat_wake(&mut self) -> RbSlpBatWakeW<'_, R8SlpWakeCtrlSpec> {
        RbSlpBatWakeW::new(self, 5)
    }
    #[doc = "Bit 6 - RWA, event wakeup mode"]
    #[inline(always)]
    pub fn rb_wake_ev_mode(&mut self) -> RbWakeEvModeW<'_, R8SlpWakeCtrlSpec> {
        RbWakeEvModeW::new(self, 6)
    }
    #[doc = "Bit 7 - RWA, wakeup delay"]
    #[inline(always)]
    pub fn rb_wake_delay(&mut self) -> RbWakeDelayW<'_, R8SlpWakeCtrlSpec> {
        RbWakeDelayW::new(self, 7)
    }
}
#[doc = "RWA, wake control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_wake_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_wake_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8SlpWakeCtrlSpec;
impl crate::RegisterSpec for R8SlpWakeCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_slp_wake_ctrl::R`](R) reader structure"]
impl crate::Readable for R8SlpWakeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_slp_wake_ctrl::W`](W) writer structure"]
impl crate::Writable for R8SlpWakeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SLP_WAKE_CTRL to value 0x20"]
impl crate::Resettable for R8SlpWakeCtrlSpec {
    const RESET_VALUE: u8 = 0x20;
}
