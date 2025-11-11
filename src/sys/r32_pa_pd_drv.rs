#[doc = "Register `R32_PA_PD_DRV` reader"]
pub type R = crate::R<R32PaPdDrvSpec>;
#[doc = "Register `R32_PA_PD_DRV` writer"]
pub type W = crate::W<R32PaPdDrvSpec>;
#[doc = "Field `R8_PA_PD_DRV_0` reader - PA pulldown for input or PA driving capability for output byte 0"]
pub type R8PaPdDrv0R = crate::FieldReader;
#[doc = "Field `R8_PA_PD_DRV_0` writer - PA pulldown for input or PA driving capability for output byte 0"]
pub type R8PaPdDrv0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PA_PD_DRV_1` reader - PA pulldown for input or PA driving capability for output byte 1"]
pub type R8PaPdDrv1R = crate::FieldReader;
#[doc = "Field `R8_PA_PD_DRV_1` writer - PA pulldown for input or PA driving capability for output byte 1"]
pub type R8PaPdDrv1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PA pulldown for input or PA driving capability for output byte 0"]
    #[inline(always)]
    pub fn r8_pa_pd_drv_0(&self) -> R8PaPdDrv0R {
        R8PaPdDrv0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PA pulldown for input or PA driving capability for output byte 1"]
    #[inline(always)]
    pub fn r8_pa_pd_drv_1(&self) -> R8PaPdDrv1R {
        R8PaPdDrv1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PA pulldown for input or PA driving capability for output byte 0"]
    #[inline(always)]
    pub fn r8_pa_pd_drv_0(&mut self) -> R8PaPdDrv0W<'_, R32PaPdDrvSpec> {
        R8PaPdDrv0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - PA pulldown for input or PA driving capability for output byte 1"]
    #[inline(always)]
    pub fn r8_pa_pd_drv_1(&mut self) -> R8PaPdDrv1W<'_, R32PaPdDrvSpec> {
        R8PaPdDrv1W::new(self, 8)
    }
}
#[doc = "RW, PA pulldown for input or PA driving capability for output\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pd_drv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_pd_drv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaPdDrvSpec;
impl crate::RegisterSpec for R32PaPdDrvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_pd_drv::R`](R) reader structure"]
impl crate::Readable for R32PaPdDrvSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pa_pd_drv::W`](W) writer structure"]
impl crate::Writable for R32PaPdDrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_PD_DRV to value 0"]
impl crate::Resettable for R32PaPdDrvSpec {}
