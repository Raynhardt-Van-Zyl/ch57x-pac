#[doc = "Register `R8_USB_RX_LEN` reader"]
pub type R = crate::R<R8UsbRxLenSpec>;
#[doc = "Field `R8_USB_RX_LEN` reader - RO,USB receiving length"]
pub type R8UsbRxLenR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RO,USB receiving length"]
    #[inline(always)]
    pub fn r8_usb_rx_len(&self) -> R8UsbRxLenR {
        R8UsbRxLenR::new(self.bits)
    }
}
#[doc = "USB receiving length\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_rx_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbRxLenSpec;
impl crate::RegisterSpec for R8UsbRxLenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_rx_len::R`](R) reader structure"]
impl crate::Readable for R8UsbRxLenSpec {}
#[doc = "`reset()` method sets R8_USB_RX_LEN to value 0"]
impl crate::Resettable for R8UsbRxLenSpec {}
