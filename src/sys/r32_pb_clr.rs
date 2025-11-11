#[doc = "Register `R32_PB_CLR` reader"]
pub type R = crate::R<R32PbClrSpec>;
#[doc = "Register `R32_PB_CLR` writer"]
pub type W = crate::W<R32PbClrSpec>;
#[doc = "Field `R8_PB_CLR_0` reader - GPIO PB clear output byte 0"]
pub type R8PbClr0R = crate::FieldReader;
#[doc = "Field `R8_PB_CLR_0` writer - GPIO PB clear output byte 0"]
pub type R8PbClr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PB_CLR_1` reader - GPIO PB clear output byte 1"]
pub type R8PbClr1R = crate::FieldReader;
#[doc = "Field `R8_PB_CLR_1` writer - GPIO PB clear output byte 1"]
pub type R8PbClr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PB_CLR_2` reader - GPIO PB clear output byte 2"]
pub type R8PbClr2R = crate::FieldReader;
#[doc = "Field `R8_PB_CLR_2` writer - GPIO PB clear output byte 2"]
pub type R8PbClr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO PB clear output byte 0"]
    #[inline(always)]
    pub fn r8_pb_clr_0(&self) -> R8PbClr0R {
        R8PbClr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO PB clear output byte 1"]
    #[inline(always)]
    pub fn r8_pb_clr_1(&self) -> R8PbClr1R {
        R8PbClr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO PB clear output byte 2"]
    #[inline(always)]
    pub fn r8_pb_clr_2(&self) -> R8PbClr2R {
        R8PbClr2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO PB clear output byte 0"]
    #[inline(always)]
    pub fn r8_pb_clr_0(&mut self) -> R8PbClr0W<'_, R32PbClrSpec> {
        R8PbClr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO PB clear output byte 1"]
    #[inline(always)]
    pub fn r8_pb_clr_1(&mut self) -> R8PbClr1W<'_, R32PbClrSpec> {
        R8PbClr1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO PB clear output byte 2"]
    #[inline(always)]
    pub fn r8_pb_clr_2(&mut self) -> R8PbClr2W<'_, R32PbClrSpec> {
        R8PbClr2W::new(self, 16)
    }
}
#[doc = "WZ, GPIO PB clear output: 0=keep, 1=clear\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbClrSpec;
impl crate::RegisterSpec for R32PbClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_clr::R`](R) reader structure"]
impl crate::Readable for R32PbClrSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pb_clr::W`](W) writer structure"]
impl crate::Writable for R32PbClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PB_CLR to value 0"]
impl crate::Resettable for R32PbClrSpec {}
