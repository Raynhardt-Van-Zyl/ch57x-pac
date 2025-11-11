#[doc = "Register `R8_UEP2_T_LEN_R8_UH_EP_PID` reader"]
pub type R = crate::R<R8Uep2TLenR8UhEpPidSpec>;
#[doc = "Register `R8_UEP2_T_LEN_R8_UH_EP_PID` writer"]
pub type W = crate::W<R8Uep2TLenR8UhEpPidSpec>;
#[doc = "Field `MASK_UH_ENDP` reader - bit mask of endpoint number for USB host transfer"]
pub type MaskUhEndpR = crate::FieldReader;
#[doc = "Field `MASK_UH_ENDP` writer - bit mask of endpoint number for USB host transfer"]
pub type MaskUhEndpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `R8_UEP2_T_LEN` reader - endpoint 2 transmittal length;"]
pub type R8Uep2TLenR = crate::FieldReader;
#[doc = "Field `R8_UEP2_T_LEN` writer - endpoint 2 transmittal length;"]
pub type R8Uep2TLenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MASK_UH_TOKEN` reader - bit mask of token PID for USB host transfer"]
pub type MaskUhTokenR = crate::FieldReader;
#[doc = "Field `MASK_UH_TOKEN` writer - bit mask of token PID for USB host transfer"]
pub type MaskUhTokenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - bit mask of endpoint number for USB host transfer"]
    #[inline(always)]
    pub fn mask_uh_endp(&self) -> MaskUhEndpR {
        MaskUhEndpR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 0:7 - endpoint 2 transmittal length;"]
    #[inline(always)]
    pub fn r8_uep2_t_len(&self) -> R8Uep2TLenR {
        R8Uep2TLenR::new(self.bits)
    }
    #[doc = "Bits 4:7 - bit mask of token PID for USB host transfer"]
    #[inline(always)]
    pub fn mask_uh_token(&self) -> MaskUhTokenR {
        MaskUhTokenR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - bit mask of endpoint number for USB host transfer"]
    #[inline(always)]
    pub fn mask_uh_endp(&mut self) -> MaskUhEndpW<'_, R8Uep2TLenR8UhEpPidSpec> {
        MaskUhEndpW::new(self, 0)
    }
    #[doc = "Bits 0:7 - endpoint 2 transmittal length;"]
    #[inline(always)]
    pub fn r8_uep2_t_len(&mut self) -> R8Uep2TLenW<'_, R8Uep2TLenR8UhEpPidSpec> {
        R8Uep2TLenW::new(self, 0)
    }
    #[doc = "Bits 4:7 - bit mask of token PID for USB host transfer"]
    #[inline(always)]
    pub fn mask_uh_token(&mut self) -> MaskUhTokenW<'_, R8Uep2TLenR8UhEpPidSpec> {
        MaskUhTokenW::new(self, 4)
    }
}
#[doc = "endpoint 2 transmittal length;host endpoint and PID\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep2_t_len_r8_uh_ep_pid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep2_t_len_r8_uh_ep_pid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep2TLenR8UhEpPidSpec;
impl crate::RegisterSpec for R8Uep2TLenR8UhEpPidSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep2_t_len_r8_uh_ep_pid::R`](R) reader structure"]
impl crate::Readable for R8Uep2TLenR8UhEpPidSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep2_t_len_r8_uh_ep_pid::W`](W) writer structure"]
impl crate::Writable for R8Uep2TLenR8UhEpPidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP2_T_LEN_R8_UH_EP_PID to value 0"]
impl crate::Resettable for R8Uep2TLenR8UhEpPidSpec {}
