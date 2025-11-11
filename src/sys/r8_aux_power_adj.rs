#[doc = "Register `R8_AUX_POWER_ADJ` reader"]
pub type R = crate::R<R8AuxPowerAdjSpec>;
#[doc = "Register `R8_AUX_POWER_ADJ` writer"]
pub type W = crate::W<R8AuxPowerAdjSpec>;
#[doc = "Field `RB_ULPLDO_ADJ` reader - RWA, Ultra-Low-Power LDO voltage adjust"]
pub type RbUlpldoAdjR = crate::FieldReader;
#[doc = "Field `RB_ULPLDO_ADJ` writer - RWA, Ultra-Low-Power LDO voltage adjust"]
pub type RbUlpldoAdjW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - RWA, Ultra-Low-Power LDO voltage adjust"]
    #[inline(always)]
    pub fn rb_ulpldo_adj(&self) -> RbUlpldoAdjR {
        RbUlpldoAdjR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - RWA, Ultra-Low-Power LDO voltage adjust"]
    #[inline(always)]
    pub fn rb_ulpldo_adj(&mut self) -> RbUlpldoAdjW<'_, R8AuxPowerAdjSpec> {
        RbUlpldoAdjW::new(self, 0)
    }
}
#[doc = "RWA, aux power adjust control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_aux_power_adj::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_aux_power_adj::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8AuxPowerAdjSpec;
impl crate::RegisterSpec for R8AuxPowerAdjSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_aux_power_adj::R`](R) reader structure"]
impl crate::Readable for R8AuxPowerAdjSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_aux_power_adj::W`](W) writer structure"]
impl crate::Writable for R8AuxPowerAdjSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_AUX_POWER_ADJ to value 0"]
impl crate::Resettable for R8AuxPowerAdjSpec {}
