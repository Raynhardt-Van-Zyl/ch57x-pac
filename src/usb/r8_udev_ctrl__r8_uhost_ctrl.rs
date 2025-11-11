#[doc = "Register `R8_UDEV_CTRL__R8_UHOST_CTRL` reader"]
pub type R = crate::R<R8UdevCtrl_R8UhostCtrlSpec>;
#[doc = "Register `R8_UDEV_CTRL__R8_UHOST_CTRL` writer"]
pub type W = crate::W<R8UdevCtrl_R8UhostCtrlSpec>;
#[doc = "Field `RB_UD_PORT_EN__RB_UH_PORT_EN` reader - enable USB physical port I-O: 0=disable, 1=enable;enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached"]
pub type RbUdPortEn_RbUhPortEnR = crate::BitReader;
#[doc = "Field `RB_UD_PORT_EN__RB_UH_PORT_EN` writer - enable USB physical port I-O: 0=disable, 1=enable;enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached"]
pub type RbUdPortEn_RbUhPortEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UD_GP_BIT__RB_UH_BUS_RESET` reader - general purpose bit;control USB bus reset: 0=normal, 1=force bus reset"]
pub type RbUdGpBit_RbUhBusResetR = crate::BitReader;
#[doc = "Field `RB_UD_GP_BIT__RB_UH_BUS_RESET` writer - general purpose bit;control USB bus reset: 0=normal, 1=force bus reset"]
pub type RbUdGpBit_RbUhBusResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UD_LOW_SPEED__RB_UH_LOW_SPEED` reader - enable USB physical port low speed: 0=full speed, 1=low speed;enable USB port low speed: 0=full speed, 1=low speed"]
pub type RbUdLowSpeed_RbUhLowSpeedR = crate::BitReader;
#[doc = "Field `RB_UD_LOW_SPEED__RB_UH_LOW_SPEED` writer - enable USB physical port low speed: 0=full speed, 1=low speed;enable USB port low speed: 0=full speed, 1=low speed"]
pub type RbUdLowSpeed_RbUhLowSpeedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UD_DM_PIN__RB_UH_DM_PIN` reader - ReadOnly: indicate current UDM pin level"]
pub type RbUdDmPin_RbUhDmPinR = crate::BitReader;
#[doc = "Field `RB_UD_DP_PIN__RB_UH_DP_PIN` reader - ReadOnly: indicate current UDP pin level"]
pub type RbUdDpPin_RbUhDpPinR = crate::BitReader;
#[doc = "Field `RB_UD_PD_DIS__RB_UH_PD_DIS` reader - disable USB UDP-UDM pulldown resistance: 0=enable pulldown, 1=disable"]
pub type RbUdPdDis_RbUhPdDisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - enable USB physical port I-O: 0=disable, 1=enable;enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached"]
    #[inline(always)]
    pub fn rb_ud_port_en__rb_uh_port_en(&self) -> RbUdPortEn_RbUhPortEnR {
        RbUdPortEn_RbUhPortEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - general purpose bit;control USB bus reset: 0=normal, 1=force bus reset"]
    #[inline(always)]
    pub fn rb_ud_gp_bit__rb_uh_bus_reset(&self) -> RbUdGpBit_RbUhBusResetR {
        RbUdGpBit_RbUhBusResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable USB physical port low speed: 0=full speed, 1=low speed;enable USB port low speed: 0=full speed, 1=low speed"]
    #[inline(always)]
    pub fn rb_ud_low_speed__rb_uh_low_speed(&self) -> RbUdLowSpeed_RbUhLowSpeedR {
        RbUdLowSpeed_RbUhLowSpeedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ReadOnly: indicate current UDM pin level"]
    #[inline(always)]
    pub fn rb_ud_dm_pin__rb_uh_dm_pin(&self) -> RbUdDmPin_RbUhDmPinR {
        RbUdDmPin_RbUhDmPinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ReadOnly: indicate current UDP pin level"]
    #[inline(always)]
    pub fn rb_ud_dp_pin__rb_uh_dp_pin(&self) -> RbUdDpPin_RbUhDpPinR {
        RbUdDpPin_RbUhDpPinR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - disable USB UDP-UDM pulldown resistance: 0=enable pulldown, 1=disable"]
    #[inline(always)]
    pub fn rb_ud_pd_dis__rb_uh_pd_dis(&self) -> RbUdPdDis_RbUhPdDisR {
        RbUdPdDis_RbUhPdDisR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable USB physical port I-O: 0=disable, 1=enable;enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached"]
    #[inline(always)]
    pub fn rb_ud_port_en__rb_uh_port_en(
        &mut self,
    ) -> RbUdPortEn_RbUhPortEnW<'_, R8UdevCtrl_R8UhostCtrlSpec> {
        RbUdPortEn_RbUhPortEnW::new(self, 0)
    }
    #[doc = "Bit 1 - general purpose bit;control USB bus reset: 0=normal, 1=force bus reset"]
    #[inline(always)]
    pub fn rb_ud_gp_bit__rb_uh_bus_reset(
        &mut self,
    ) -> RbUdGpBit_RbUhBusResetW<'_, R8UdevCtrl_R8UhostCtrlSpec> {
        RbUdGpBit_RbUhBusResetW::new(self, 1)
    }
    #[doc = "Bit 2 - enable USB physical port low speed: 0=full speed, 1=low speed;enable USB port low speed: 0=full speed, 1=low speed"]
    #[inline(always)]
    pub fn rb_ud_low_speed__rb_uh_low_speed(
        &mut self,
    ) -> RbUdLowSpeed_RbUhLowSpeedW<'_, R8UdevCtrl_R8UhostCtrlSpec> {
        RbUdLowSpeed_RbUhLowSpeedW::new(self, 2)
    }
}
#[doc = "USB device physical prot control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_udev_ctrl__r8_uhost_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_udev_ctrl__r8_uhost_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UdevCtrl_R8UhostCtrlSpec;
impl crate::RegisterSpec for R8UdevCtrl_R8UhostCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_udev_ctrl__r8_uhost_ctrl::R`](R) reader structure"]
impl crate::Readable for R8UdevCtrl_R8UhostCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_udev_ctrl__r8_uhost_ctrl::W`](W) writer structure"]
impl crate::Writable for R8UdevCtrl_R8UhostCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UDEV_CTRL__R8_UHOST_CTRL to value 0"]
impl crate::Resettable for R8UdevCtrl_R8UhostCtrlSpec {}
