#[doc = "Register `R8_USB_CTRL` reader"]
pub type R = crate::R<R8UsbCtrlSpec>;
#[doc = "Register `R8_USB_CTRL` writer"]
pub type W = crate::W<R8UsbCtrlSpec>;
#[doc = "Field `RB_UC_DMA_EN` reader - DMA enable and DMA interrupt enable for USB"]
pub type RbUcDmaEnR = crate::BitReader;
#[doc = "Field `RB_UC_DMA_EN` writer - DMA enable and DMA interrupt enable for USB"]
pub type RbUcDmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UC_CLR_ALL` reader - force clear FIFO and count of USB"]
pub type RbUcClrAllR = crate::BitReader;
#[doc = "Field `RB_UC_CLR_ALL` writer - force clear FIFO and count of USB"]
pub type RbUcClrAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UC_RESET_SIE` reader - force reset USB SIE, need software clear"]
pub type RbUcResetSieR = crate::BitReader;
#[doc = "Field `RB_UC_RESET_SIE` writer - force reset USB SIE, need software clear"]
pub type RbUcResetSieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UC_INT_BUSY` reader - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
pub type RbUcIntBusyR = crate::BitReader;
#[doc = "Field `RB_UC_INT_BUSY` writer - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
pub type RbUcIntBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_UC_SYS_CTRL` reader - bit mask of USB system control"]
pub type MaskUcSysCtrlR = crate::FieldReader;
#[doc = "Field `MASK_UC_SYS_CTRL` writer - bit mask of USB system control"]
pub type MaskUcSysCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_UC_DEV_PU_EN` reader - USB device enable and internal pullup resistance enable"]
pub type RbUcDevPuEnR = crate::BitReader;
#[doc = "Field `RB_UC_DEV_PU_EN` writer - USB device enable and internal pullup resistance enable"]
pub type RbUcDevPuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UC_LOW_SPEED` reader - enable USB low speed: 0=12Mbps, 1=1.5Mbps"]
pub type RbUcLowSpeedR = crate::BitReader;
#[doc = "Field `RB_UC_LOW_SPEED` writer - enable USB low speed: 0=12Mbps, 1=1.5Mbps"]
pub type RbUcLowSpeedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UC_HOST_MODE` reader - enable USB host mode: 0=device mode, 1=host mode"]
pub type RbUcHostModeR = crate::BitReader;
#[doc = "Field `RB_UC_HOST_MODE` writer - enable USB host mode: 0=device mode, 1=host mode"]
pub type RbUcHostModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA enable and DMA interrupt enable for USB"]
    #[inline(always)]
    pub fn rb_uc_dma_en(&self) -> RbUcDmaEnR {
        RbUcDmaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    pub fn rb_uc_clr_all(&self) -> RbUcClrAllR {
        RbUcClrAllR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - force reset USB SIE, need software clear"]
    #[inline(always)]
    pub fn rb_uc_reset_sie(&self) -> RbUcResetSieR {
        RbUcResetSieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
    #[inline(always)]
    pub fn rb_uc_int_busy(&self) -> RbUcIntBusyR {
        RbUcIntBusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - bit mask of USB system control"]
    #[inline(always)]
    pub fn mask_uc_sys_ctrl(&self) -> MaskUcSysCtrlR {
        MaskUcSysCtrlR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 5 - USB device enable and internal pullup resistance enable"]
    #[inline(always)]
    pub fn rb_uc_dev_pu_en(&self) -> RbUcDevPuEnR {
        RbUcDevPuEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable USB low speed: 0=12Mbps, 1=1.5Mbps"]
    #[inline(always)]
    pub fn rb_uc_low_speed(&self) -> RbUcLowSpeedR {
        RbUcLowSpeedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable USB host mode: 0=device mode, 1=host mode"]
    #[inline(always)]
    pub fn rb_uc_host_mode(&self) -> RbUcHostModeR {
        RbUcHostModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA enable and DMA interrupt enable for USB"]
    #[inline(always)]
    pub fn rb_uc_dma_en(&mut self) -> RbUcDmaEnW<'_, R8UsbCtrlSpec> {
        RbUcDmaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    pub fn rb_uc_clr_all(&mut self) -> RbUcClrAllW<'_, R8UsbCtrlSpec> {
        RbUcClrAllW::new(self, 1)
    }
    #[doc = "Bit 2 - force reset USB SIE, need software clear"]
    #[inline(always)]
    pub fn rb_uc_reset_sie(&mut self) -> RbUcResetSieW<'_, R8UsbCtrlSpec> {
        RbUcResetSieW::new(self, 2)
    }
    #[doc = "Bit 3 - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
    #[inline(always)]
    pub fn rb_uc_int_busy(&mut self) -> RbUcIntBusyW<'_, R8UsbCtrlSpec> {
        RbUcIntBusyW::new(self, 3)
    }
    #[doc = "Bits 4:5 - bit mask of USB system control"]
    #[inline(always)]
    pub fn mask_uc_sys_ctrl(&mut self) -> MaskUcSysCtrlW<'_, R8UsbCtrlSpec> {
        MaskUcSysCtrlW::new(self, 4)
    }
    #[doc = "Bit 5 - USB device enable and internal pullup resistance enable"]
    #[inline(always)]
    pub fn rb_uc_dev_pu_en(&mut self) -> RbUcDevPuEnW<'_, R8UsbCtrlSpec> {
        RbUcDevPuEnW::new(self, 5)
    }
    #[doc = "Bit 6 - enable USB low speed: 0=12Mbps, 1=1.5Mbps"]
    #[inline(always)]
    pub fn rb_uc_low_speed(&mut self) -> RbUcLowSpeedW<'_, R8UsbCtrlSpec> {
        RbUcLowSpeedW::new(self, 6)
    }
    #[doc = "Bit 7 - enable USB host mode: 0=device mode, 1=host mode"]
    #[inline(always)]
    pub fn rb_uc_host_mode(&mut self) -> RbUcHostModeW<'_, R8UsbCtrlSpec> {
        RbUcHostModeW::new(self, 7)
    }
}
#[doc = "USB base control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbCtrlSpec;
impl crate::RegisterSpec for R8UsbCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_ctrl::R`](R) reader structure"]
impl crate::Readable for R8UsbCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_usb_ctrl::W`](W) writer structure"]
impl crate::Writable for R8UsbCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_USB_CTRL to value 0x06"]
impl crate::Resettable for R8UsbCtrlSpec {
    const RESET_VALUE: u8 = 0x06;
}
