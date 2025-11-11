#[doc = "Register `R8_BAT_DET_CFG` reader"]
pub type R = crate::R<R8BatDetCfgSpec>;
#[doc = "Register `R8_BAT_DET_CFG` writer"]
pub type W = crate::W<R8BatDetCfgSpec>;
#[doc = "Field `RB_BAT_LOW_VTH` reader - RWA, select threshold voltage of battery voltage low"]
pub type RbBatLowVthR = crate::FieldReader;
#[doc = "Field `RB_BAT_LOW_VTH` writer - RWA, select threshold voltage of battery voltage low"]
pub type RbBatLowVthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - RWA, select threshold voltage of battery voltage low"]
    #[inline(always)]
    pub fn rb_bat_low_vth(&self) -> RbBatLowVthR {
        RbBatLowVthR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - RWA, select threshold voltage of battery voltage low"]
    #[inline(always)]
    pub fn rb_bat_low_vth(&mut self) -> RbBatLowVthW<'_, R8BatDetCfgSpec> {
        RbBatLowVthW::new(self, 0)
    }
}
#[doc = "RWA, battery voltage detector configuration, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_bat_det_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_bat_det_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8BatDetCfgSpec;
impl crate::RegisterSpec for R8BatDetCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_bat_det_cfg::R`](R) reader structure"]
impl crate::Readable for R8BatDetCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_bat_det_cfg::W`](W) writer structure"]
impl crate::Writable for R8BatDetCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_BAT_DET_CFG to value 0x01"]
impl crate::Resettable for R8BatDetCfgSpec {
    const RESET_VALUE: u8 = 0x01;
}
