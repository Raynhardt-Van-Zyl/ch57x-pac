#[doc = "Register `R32_TMR3_CNT_END` reader"]
pub type R = crate::R<R32Tmr3CntEndSpec>;
#[doc = "Register `R32_TMR3_CNT_END` writer"]
pub type W = crate::W<R32Tmr3CntEndSpec>;
#[doc = "Field `R32_TMR3_CNT_END` reader - RW, TMR3 end count value, only low 26 bit"]
pub type R32Tmr3CntEndR = crate::FieldReader<u32>;
#[doc = "Field `R32_TMR3_CNT_END` writer - RW, TMR3 end count value, only low 26 bit"]
pub type R32Tmr3CntEndW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW, TMR3 end count value, only low 26 bit"]
    #[inline(always)]
    pub fn r32_tmr3_cnt_end(&self) -> R32Tmr3CntEndR {
        R32Tmr3CntEndR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW, TMR3 end count value, only low 26 bit"]
    #[inline(always)]
    pub fn r32_tmr3_cnt_end(&mut self) -> R32Tmr3CntEndW<'_, R32Tmr3CntEndSpec> {
        R32Tmr3CntEndW::new(self, 0)
    }
}
#[doc = "RW, TMR3 end count value, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr3_cnt_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr3_cnt_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr3CntEndSpec;
impl crate::RegisterSpec for R32Tmr3CntEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr3_cnt_end::R`](R) reader structure"]
impl crate::Readable for R32Tmr3CntEndSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_tmr3_cnt_end::W`](W) writer structure"]
impl crate::Writable for R32Tmr3CntEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_TMR3_CNT_END to value 0"]
impl crate::Resettable for R32Tmr3CntEndSpec {}
