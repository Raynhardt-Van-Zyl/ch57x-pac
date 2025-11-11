#[doc = "Register `R8_BAT_DET_CTRL` reader"]
pub type R = crate::R<R8BatDetCtrlSpec>;
#[doc = "Register `R8_BAT_DET_CTRL` writer"]
pub type W = crate::W<R8BatDetCtrlSpec>;
#[doc = "Field `RB_BAT_DET_EN__RB_BAT_LOW_VTHX` reader - RWA, battery voltage detector enable/select monitor threshold voltage"]
pub type RbBatDetEn_RbBatLowVthxR = crate::BitReader;
#[doc = "Field `RB_BAT_DET_EN__RB_BAT_LOW_VTHX` writer - RWA, battery voltage detector enable/select monitor threshold voltage"]
pub type RbBatDetEn_RbBatLowVthxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_BAT_MON_EN` reader - RWA, battery voltage monitor enable under sleep mode"]
pub type RbBatMonEnR = crate::BitReader;
#[doc = "Field `RB_BAT_MON_EN` writer - RWA, battery voltage monitor enable under sleep mode"]
pub type RbBatMonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_BAT_LOWER_IE` reader - RWA, interrupt enable for battery lower voltage"]
pub type RbBatLowerIeR = crate::BitReader;
#[doc = "Field `RB_BAT_LOWER_IE` writer - RWA, interrupt enable for battery lower voltage"]
pub type RbBatLowerIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_BAT_LOW_IE` reader - RWA, interrupt enable for battery low voltage"]
pub type RbBatLowIeR = crate::BitReader;
#[doc = "Field `RB_BAT_LOW_IE` writer - RWA, interrupt enable for battery low voltage"]
pub type RbBatLowIeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RWA, battery voltage detector enable/select monitor threshold voltage"]
    #[inline(always)]
    pub fn rb_bat_det_en__rb_bat_low_vthx(&self) -> RbBatDetEn_RbBatLowVthxR {
        RbBatDetEn_RbBatLowVthxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RWA, battery voltage monitor enable under sleep mode"]
    #[inline(always)]
    pub fn rb_bat_mon_en(&self) -> RbBatMonEnR {
        RbBatMonEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RWA, interrupt enable for battery lower voltage"]
    #[inline(always)]
    pub fn rb_bat_lower_ie(&self) -> RbBatLowerIeR {
        RbBatLowerIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RWA, interrupt enable for battery low voltage"]
    #[inline(always)]
    pub fn rb_bat_low_ie(&self) -> RbBatLowIeR {
        RbBatLowIeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RWA, battery voltage detector enable/select monitor threshold voltage"]
    #[inline(always)]
    pub fn rb_bat_det_en__rb_bat_low_vthx(
        &mut self,
    ) -> RbBatDetEn_RbBatLowVthxW<'_, R8BatDetCtrlSpec> {
        RbBatDetEn_RbBatLowVthxW::new(self, 0)
    }
    #[doc = "Bit 1 - RWA, battery voltage monitor enable under sleep mode"]
    #[inline(always)]
    pub fn rb_bat_mon_en(&mut self) -> RbBatMonEnW<'_, R8BatDetCtrlSpec> {
        RbBatMonEnW::new(self, 1)
    }
    #[doc = "Bit 2 - RWA, interrupt enable for battery lower voltage"]
    #[inline(always)]
    pub fn rb_bat_lower_ie(&mut self) -> RbBatLowerIeW<'_, R8BatDetCtrlSpec> {
        RbBatLowerIeW::new(self, 2)
    }
    #[doc = "Bit 3 - RWA, interrupt enable for battery low voltage"]
    #[inline(always)]
    pub fn rb_bat_low_ie(&mut self) -> RbBatLowIeW<'_, R8BatDetCtrlSpec> {
        RbBatLowIeW::new(self, 3)
    }
}
#[doc = "RWA, battery voltage detector control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_bat_det_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_bat_det_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8BatDetCtrlSpec;
impl crate::RegisterSpec for R8BatDetCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_bat_det_ctrl::R`](R) reader structure"]
impl crate::Readable for R8BatDetCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_bat_det_ctrl::W`](W) writer structure"]
impl crate::Writable for R8BatDetCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_BAT_DET_CTRL to value 0"]
impl crate::Resettable for R8BatDetCtrlSpec {}
