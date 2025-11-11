#[doc = "Register `R16_PB_INT_MODE` reader"]
pub type R = crate::R<R16PbIntModeSpec>;
#[doc = "Register `R16_PB_INT_MODE` writer"]
pub type W = crate::W<R16PbIntModeSpec>;
#[doc = "Field `R16_PB_INT_MODE` reader - GPIO PB interrupt mode"]
pub type R16PbIntModeR = crate::FieldReader<u16>;
#[doc = "Field `R16_PB_INT_MODE` writer - GPIO PB interrupt mode"]
pub type R16PbIntModeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO PB interrupt mode"]
    #[inline(always)]
    pub fn r16_pb_int_mode(&self) -> R16PbIntModeR {
        R16PbIntModeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO PB interrupt mode"]
    #[inline(always)]
    pub fn r16_pb_int_mode(&mut self) -> R16PbIntModeW<'_, R16PbIntModeSpec> {
        R16PbIntModeW::new(self, 0)
    }
}
#[doc = "RW, GPIO PB interrupt mode: 0=level action, 1=edge action;RW, status for parallel slave read\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pb_int_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pb_int_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16PbIntModeSpec;
impl crate::RegisterSpec for R16PbIntModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_pb_int_mode::R`](R) reader structure"]
impl crate::Readable for R16PbIntModeSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_pb_int_mode::W`](W) writer structure"]
impl crate::Writable for R16PbIntModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_PB_INT_MODE to value 0"]
impl crate::Resettable for R16PbIntModeSpec {}
