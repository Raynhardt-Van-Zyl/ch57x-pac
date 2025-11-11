#[doc = "Register `R8_RESET_STATUS__R8_GLOB_ROM_CFG` reader"]
pub type R = crate::R<R8ResetStatus_R8GlobRomCfgSpec>;
#[doc = "Field `RB_RESET_FLAG` reader - RO, recent reset flag"]
pub type RbResetFlagR = crate::FieldReader;
#[doc = "Field `RB_ROM_CODE_OFS` reader - RWA, code offset address selection in Flash ROM: 0=start address 0x000000, 1=start address 0x008000"]
pub type RbRomCodeOfsR = crate::BitReader;
#[doc = "Field `RB_ROM_CTRL_EN` reader - RWA, enable flash ROM control interface enable"]
pub type RbRomCtrlEnR = crate::BitReader;
#[doc = "Field `RB_ROM_DATA_WE` reader - RWA,enable flash ROM data and code area being erase/write"]
pub type RbRomDataWeR = crate::BitReader;
#[doc = "Field `RB_ROM_CODE_WE` reader - RWA, enable flash ROM code area being erase or write"]
pub type RbRomCodeWeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - RO, recent reset flag"]
    #[inline(always)]
    pub fn rb_reset_flag(&self) -> RbResetFlagR {
        RbResetFlagR::new(self.bits & 7)
    }
    #[doc = "Bit 4 - RWA, code offset address selection in Flash ROM: 0=start address 0x000000, 1=start address 0x008000"]
    #[inline(always)]
    pub fn rb_rom_code_ofs(&self) -> RbRomCodeOfsR {
        RbRomCodeOfsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RWA, enable flash ROM control interface enable"]
    #[inline(always)]
    pub fn rb_rom_ctrl_en(&self) -> RbRomCtrlEnR {
        RbRomCtrlEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RWA,enable flash ROM data and code area being erase/write"]
    #[inline(always)]
    pub fn rb_rom_data_we(&self) -> RbRomDataWeR {
        RbRomDataWeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RWA, enable flash ROM code area being erase or write"]
    #[inline(always)]
    pub fn rb_rom_code_we(&self) -> RbRomCodeWeR {
        RbRomCodeWeR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "RWA, reset status, SAM or flash ROM configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_reset_status__r8_glob_rom_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8ResetStatus_R8GlobRomCfgSpec;
impl crate::RegisterSpec for R8ResetStatus_R8GlobRomCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_reset_status__r8_glob_rom_cfg::R`](R) reader structure"]
impl crate::Readable for R8ResetStatus_R8GlobRomCfgSpec {}
#[doc = "`reset()` method sets R8_RESET_STATUS__R8_GLOB_ROM_CFG to value 0x01"]
impl crate::Resettable for R8ResetStatus_R8GlobRomCfgSpec {
    const RESET_VALUE: u8 = 0x01;
}
