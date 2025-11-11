#[doc = "Register `R8_USB_MIS_ST` reader"]
pub type R = crate::R<R8UsbMisStSpec>;
#[doc = "Field `RB_UMS_DEV_ATTACH` reader - RO, indicate device attached status on USB host"]
pub type RbUmsDevAttachR = crate::BitReader;
#[doc = "Field `RB_UMS_DM_LEVEL` reader - RO, indicate UDM level saved at device attached to USB host"]
pub type RbUmsDmLevelR = crate::BitReader;
#[doc = "Field `RB_UMS_SUSPEND` reader - RO, indicate USB suspend status"]
pub type RbUmsSuspendR = crate::BitReader;
#[doc = "Field `RB_UMS_BUS_RESET` reader - RO, indicate USB bus reset status"]
pub type RbUmsBusResetR = crate::BitReader;
#[doc = "Field `RB_UMS_R_FIFO_RDY` reader - RO, indicate USB receiving FIFO ready status (not empty)"]
pub type RbUmsRFifoRdyR = crate::BitReader;
#[doc = "Field `RB_UMS_SIE_FREE` reader - RO, indicate USB SIE free status"]
pub type RbUmsSieFreeR = crate::BitReader;
#[doc = "Field `RB_UMS_SOF_ACT` reader - RO, indicate host SOF timer action status for USB host"]
pub type RbUmsSofActR = crate::BitReader;
#[doc = "Field `RB_UMS_SOF_PRES` reader - RO, indicate host SOF timer presage status"]
pub type RbUmsSofPresR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RO, indicate device attached status on USB host"]
    #[inline(always)]
    pub fn rb_ums_dev_attach(&self) -> RbUmsDevAttachR {
        RbUmsDevAttachR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RO, indicate UDM level saved at device attached to USB host"]
    #[inline(always)]
    pub fn rb_ums_dm_level(&self) -> RbUmsDmLevelR {
        RbUmsDmLevelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RO, indicate USB suspend status"]
    #[inline(always)]
    pub fn rb_ums_suspend(&self) -> RbUmsSuspendR {
        RbUmsSuspendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RO, indicate USB bus reset status"]
    #[inline(always)]
    pub fn rb_ums_bus_reset(&self) -> RbUmsBusResetR {
        RbUmsBusResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RO, indicate USB receiving FIFO ready status (not empty)"]
    #[inline(always)]
    pub fn rb_ums_r_fifo_rdy(&self) -> RbUmsRFifoRdyR {
        RbUmsRFifoRdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RO, indicate USB SIE free status"]
    #[inline(always)]
    pub fn rb_ums_sie_free(&self) -> RbUmsSieFreeR {
        RbUmsSieFreeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RO, indicate host SOF timer action status for USB host"]
    #[inline(always)]
    pub fn rb_ums_sof_act(&self) -> RbUmsSofActR {
        RbUmsSofActR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, indicate host SOF timer presage status"]
    #[inline(always)]
    pub fn rb_ums_sof_pres(&self) -> RbUmsSofPresR {
        RbUmsSofPresR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "USB miscellaneous status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_mis_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbMisStSpec;
impl crate::RegisterSpec for R8UsbMisStSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_mis_st::R`](R) reader structure"]
impl crate::Readable for R8UsbMisStSpec {}
#[doc = "`reset()` method sets R8_USB_MIS_ST to value 0"]
impl crate::Resettable for R8UsbMisStSpec {}
