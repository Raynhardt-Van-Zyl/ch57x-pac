#[doc = "Register `R32_PFIC_GISR` reader"]
pub type R = crate::R<R32PficGisrSpec>;
#[doc = "Field `NESTSTA` reader - RO,NESTSTA"]
pub type NeststaR = crate::FieldReader;
#[doc = "Field `GACTSTA` reader - RO,GACTSTA"]
pub type GactstaR = crate::BitReader;
#[doc = "Field `GPENDSTA` reader - RO,GPENDSTA"]
pub type GpendstaR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - RO,NESTSTA"]
    #[inline(always)]
    pub fn neststa(&self) -> NeststaR {
        NeststaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - RO,GACTSTA"]
    #[inline(always)]
    pub fn gactsta(&self) -> GactstaR {
        GactstaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RO,GPENDSTA"]
    #[inline(always)]
    pub fn gpendsta(&self) -> GpendstaR {
        GpendstaR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Global Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_gisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficGisrSpec;
impl crate::RegisterSpec for R32PficGisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_gisr::R`](R) reader structure"]
impl crate::Readable for R32PficGisrSpec {}
#[doc = "`reset()` method sets R32_PFIC_GISR to value 0"]
impl crate::Resettable for R32PficGisrSpec {}
