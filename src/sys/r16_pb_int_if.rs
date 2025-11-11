#[doc = "Register `R16_PB_INT_IF` reader"]
pub type R = crate::R<R16PbIntIfSpec>;
#[doc = "Register `R16_PB_INT_IF` writer"]
pub type W = crate::W<R16PbIntIfSpec>;
#[doc = "Field `R16_PB_INT_IF` reader - GPIO PB interrupt flag"]
pub type R16PbIntIfR = crate::FieldReader<u16>;
#[doc = "Field `R16_PB_INT_IF` writer - GPIO PB interrupt flag"]
pub type R16PbIntIfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO PB interrupt flag"]
    #[inline(always)]
    pub fn r16_pb_int_if(&self) -> R16PbIntIfR {
        R16PbIntIfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO PB interrupt flag"]
    #[inline(always)]
    pub fn r16_pb_int_if(&mut self) -> R16PbIntIfW<'_, R16PbIntIfSpec> {
        R16PbIntIfW::new(self, 0)
    }
}
#[doc = "RW1, GPIO PB interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pb_int_if::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pb_int_if::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16PbIntIfSpec;
impl crate::RegisterSpec for R16PbIntIfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_pb_int_if::R`](R) reader structure"]
impl crate::Readable for R16PbIntIfSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_pb_int_if::W`](W) writer structure"]
impl crate::Writable for R16PbIntIfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_PB_INT_IF to value 0"]
impl crate::Resettable for R16PbIntIfSpec {}
