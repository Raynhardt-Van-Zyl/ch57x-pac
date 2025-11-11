#[doc = "Register `R8_USB_DEV_AD` reader"]
pub type R = crate::R<R8UsbDevAdSpec>;
#[doc = "Register `R8_USB_DEV_AD` writer"]
pub type W = crate::W<R8UsbDevAdSpec>;
#[doc = "Field `MASK_USB_ADDR` reader - bit mask for USB device address"]
pub type MaskUsbAddrR = crate::FieldReader;
#[doc = "Field `MASK_USB_ADDR` writer - bit mask for USB device address"]
pub type MaskUsbAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RB_UDA_GP_BIT` reader - general purpose bit"]
pub type RbUdaGpBitR = crate::BitReader;
#[doc = "Field `RB_UDA_GP_BIT` writer - general purpose bit"]
pub type RbUdaGpBitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - bit mask for USB device address"]
    #[inline(always)]
    pub fn mask_usb_addr(&self) -> MaskUsbAddrR {
        MaskUsbAddrR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - general purpose bit"]
    #[inline(always)]
    pub fn rb_uda_gp_bit(&self) -> RbUdaGpBitR {
        RbUdaGpBitR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - bit mask for USB device address"]
    #[inline(always)]
    pub fn mask_usb_addr(&mut self) -> MaskUsbAddrW<'_, R8UsbDevAdSpec> {
        MaskUsbAddrW::new(self, 0)
    }
    #[doc = "Bit 7 - general purpose bit"]
    #[inline(always)]
    pub fn rb_uda_gp_bit(&mut self) -> RbUdaGpBitW<'_, R8UsbDevAdSpec> {
        RbUdaGpBitW::new(self, 7)
    }
}
#[doc = "USB device address\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_dev_ad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_dev_ad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbDevAdSpec;
impl crate::RegisterSpec for R8UsbDevAdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_dev_ad::R`](R) reader structure"]
impl crate::Readable for R8UsbDevAdSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_usb_dev_ad::W`](W) writer structure"]
impl crate::Writable for R8UsbDevAdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_USB_DEV_AD to value 0"]
impl crate::Resettable for R8UsbDevAdSpec {}
