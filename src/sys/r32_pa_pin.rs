#[doc = "Register `R32_PA_PIN` reader"]
pub type R = crate::R<R32PaPinSpec>;
#[doc = "Field `R8_PA_PIN_0` reader - GPIO PA input byte 0"]
pub type R8PaPin0R = crate::FieldReader;
#[doc = "Field `R8_PA_PIN_1` reader - GPIO PA input byte 1"]
pub type R8PaPin1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - GPIO PA input byte 0"]
    #[inline(always)]
    pub fn r8_pa_pin_0(&self) -> R8PaPin0R {
        R8PaPin0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO PA input byte 1"]
    #[inline(always)]
    pub fn r8_pa_pin_1(&self) -> R8PaPin1R {
        R8PaPin1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "RO, GPIO PA input\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaPinSpec;
impl crate::RegisterSpec for R32PaPinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_pin::R`](R) reader structure"]
impl crate::Readable for R32PaPinSpec {}
#[doc = "`reset()` method sets R32_PA_PIN to value 0"]
impl crate::Resettable for R32PaPinSpec {}
