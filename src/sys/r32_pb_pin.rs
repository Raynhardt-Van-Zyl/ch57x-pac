#[doc = "Register `R32_PB_PIN` reader"]
pub type R = crate::R<R32PbPinSpec>;
#[doc = "Field `R8_PB_PIN_0` reader - GPIO PB input byte 0"]
pub type R8PbPin0R = crate::FieldReader;
#[doc = "Field `R8_PB_PIN_1` reader - GPIO PB input byte 1"]
pub type R8PbPin1R = crate::FieldReader;
#[doc = "Field `R8_PB_PIN_2` reader - GPIO PB input byte 2"]
pub type R8PbPin2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - GPIO PB input byte 0"]
    #[inline(always)]
    pub fn r8_pb_pin_0(&self) -> R8PbPin0R {
        R8PbPin0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO PB input byte 1"]
    #[inline(always)]
    pub fn r8_pb_pin_1(&self) -> R8PbPin1R {
        R8PbPin1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO PB input byte 2"]
    #[inline(always)]
    pub fn r8_pb_pin_2(&self) -> R8PbPin2R {
        R8PbPin2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "RO, GPIO PB input\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbPinSpec;
impl crate::RegisterSpec for R32PbPinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_pin::R`](R) reader structure"]
impl crate::Readable for R32PbPinSpec {}
#[doc = "`reset()` method sets R32_PB_PIN to value 0"]
impl crate::Resettable for R32PbPinSpec {}
