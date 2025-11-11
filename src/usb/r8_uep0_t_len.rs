#[doc = "Register `R8_UEP0_T_LEN` reader"]
pub type R = crate::R<R8Uep0TLenSpec>;
#[doc = "Register `R8_UEP0_T_LEN` writer"]
pub type W = crate::W<R8Uep0TLenSpec>;
#[doc = "Field `R8_UEP0_T_LEN` reader - endpoint 0 transmittal length"]
pub type R8Uep0TLenR = crate::FieldReader;
#[doc = "Field `R8_UEP0_T_LEN` writer - endpoint 0 transmittal length"]
pub type R8Uep0TLenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - endpoint 0 transmittal length"]
    #[inline(always)]
    pub fn r8_uep0_t_len(&self) -> R8Uep0TLenR {
        R8Uep0TLenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - endpoint 0 transmittal length"]
    #[inline(always)]
    pub fn r8_uep0_t_len(&mut self) -> R8Uep0TLenW<'_, R8Uep0TLenSpec> {
        R8Uep0TLenW::new(self, 0)
    }
}
#[doc = "endpoint 0 transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep0_t_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep0_t_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep0TLenSpec;
impl crate::RegisterSpec for R8Uep0TLenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep0_t_len::R`](R) reader structure"]
impl crate::Readable for R8Uep0TLenSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep0_t_len::W`](W) writer structure"]
impl crate::Writable for R8Uep0TLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP0_T_LEN to value 0"]
impl crate::Resettable for R8Uep0TLenSpec {}
