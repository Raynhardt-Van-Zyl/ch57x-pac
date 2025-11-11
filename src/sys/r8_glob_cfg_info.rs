#[doc = "Register `R8_GLOB_CFG_INFO` reader"]
pub type R = crate::R<R8GlobCfgInfoSpec>;
#[doc = "Field `RB_CFG_ROM_READ` reader - RO, indicate protected status of Flash ROM code and data: 0=reading protect, 1=enable read by external programmer"]
pub type RbCfgRomReadR = crate::BitReader;
#[doc = "Field `RB_CFG_RESET_EN` reader - RO, manual reset input enable status"]
pub type RbCfgResetEnR = crate::BitReader;
#[doc = "Field `RB_CFG_BOOT_EN` reader - RO, boot-loader enable status"]
pub type RbCfgBootEnR = crate::BitReader;
#[doc = "Field `RB_CFG_DEBUG_EN` reader - RO, debug enable status"]
pub type RbCfgDebugEnR = crate::BitReader;
#[doc = "Field `RB_BOOT_LOADER` reader - RO, indicate boot loader status: 0=application status (by software reset), 1=boot loader status"]
pub type RbBootLoaderR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RO, indicate protected status of Flash ROM code and data: 0=reading protect, 1=enable read by external programmer"]
    #[inline(always)]
    pub fn rb_cfg_rom_read(&self) -> RbCfgRomReadR {
        RbCfgRomReadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - RO, manual reset input enable status"]
    #[inline(always)]
    pub fn rb_cfg_reset_en(&self) -> RbCfgResetEnR {
        RbCfgResetEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RO, boot-loader enable status"]
    #[inline(always)]
    pub fn rb_cfg_boot_en(&self) -> RbCfgBootEnR {
        RbCfgBootEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RO, debug enable status"]
    #[inline(always)]
    pub fn rb_cfg_debug_en(&self) -> RbCfgDebugEnR {
        RbCfgDebugEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RO, indicate boot loader status: 0=application status (by software reset), 1=boot loader status"]
    #[inline(always)]
    pub fn rb_boot_loader(&self) -> RbBootLoaderR {
        RbBootLoaderR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "RO, global configuration information and status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_glob_cfg_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8GlobCfgInfoSpec;
impl crate::RegisterSpec for R8GlobCfgInfoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_glob_cfg_info::R`](R) reader structure"]
impl crate::Readable for R8GlobCfgInfoSpec {}
#[doc = "`reset()` method sets R8_GLOB_CFG_INFO to value 0xe8"]
impl crate::Resettable for R8GlobCfgInfoSpec {
    const RESET_VALUE: u8 = 0xe8;
}
