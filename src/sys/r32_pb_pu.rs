#[doc = "Register `R32_PB_PU` reader"]
pub type R = crate::R<R32PbPuSpec>;
#[doc = "Register `R32_PB_PU` writer"]
pub type W = crate::W<R32PbPuSpec>;
#[doc = "Field `R8_PB_PU_0` reader - GPIO PB pullup resistance enable byte 0"]
pub type R8PbPu0R = crate::FieldReader;
#[doc = "Field `R8_PB_PU_0` writer - GPIO PB pullup resistance enable byte 0"]
pub type R8PbPu0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PB_PU_1` reader - GPIO PB pullup resistance enable byte 1"]
pub type R8PbPu1R = crate::FieldReader;
#[doc = "Field `R8_PB_PU_1` writer - GPIO PB pullup resistance enable byte 1"]
pub type R8PbPu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PB_PU_2` reader - GPIO PB pullup resistance enable byte 2"]
pub type R8PbPu2R = crate::FieldReader;
#[doc = "Field `R8_PB_PU_2` writer - GPIO PB pullup resistance enable byte 2"]
pub type R8PbPu2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO PB pullup resistance enable byte 0"]
    #[inline(always)]
    pub fn r8_pb_pu_0(&self) -> R8PbPu0R {
        R8PbPu0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO PB pullup resistance enable byte 1"]
    #[inline(always)]
    pub fn r8_pb_pu_1(&self) -> R8PbPu1R {
        R8PbPu1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO PB pullup resistance enable byte 2"]
    #[inline(always)]
    pub fn r8_pb_pu_2(&self) -> R8PbPu2R {
        R8PbPu2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO PB pullup resistance enable byte 0"]
    #[inline(always)]
    pub fn r8_pb_pu_0(&mut self) -> R8PbPu0W<'_, R32PbPuSpec> {
        R8PbPu0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO PB pullup resistance enable byte 1"]
    #[inline(always)]
    pub fn r8_pb_pu_1(&mut self) -> R8PbPu1W<'_, R32PbPuSpec> {
        R8PbPu1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO PB pullup resistance enable byte 2"]
    #[inline(always)]
    pub fn r8_pb_pu_2(&mut self) -> R8PbPu2W<'_, R32PbPuSpec> {
        R8PbPu2W::new(self, 16)
    }
}
#[doc = "RW, GPIO PB pullup resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_pu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbPuSpec;
impl crate::RegisterSpec for R32PbPuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_pu::R`](R) reader structure"]
impl crate::Readable for R32PbPuSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pb_pu::W`](W) writer structure"]
impl crate::Writable for R32PbPuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PB_PU to value 0"]
impl crate::Resettable for R32PbPuSpec {}
