#[doc = "Register `R8_USB_INT_FG` reader"]
pub type R = crate::R<R8UsbIntFgSpec>;
#[doc = "Register `R8_USB_INT_FG` writer"]
pub type W = crate::W<R8UsbIntFgSpec>;
#[doc = "Field `RB_UIF_BUS_RST__RB_UIF_DETECT` reader - RW,bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
pub type RbUifBusRst_RbUifDetectR = crate::BitReader;
#[doc = "Field `RB_UIF_BUS_RST__RB_UIF_DETECT` writer - RW,bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
pub type RbUifBusRst_RbUifDetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UIF_TRANSFER` reader - RW,USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
pub type RbUifTransferR = crate::BitReader;
#[doc = "Field `RB_UIF_TRANSFER` writer - RW,USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
pub type RbUifTransferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UIF_SUSPEND` reader - RW,USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
pub type RbUifSuspendR = crate::BitReader;
#[doc = "Field `RB_UIF_SUSPEND` writer - RW,USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
pub type RbUifSuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UIF_HST_SOF` reader - RW,host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
pub type RbUifHstSofR = crate::BitReader;
#[doc = "Field `RB_UIF_HST_SOF` writer - RW,host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
pub type RbUifHstSofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UIF_FIFO_OV` reader - RW,FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
pub type RbUifFifoOvR = crate::BitReader;
#[doc = "Field `RB_UIF_FIFO_OV` writer - RW,FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
pub type RbUifFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_U_SIE_FREE` reader - RO, indicate USB SIE free status"]
pub type RbUSieFreeR = crate::BitReader;
#[doc = "Field `RB_U_TOG_OK` reader - RO, indicate current USB transfer toggle is OK"]
pub type RbUTogOkR = crate::BitReader;
#[doc = "Field `RB_U_IS_NAK` reader - RO, indicate current USB transfer is NAK received"]
pub type RbUIsNakR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RW,bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_bus_rst__rb_uif_detect(&self) -> RbUifBusRst_RbUifDetectR {
        RbUifBusRst_RbUifDetectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW,USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_transfer(&self) -> RbUifTransferR {
        RbUifTransferR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RW,USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_suspend(&self) -> RbUifSuspendR {
        RbUifSuspendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RW,host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_hst_sof(&self) -> RbUifHstSofR {
        RbUifHstSofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RW,FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_fifo_ov(&self) -> RbUifFifoOvR {
        RbUifFifoOvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RO, indicate USB SIE free status"]
    #[inline(always)]
    pub fn rb_u_sie_free(&self) -> RbUSieFreeR {
        RbUSieFreeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RO, indicate current USB transfer toggle is OK"]
    #[inline(always)]
    pub fn rb_u_tog_ok(&self) -> RbUTogOkR {
        RbUTogOkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, indicate current USB transfer is NAK received"]
    #[inline(always)]
    pub fn rb_u_is_nak(&self) -> RbUIsNakR {
        RbUIsNakR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW,bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_bus_rst__rb_uif_detect(
        &mut self,
    ) -> RbUifBusRst_RbUifDetectW<'_, R8UsbIntFgSpec> {
        RbUifBusRst_RbUifDetectW::new(self, 0)
    }
    #[doc = "Bit 1 - RW,USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_transfer(&mut self) -> RbUifTransferW<'_, R8UsbIntFgSpec> {
        RbUifTransferW::new(self, 1)
    }
    #[doc = "Bit 2 - RW,USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_suspend(&mut self) -> RbUifSuspendW<'_, R8UsbIntFgSpec> {
        RbUifSuspendW::new(self, 2)
    }
    #[doc = "Bit 3 - RW,host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_hst_sof(&mut self) -> RbUifHstSofW<'_, R8UsbIntFgSpec> {
        RbUifHstSofW::new(self, 3)
    }
    #[doc = "Bit 4 - RW,FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_fifo_ov(&mut self) -> RbUifFifoOvW<'_, R8UsbIntFgSpec> {
        RbUifFifoOvW::new(self, 4)
    }
}
#[doc = "USB interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_fg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_int_fg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbIntFgSpec;
impl crate::RegisterSpec for R8UsbIntFgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_int_fg::R`](R) reader structure"]
impl crate::Readable for R8UsbIntFgSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_usb_int_fg::W`](W) writer structure"]
impl crate::Writable for R8UsbIntFgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_USB_INT_FG to value 0x20"]
impl crate::Resettable for R8UsbIntFgSpec {
    const RESET_VALUE: u8 = 0x20;
}
