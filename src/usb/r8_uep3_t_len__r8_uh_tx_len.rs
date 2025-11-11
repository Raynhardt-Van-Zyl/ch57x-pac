#[doc = "Register `R8_UEP3_T_LEN__R8_UH_TX_LEN` reader"]
pub type R = crate::R<R8Uep3TLen_R8UhTxLenSpec>;
#[doc = "Register `R8_UEP3_T_LEN__R8_UH_TX_LEN` writer"]
pub type W = crate::W<R8Uep3TLen_R8UhTxLenSpec>;
#[doc = "Field `R8_UEP3_T_LEN__R8_UH_TX_LEN` reader - endpoint 1 transmittal length"]
pub type R8Uep3TLen_R8UhTxLenR = crate::FieldReader;
#[doc = "Field `R8_UEP3_T_LEN__R8_UH_TX_LEN` writer - endpoint 1 transmittal length"]
pub type R8Uep3TLen_R8UhTxLenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - endpoint 1 transmittal length"]
    #[inline(always)]
    pub fn r8_uep3_t_len__r8_uh_tx_len(&self) -> R8Uep3TLen_R8UhTxLenR {
        R8Uep3TLen_R8UhTxLenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - endpoint 1 transmittal length"]
    #[inline(always)]
    pub fn r8_uep3_t_len__r8_uh_tx_len(
        &mut self,
    ) -> R8Uep3TLen_R8UhTxLenW<'_, R8Uep3TLen_R8UhTxLenSpec> {
        R8Uep3TLen_R8UhTxLenW::new(self, 0)
    }
}
#[doc = "endpoint 3 transmittal length;host transmittal endpoint transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep3_t_len__r8_uh_tx_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep3_t_len__r8_uh_tx_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep3TLen_R8UhTxLenSpec;
impl crate::RegisterSpec for R8Uep3TLen_R8UhTxLenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep3_t_len__r8_uh_tx_len::R`](R) reader structure"]
impl crate::Readable for R8Uep3TLen_R8UhTxLenSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep3_t_len__r8_uh_tx_len::W`](W) writer structure"]
impl crate::Writable for R8Uep3TLen_R8UhTxLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP3_T_LEN__R8_UH_TX_LEN to value 0"]
impl crate::Resettable for R8Uep3TLen_R8UhTxLenSpec {}
