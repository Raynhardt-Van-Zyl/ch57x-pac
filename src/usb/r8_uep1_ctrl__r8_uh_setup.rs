#[doc = "Register `R8_UEP1_CTRL__R8_UH_SETUP` reader"]
pub type R = crate::R<R8Uep1Ctrl_R8UhSetupSpec>;
#[doc = "Register `R8_UEP1_CTRL__R8_UH_SETUP` writer"]
pub type W = crate::W<R8Uep1Ctrl_R8UhSetupSpec>;
#[doc = "Field `MASK_UEP_T_RES` reader - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type MaskUepTResR = crate::FieldReader;
#[doc = "Field `MASK_UEP_T_RES` writer - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type MaskUepTResW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MASK_UEP_R_RES` reader - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
pub type MaskUepRResR = crate::FieldReader;
#[doc = "Field `MASK_UEP_R_RES` writer - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
pub type MaskUepRResW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_UEP_AUTO_TOG` reader - enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle"]
pub type RbUepAutoTogR = crate::BitReader;
#[doc = "Field `RB_UEP_AUTO_TOG` writer - enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle"]
pub type RbUepAutoTogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP_T_TOG__RB_UH_SOF_EN` reader - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1;USB host automatic SOF enable"]
pub type RbUepTTog_RbUhSofEnR = crate::BitReader;
#[doc = "Field `RB_UEP_T_TOG__RB_UH_SOF_EN` writer - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1;USB host automatic SOF enable"]
pub type RbUepTTog_RbUhSofEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP_R_TOG__RB_UH_PRE_PID_EN` reader - expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1;RB_UH_PRE_PID_EN;USB host PRE PID enable for low speed device via hub"]
pub type RbUepRTog_RbUhPrePidEnR = crate::BitReader;
#[doc = "Field `RB_UEP_R_TOG__RB_UH_PRE_PID_EN` writer - expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1;RB_UH_PRE_PID_EN;USB host PRE PID enable for low speed device via hub"]
pub type RbUepRTog_RbUhPrePidEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    pub fn mask_uep_t_res(&self) -> MaskUepTResR {
        MaskUepTResR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
    #[inline(always)]
    pub fn mask_uep_r_res(&self) -> MaskUepRResR {
        MaskUepRResR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle"]
    #[inline(always)]
    pub fn rb_uep_auto_tog(&self) -> RbUepAutoTogR {
        RbUepAutoTogR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1;USB host automatic SOF enable"]
    #[inline(always)]
    pub fn rb_uep_t_tog__rb_uh_sof_en(&self) -> RbUepTTog_RbUhSofEnR {
        RbUepTTog_RbUhSofEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1;RB_UH_PRE_PID_EN;USB host PRE PID enable for low speed device via hub"]
    #[inline(always)]
    pub fn rb_uep_r_tog__rb_uh_pre_pid_en(&self) -> RbUepRTog_RbUhPrePidEnR {
        RbUepRTog_RbUhPrePidEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    pub fn mask_uep_t_res(&mut self) -> MaskUepTResW<'_, R8Uep1Ctrl_R8UhSetupSpec> {
        MaskUepTResW::new(self, 0)
    }
    #[doc = "Bits 2:3 - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
    #[inline(always)]
    pub fn mask_uep_r_res(&mut self) -> MaskUepRResW<'_, R8Uep1Ctrl_R8UhSetupSpec> {
        MaskUepRResW::new(self, 2)
    }
    #[doc = "Bit 4 - enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle"]
    #[inline(always)]
    pub fn rb_uep_auto_tog(&mut self) -> RbUepAutoTogW<'_, R8Uep1Ctrl_R8UhSetupSpec> {
        RbUepAutoTogW::new(self, 4)
    }
    #[doc = "Bit 6 - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1;USB host automatic SOF enable"]
    #[inline(always)]
    pub fn rb_uep_t_tog__rb_uh_sof_en(
        &mut self,
    ) -> RbUepTTog_RbUhSofEnW<'_, R8Uep1Ctrl_R8UhSetupSpec> {
        RbUepTTog_RbUhSofEnW::new(self, 6)
    }
    #[doc = "Bit 7 - expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1;RB_UH_PRE_PID_EN;USB host PRE PID enable for low speed device via hub"]
    #[inline(always)]
    pub fn rb_uep_r_tog__rb_uh_pre_pid_en(
        &mut self,
    ) -> RbUepRTog_RbUhPrePidEnW<'_, R8Uep1Ctrl_R8UhSetupSpec> {
        RbUepRTog_RbUhPrePidEnW::new(self, 7)
    }
}
#[doc = "endpoint 1 control;host aux setup\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep1_ctrl__r8_uh_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep1_ctrl__r8_uh_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep1Ctrl_R8UhSetupSpec;
impl crate::RegisterSpec for R8Uep1Ctrl_R8UhSetupSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep1_ctrl__r8_uh_setup::R`](R) reader structure"]
impl crate::Readable for R8Uep1Ctrl_R8UhSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep1_ctrl__r8_uh_setup::W`](W) writer structure"]
impl crate::Writable for R8Uep1Ctrl_R8UhSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP1_CTRL__R8_UH_SETUP to value 0"]
impl crate::Resettable for R8Uep1Ctrl_R8UhSetupSpec {}
