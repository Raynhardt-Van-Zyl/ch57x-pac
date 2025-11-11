#[doc = "Register `R8_USB_INT_ST` reader"]
pub type R = crate::R<R8UsbIntStSpec>;
#[doc = "Field `MASK_UIS_H_RES__MASK_UIS_ENDP` reader - RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode"]
pub type MaskUisHRes_MaskUisEndpR = crate::FieldReader;
#[doc = "Field `MASK_UIS_TOKEN` reader - RO, bit mask of current token PID code received for USB device mode"]
pub type MaskUisTokenR = crate::FieldReader;
#[doc = "Field `RB_UIS_TOG_OK` reader - RO, indicate current USB transfer toggle is OK"]
pub type RbUisTogOkR = crate::BitReader;
#[doc = "Field `RB_UIS_SETUP_ACT` reader - RO, indicate current USB transfer is NAK received for USB device mode"]
pub type RbUisSetupActR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode"]
    #[inline(always)]
    pub fn mask_uis_h_res__mask_uis_endp(&self) -> MaskUisHRes_MaskUisEndpR {
        MaskUisHRes_MaskUisEndpR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - RO, bit mask of current token PID code received for USB device mode"]
    #[inline(always)]
    pub fn mask_uis_token(&self) -> MaskUisTokenR {
        MaskUisTokenR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - RO, indicate current USB transfer toggle is OK"]
    #[inline(always)]
    pub fn rb_uis_tog_ok(&self) -> RbUisTogOkR {
        RbUisTogOkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, indicate current USB transfer is NAK received for USB device mode"]
    #[inline(always)]
    pub fn rb_uis_setup_act(&self) -> RbUisSetupActR {
        RbUisSetupActR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "USB interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbIntStSpec;
impl crate::RegisterSpec for R8UsbIntStSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_int_st::R`](R) reader structure"]
impl crate::Readable for R8UsbIntStSpec {}
#[doc = "`reset()` method sets R8_USB_INT_ST to value 0"]
impl crate::Resettable for R8UsbIntStSpec {}
