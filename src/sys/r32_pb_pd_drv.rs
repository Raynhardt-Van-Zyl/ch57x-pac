#[doc = "Register `R32_PB_PD_DRV` reader"]
pub type R = crate::R<R32PbPdDrvSpec>;
#[doc = "Register `R32_PB_PD_DRV` writer"]
pub type W = crate::W<R32PbPdDrvSpec>;
#[doc = "Field `R8_PB_PD_DRV_0` reader - PB pulldown for input or PB driving capability for output byte 0"]
pub type R8PbPdDrv0R = crate::FieldReader;
#[doc = "Field `R8_PB_PD_DRV_0` writer - PB pulldown for input or PB driving capability for output byte 0"]
pub type R8PbPdDrv0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PB_PD_DRV_1` reader - PB pulldown for input or PB driving capability for output byte 0"]
pub type R8PbPdDrv1R = crate::FieldReader;
#[doc = "Field `R8_PB_PD_DRV_1` writer - PB pulldown for input or PB driving capability for output byte 0"]
pub type R8PbPdDrv1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PB_PD_DRV_2` reader - PB pulldown for input or PB driving capability for output byte 0"]
pub type R8PbPdDrv2R = crate::FieldReader;
#[doc = "Field `R8_PB_PD_DRV_2` writer - PB pulldown for input or PB driving capability for output byte 0"]
pub type R8PbPdDrv2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PB pulldown for input or PB driving capability for output byte 0"]
    #[inline(always)]
    pub fn r8_pb_pd_drv_0(&self) -> R8PbPdDrv0R {
        R8PbPdDrv0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PB pulldown for input or PB driving capability for output byte 0"]
    #[inline(always)]
    pub fn r8_pb_pd_drv_1(&self) -> R8PbPdDrv1R {
        R8PbPdDrv1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PB pulldown for input or PB driving capability for output byte 0"]
    #[inline(always)]
    pub fn r8_pb_pd_drv_2(&self) -> R8PbPdDrv2R {
        R8PbPdDrv2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PB pulldown for input or PB driving capability for output byte 0"]
    #[inline(always)]
    pub fn r8_pb_pd_drv_0(&mut self) -> R8PbPdDrv0W<'_, R32PbPdDrvSpec> {
        R8PbPdDrv0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - PB pulldown for input or PB driving capability for output byte 0"]
    #[inline(always)]
    pub fn r8_pb_pd_drv_1(&mut self) -> R8PbPdDrv1W<'_, R32PbPdDrvSpec> {
        R8PbPdDrv1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - PB pulldown for input or PB driving capability for output byte 0"]
    #[inline(always)]
    pub fn r8_pb_pd_drv_2(&mut self) -> R8PbPdDrv2W<'_, R32PbPdDrvSpec> {
        R8PbPdDrv2W::new(self, 16)
    }
}
#[doc = "RW, PB pulldown for input or PB driving capability for output\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pd_drv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_pd_drv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbPdDrvSpec;
impl crate::RegisterSpec for R32PbPdDrvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_pd_drv::R`](R) reader structure"]
impl crate::Readable for R32PbPdDrvSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pb_pd_drv::W`](W) writer structure"]
impl crate::Writable for R32PbPdDrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PB_PD_DRV to value 0"]
impl crate::Resettable for R32PbPdDrvSpec {}
