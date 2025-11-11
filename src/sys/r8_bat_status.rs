#[doc = "Register `R8_BAT_STATUS` reader"]
pub type R = crate::R<R8BatStatusSpec>;
#[doc = "Field `RB_BAT_STAT_LOWER` reader - RO, battery lower voltage status, high action"]
pub type RbBatStatLowerR = crate::BitReader;
#[doc = "Field `RB_BAT_STAT_LOW` reader - RO, battery low voltage status, high action"]
pub type RbBatStatLowR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RO, battery lower voltage status, high action"]
    #[inline(always)]
    pub fn rb_bat_stat_lower(&self) -> RbBatStatLowerR {
        RbBatStatLowerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RO, battery low voltage status, high action"]
    #[inline(always)]
    pub fn rb_bat_stat_low(&self) -> RbBatStatLowR {
        RbBatStatLowR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "RO, battery status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_bat_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8BatStatusSpec;
impl crate::RegisterSpec for R8BatStatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_bat_status::R`](R) reader structure"]
impl crate::Readable for R8BatStatusSpec {}
#[doc = "`reset()` method sets R8_BAT_STATUS to value 0"]
impl crate::Resettable for R8BatStatusSpec {}
