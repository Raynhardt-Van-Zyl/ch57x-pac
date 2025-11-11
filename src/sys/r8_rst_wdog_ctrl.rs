#[doc = "Register `R8_RST_WDOG_CTRL` reader"]
pub type R = crate::R<R8RstWdogCtrlSpec>;
#[doc = "Register `R8_RST_WDOG_CTRL` writer"]
pub type W = crate::W<R8RstWdogCtrlSpec>;
#[doc = "Field `RB_SOFTWARE_RESET` reader - WA or WZ, global software reset, high action, auto clear"]
pub type RbSoftwareResetR = crate::BitReader;
#[doc = "Field `RB_SOFTWARE_RESET` writer - WA or WZ, global software reset, high action, auto clear"]
pub type RbSoftwareResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_WDOG_RST_EN` reader - RWA, enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow"]
pub type RbWdogRstEnR = crate::BitReader;
#[doc = "Field `RB_WDOG_RST_EN` writer - RWA, enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow"]
pub type RbWdogRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_WDOG_INT_EN` reader - RWA, watch-dog timer overflow interrupt enable: 0=disable, 1=enable"]
pub type RbWdogIntEnR = crate::BitReader;
#[doc = "Field `RB_WDOG_INT_EN` writer - RWA, watch-dog timer overflow interrupt enable: 0=disable, 1=enable"]
pub type RbWdogIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_WDOG_INT_FLAG` reader - RW1, watch-dog timer overflow interrupt flag, cleared by RW1 or reload watch-dog count or __SEV(Send-Event)"]
pub type RbWdogIntFlagR = crate::BitReader;
#[doc = "Field `RB_WDOG_INT_FLAG` writer - RW1, watch-dog timer overflow interrupt flag, cleared by RW1 or reload watch-dog count or __SEV(Send-Event)"]
pub type RbWdogIntFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WA or WZ, global software reset, high action, auto clear"]
    #[inline(always)]
    pub fn rb_software_reset(&self) -> RbSoftwareResetR {
        RbSoftwareResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RWA, enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow"]
    #[inline(always)]
    pub fn rb_wdog_rst_en(&self) -> RbWdogRstEnR {
        RbWdogRstEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RWA, watch-dog timer overflow interrupt enable: 0=disable, 1=enable"]
    #[inline(always)]
    pub fn rb_wdog_int_en(&self) -> RbWdogIntEnR {
        RbWdogIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RW1, watch-dog timer overflow interrupt flag, cleared by RW1 or reload watch-dog count or __SEV(Send-Event)"]
    #[inline(always)]
    pub fn rb_wdog_int_flag(&self) -> RbWdogIntFlagR {
        RbWdogIntFlagR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WA or WZ, global software reset, high action, auto clear"]
    #[inline(always)]
    pub fn rb_software_reset(&mut self) -> RbSoftwareResetW<'_, R8RstWdogCtrlSpec> {
        RbSoftwareResetW::new(self, 0)
    }
    #[doc = "Bit 1 - RWA, enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow"]
    #[inline(always)]
    pub fn rb_wdog_rst_en(&mut self) -> RbWdogRstEnW<'_, R8RstWdogCtrlSpec> {
        RbWdogRstEnW::new(self, 1)
    }
    #[doc = "Bit 2 - RWA, watch-dog timer overflow interrupt enable: 0=disable, 1=enable"]
    #[inline(always)]
    pub fn rb_wdog_int_en(&mut self) -> RbWdogIntEnW<'_, R8RstWdogCtrlSpec> {
        RbWdogIntEnW::new(self, 2)
    }
    #[doc = "Bit 4 - RW1, watch-dog timer overflow interrupt flag, cleared by RW1 or reload watch-dog count or __SEV(Send-Event)"]
    #[inline(always)]
    pub fn rb_wdog_int_flag(&mut self) -> RbWdogIntFlagW<'_, R8RstWdogCtrlSpec> {
        RbWdogIntFlagW::new(self, 4)
    }
}
#[doc = "RWA, reset and watch-dog control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_rst_wdog_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_rst_wdog_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8RstWdogCtrlSpec;
impl crate::RegisterSpec for R8RstWdogCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_rst_wdog_ctrl::R`](R) reader structure"]
impl crate::Readable for R8RstWdogCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_rst_wdog_ctrl::W`](W) writer structure"]
impl crate::Writable for R8RstWdogCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_RST_WDOG_CTRL to value 0"]
impl crate::Resettable for R8RstWdogCtrlSpec {}
