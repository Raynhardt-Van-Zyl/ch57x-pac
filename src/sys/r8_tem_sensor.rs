#[doc = "Register `R8_TEM_SENSOR` reader"]
pub type R = crate::R<R8TemSensorSpec>;
#[doc = "Register `R8_TEM_SENSOR` writer"]
pub type W = crate::W<R8TemSensorSpec>;
#[doc = "Field `RB_TEM_SEN_PWR_ON` reader - RW, temperature sensor power control: 0=power down, 1=power on"]
pub type RbTemSenPwrOnR = crate::BitReader;
#[doc = "Field `RB_TEM_SEN_PWR_ON` writer - RW, temperature sensor power control: 0=power down, 1=power on"]
pub type RbTemSenPwrOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - RW, temperature sensor power control: 0=power down, 1=power on"]
    #[inline(always)]
    pub fn rb_tem_sen_pwr_on(&self) -> RbTemSenPwrOnR {
        RbTemSenPwrOnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - RW, temperature sensor power control: 0=power down, 1=power on"]
    #[inline(always)]
    pub fn rb_tem_sen_pwr_on(&mut self) -> RbTemSenPwrOnW<'_, R8TemSensorSpec> {
        RbTemSenPwrOnW::new(self, 7)
    }
}
#[doc = "RW, temperature sensor control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tem_sensor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tem_sensor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8TemSensorSpec;
impl crate::RegisterSpec for R8TemSensorSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_tem_sensor::R`](R) reader structure"]
impl crate::Readable for R8TemSensorSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_tem_sensor::W`](W) writer structure"]
impl crate::Writable for R8TemSensorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_TEM_SENSOR to value 0"]
impl crate::Resettable for R8TemSensorSpec {}
