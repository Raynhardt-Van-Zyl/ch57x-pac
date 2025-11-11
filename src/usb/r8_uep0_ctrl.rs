#[doc = "Register `R8_UEP0_CTRL` reader"]
pub type R = crate::R<R8Uep0CtrlSpec>;
#[doc = "Register `R8_UEP0_CTRL` writer"]
pub type W = crate::W<R8Uep0CtrlSpec>;
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
#[doc = "Field `RB_UEP_T_TOG` reader - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
pub type RbUepTTogR = crate::BitReader;
#[doc = "Field `RB_UEP_T_TOG` writer - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
pub type RbUepTTogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP_R_TOG` reader - expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1"]
pub type RbUepRTogR = crate::BitReader;
#[doc = "Field `RB_UEP_R_TOG` writer - expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1"]
pub type RbUepRTogW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 6 - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    pub fn rb_uep_t_tog(&self) -> RbUepTTogR {
        RbUepTTogR::new(((self.bits >> 6) & 1) != 0)
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
    pub fn mask_uep_t_res(&mut self) -> MaskUepTResW<'_, R8Uep0CtrlSpec> {
        MaskUepTResW::new(self, 0)
    }
    #[doc = "Bits 2:3 - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
    #[inline(always)]
    pub fn mask_uep_r_res(&mut self) -> MaskUepRResW<'_, R8Uep0CtrlSpec> {
        MaskUepRResW::new(self, 2)
    }
    #[doc = "Bit 4 - enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle"]
    #[inline(always)]
    pub fn rb_uep_auto_tog(&mut self) -> RbUepAutoTogW<'_, R8Uep0CtrlSpec> {
        RbUepAutoTogW::new(self, 4)
    }
    #[doc = "Bit 6 - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    pub fn rb_uep_t_tog(&mut self) -> RbUepTTogW<'_, R8Uep0CtrlSpec> {
        RbUepTTogW::new(self, 6)
    }
    #[doc = "Bit 7 - expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    pub fn rb_uep_r_tog(&mut self) -> RbUepRTogW<'_, R8Uep0CtrlSpec> {
        RbUepRTogW::new(self, 7)
    }
}
#[doc = "endpoint 0 control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep0CtrlSpec;
impl crate::RegisterSpec for R8Uep0CtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep0_ctrl::R`](R) reader structure"]
impl crate::Readable for R8Uep0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep0_ctrl::W`](W) writer structure"]
impl crate::Writable for R8Uep0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP0_CTRL to value 0"]
impl crate::Resettable for R8Uep0CtrlSpec {}
