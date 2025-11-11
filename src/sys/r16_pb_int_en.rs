#[doc = "Register `R16_PB_INT_EN` reader"]
pub type R = crate::R<R16PbIntEnSpec>;
#[doc = "Register `R16_PB_INT_EN` writer"]
pub type W = crate::W<R16PbIntEnSpec>;
#[doc = "Field `R16_PB_INT_EN` reader - GPIO PB interrupt enable"]
pub type R16PbIntEnR = crate::FieldReader<u16>;
#[doc = "Field `R16_PB_INT_EN` writer - GPIO PB interrupt enable"]
pub type R16PbIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO PB interrupt enable"]
    #[inline(always)]
    pub fn r16_pb_int_en(&self) -> R16PbIntEnR {
        R16PbIntEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO PB interrupt enable"]
    #[inline(always)]
    pub fn r16_pb_int_en(&mut self) -> R16PbIntEnW<'_, R16PbIntEnSpec> {
        R16PbIntEnW::new(self, 0)
    }
}
#[doc = "RW, GPIO PB interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pb_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pb_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16PbIntEnSpec;
impl crate::RegisterSpec for R16PbIntEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_pb_int_en::R`](R) reader structure"]
impl crate::Readable for R16PbIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_pb_int_en::W`](W) writer structure"]
impl crate::Writable for R16PbIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_PB_INT_EN to value 0"]
impl crate::Resettable for R16PbIntEnSpec {}
