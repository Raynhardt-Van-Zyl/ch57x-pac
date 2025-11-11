#[doc = "Register `R16_OSC_CAL_CNT` reader"]
pub type R = crate::R<R16OscCalCntSpec>;
#[doc = "Field `RB_OSC_CAL_CNT` reader - RO, system clock count value for 32KHz 5 cycles"]
pub type RbOscCalCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - RO, system clock count value for 32KHz 5 cycles"]
    #[inline(always)]
    pub fn rb_osc_cal_cnt(&self) -> RbOscCalCntR {
        RbOscCalCntR::new(self.bits & 0x3fff)
    }
}
#[doc = "RO, system clock count value for 32KHz 5 cycles\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_osc_cal_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16OscCalCntSpec;
impl crate::RegisterSpec for R16OscCalCntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_osc_cal_cnt::R`](R) reader structure"]
impl crate::Readable for R16OscCalCntSpec {}
#[doc = "`reset()` method sets R16_OSC_CAL_CNT to value 0"]
impl crate::Resettable for R16OscCalCntSpec {}
