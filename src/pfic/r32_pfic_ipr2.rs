#[doc = "Register `R32_PFIC_IPR2` reader"]
pub type R = crate::R<R32PficIpr2Spec>;
#[doc = "Field `PENDSTA` reader - PENDSTA"]
pub type PendstaR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - PENDSTA"]
    #[inline(always)]
    pub fn pendsta(&self) -> PendstaR {
        PendstaR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "RO,Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ipr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIpr2Spec;
impl crate::RegisterSpec for R32PficIpr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_ipr2::R`](R) reader structure"]
impl crate::Readable for R32PficIpr2Spec {}
#[doc = "`reset()` method sets R32_PFIC_IPR2 to value 0"]
impl crate::Resettable for R32PficIpr2Spec {}
