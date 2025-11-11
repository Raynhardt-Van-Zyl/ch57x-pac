#[doc = "Register `R8_USB_INT_EN` reader"]
pub type R = crate::R<R8UsbIntEnSpec>;
#[doc = "Register `R8_USB_INT_EN` writer"]
pub type W = crate::W<R8UsbIntEnSpec>;
#[doc = "Field `RB_UIE_BUS_RST__RB_UIE_DETECT` reader - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
pub type RbUieBusRst_RbUieDetectR = crate::BitReader;
#[doc = "Field `RB_UIE_BUS_RST__RB_UIE_DETECT` writer - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
pub type RbUieBusRst_RbUieDetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UIE_TRANSFER` reader - enable interrupt for USB transfer completion"]
pub type RbUieTransferR = crate::BitReader;
#[doc = "Field `RB_UIE_TRANSFER` writer - enable interrupt for USB transfer completion"]
pub type RbUieTransferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UIE_SUSPEND` reader - enable interrupt for USB suspend or resume event"]
pub type RbUieSuspendR = crate::BitReader;
#[doc = "Field `RB_UIE_SUSPEND` writer - enable interrupt for USB suspend or resume event"]
pub type RbUieSuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UIE_HST_SOF` reader - enable interrupt for host SOF timer action for USB host mode"]
pub type RbUieHstSofR = crate::BitReader;
#[doc = "Field `RB_UIE_HST_SOF` writer - enable interrupt for host SOF timer action for USB host mode"]
pub type RbUieHstSofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UIE_FIFO_OV` reader - enable interrupt for FIFO overflow"]
pub type RbUieFifoOvR = crate::BitReader;
#[doc = "Field `RB_UIE_FIFO_OV` writer - enable interrupt for FIFO overflow"]
pub type RbUieFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UIE_DEV_NAK` reader - enable interrupt for NAK responded for USB device mode"]
pub type RbUieDevNakR = crate::BitReader;
#[doc = "Field `RB_UIE_DEV_NAK` writer - enable interrupt for NAK responded for USB device mode"]
pub type RbUieDevNakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UIE_DEV_SOF` reader - enable interrupt for SOF received for USB device mode"]
pub type RbUieDevSofR = crate::BitReader;
#[doc = "Field `RB_UIE_DEV_SOF` writer - enable interrupt for SOF received for USB device mode"]
pub type RbUieDevSofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    pub fn rb_uie_bus_rst__rb_uie_detect(&self) -> RbUieBusRst_RbUieDetectR {
        RbUieBusRst_RbUieDetectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn rb_uie_transfer(&self) -> RbUieTransferR {
        RbUieTransferR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn rb_uie_suspend(&self) -> RbUieSuspendR {
        RbUieSuspendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable interrupt for host SOF timer action for USB host mode"]
    #[inline(always)]
    pub fn rb_uie_hst_sof(&self) -> RbUieHstSofR {
        RbUieHstSofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn rb_uie_fifo_ov(&self) -> RbUieFifoOvR {
        RbUieFifoOvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_uie_dev_nak(&self) -> RbUieDevNakR {
        RbUieDevNakR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable interrupt for SOF received for USB device mode"]
    #[inline(always)]
    pub fn rb_uie_dev_sof(&self) -> RbUieDevSofR {
        RbUieDevSofR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    pub fn rb_uie_bus_rst__rb_uie_detect(
        &mut self,
    ) -> RbUieBusRst_RbUieDetectW<'_, R8UsbIntEnSpec> {
        RbUieBusRst_RbUieDetectW::new(self, 0)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn rb_uie_transfer(&mut self) -> RbUieTransferW<'_, R8UsbIntEnSpec> {
        RbUieTransferW::new(self, 1)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn rb_uie_suspend(&mut self) -> RbUieSuspendW<'_, R8UsbIntEnSpec> {
        RbUieSuspendW::new(self, 2)
    }
    #[doc = "Bit 3 - enable interrupt for host SOF timer action for USB host mode"]
    #[inline(always)]
    pub fn rb_uie_hst_sof(&mut self) -> RbUieHstSofW<'_, R8UsbIntEnSpec> {
        RbUieHstSofW::new(self, 3)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn rb_uie_fifo_ov(&mut self) -> RbUieFifoOvW<'_, R8UsbIntEnSpec> {
        RbUieFifoOvW::new(self, 4)
    }
    #[doc = "Bit 6 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_uie_dev_nak(&mut self) -> RbUieDevNakW<'_, R8UsbIntEnSpec> {
        RbUieDevNakW::new(self, 6)
    }
    #[doc = "Bit 7 - enable interrupt for SOF received for USB device mode"]
    #[inline(always)]
    pub fn rb_uie_dev_sof(&mut self) -> RbUieDevSofW<'_, R8UsbIntEnSpec> {
        RbUieDevSofW::new(self, 7)
    }
}
#[doc = "USB interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbIntEnSpec;
impl crate::RegisterSpec for R8UsbIntEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_int_en::R`](R) reader structure"]
impl crate::Readable for R8UsbIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_usb_int_en::W`](W) writer structure"]
impl crate::Writable for R8UsbIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_USB_INT_EN to value 0"]
impl crate::Resettable for R8UsbIntEnSpec {}
