#[doc = "Register `R8_UEP3_CTRL__R8_UH_TX_CTRL` reader"]
pub type R = crate::R<R8Uep3Ctrl_R8UhTxCtrlSpec>;
#[doc = "Register `R8_UEP3_CTRL__R8_UH_TX_CTRL` writer"]
pub type W = crate::W<R8Uep3Ctrl_R8UhTxCtrlSpec>;
#[doc = "Field `RB_UH_T_RES` reader - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type RbUhTResR = crate::FieldReader;
#[doc = "Field `RB_UH_T_RES` writer - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type RbUhTResW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MASK_UEP_T_RES` reader - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type MaskUepTResR = crate::FieldReader;
#[doc = "Field `MASK_UEP_T_RES` writer - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type MaskUepTResW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MASK_UEP_R_RES` reader - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
pub type MaskUepRResR = crate::FieldReader;
#[doc = "Field `MASK_UEP_R_RES` writer - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
pub type MaskUepRResW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_UEP_AUTO_TOG_RB_UH_T_AUTO_TOG` reader - enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle"]
pub type RbUepAutoTogRbUhTAutoTogR = crate::BitReader;
#[doc = "Field `RB_UEP_AUTO_TOG_RB_UH_T_AUTO_TOG` writer - enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle"]
pub type RbUepAutoTogRbUhTAutoTogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP_T_TOG_RB_UH_T_TOG` reader - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
pub type RbUepTTogRbUhTTogR = crate::BitReader;
#[doc = "Field `RB_UEP_T_TOG_RB_UH_T_TOG` writer - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
pub type RbUepTTogRbUhTTogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP_R_TOG` reader - expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1"]
pub type RbUepRTogR = crate::BitReader;
#[doc = "Field `RB_UEP_R_TOG` writer - expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1"]
pub type RbUepRTogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uh_t_res(&self) -> RbUhTResR {
        RbUhTResR::new(self.bits & 3)
    }
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
    pub fn rb_uep_auto_tog_rb_uh_t_auto_tog(&self) -> RbUepAutoTogRbUhTAutoTogR {
        RbUepAutoTogRbUhTAutoTogR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    pub fn rb_uep_t_tog_rb_uh_t_tog(&self) -> RbUepTTogRbUhTTogR {
        RbUepTTogRbUhTTogR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    pub fn rb_uep_r_tog(&self) -> RbUepRTogR {
        RbUepRTogR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uh_t_res(&mut self) -> RbUhTResW<'_, R8Uep3Ctrl_R8UhTxCtrlSpec> {
        RbUhTResW::new(self, 0)
    }
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    pub fn mask_uep_t_res(&mut self) -> MaskUepTResW<'_, R8Uep3Ctrl_R8UhTxCtrlSpec> {
        MaskUepTResW::new(self, 0)
    }
    #[doc = "Bits 2:3 - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
    #[inline(always)]
    pub fn mask_uep_r_res(&mut self) -> MaskUepRResW<'_, R8Uep3Ctrl_R8UhTxCtrlSpec> {
        MaskUepRResW::new(self, 2)
    }
    #[doc = "Bit 4 - enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle"]
    #[inline(always)]
    pub fn rb_uep_auto_tog_rb_uh_t_auto_tog(
        &mut self,
    ) -> RbUepAutoTogRbUhTAutoTogW<'_, R8Uep3Ctrl_R8UhTxCtrlSpec> {
        RbUepAutoTogRbUhTAutoTogW::new(self, 4)
    }
    #[doc = "Bit 6 - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    pub fn rb_uep_t_tog_rb_uh_t_tog(
        &mut self,
    ) -> RbUepTTogRbUhTTogW<'_, R8Uep3Ctrl_R8UhTxCtrlSpec> {
        RbUepTTogRbUhTTogW::new(self, 6)
    }
    #[doc = "Bit 7 - expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    pub fn rb_uep_r_tog(&mut self) -> RbUepRTogW<'_, R8Uep3Ctrl_R8UhTxCtrlSpec> {
        RbUepRTogW::new(self, 7)
    }
}
#[doc = "endpoint 3 control;host transmittal endpoint control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep3_ctrl__r8_uh_tx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep3_ctrl__r8_uh_tx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep3Ctrl_R8UhTxCtrlSpec;
impl crate::RegisterSpec for R8Uep3Ctrl_R8UhTxCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep3_ctrl__r8_uh_tx_ctrl::R`](R) reader structure"]
impl crate::Readable for R8Uep3Ctrl_R8UhTxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep3_ctrl__r8_uh_tx_ctrl::W`](W) writer structure"]
impl crate::Writable for R8Uep3Ctrl_R8UhTxCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP3_CTRL__R8_UH_TX_CTRL to value 0"]
impl crate::Resettable for R8Uep3Ctrl_R8UhTxCtrlSpec {}
