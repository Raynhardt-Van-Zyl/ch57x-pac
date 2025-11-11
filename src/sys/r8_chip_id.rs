#[doc = "Register `R8_CHIP_ID` reader"]
pub type R = crate::R<R8ChipIdSpec>;
#[doc = "Field `R8_CHIP_ID` reader - RF,chip ID register"]
pub type R8ChipIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RF,chip ID register"]
    #[inline(always)]
    pub fn r8_chip_id(&self) -> R8ChipIdR {
        R8ChipIdR::new(self.bits)
    }
}
#[doc = "RF, chip ID register, always is ID_CH57*\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_chip_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8ChipIdSpec;
impl crate::RegisterSpec for R8ChipIdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_chip_id::R`](R) reader structure"]
impl crate::Readable for R8ChipIdSpec {}
#[doc = "`reset()` method sets R8_CHIP_ID to value 0x73"]
impl crate::Resettable for R8ChipIdSpec {
    const RESET_VALUE: u8 = 0x73;
}
