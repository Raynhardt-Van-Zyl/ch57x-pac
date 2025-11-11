#[doc = "Register `R8_UEP2_3_MOD__R8_UH_EP_MOD` reader"]
pub type R = crate::R<R8Uep2_3Mod_R8UhEpModSpec>;
#[doc = "Register `R8_UEP2_3_MOD__R8_UH_EP_MOD` writer"]
pub type W = crate::W<R8Uep2_3Mod_R8UhEpModSpec>;
#[doc = "Field `RB_UEP2_BUF_MOD__RB_UH_EP_RBUF_MOD` reader - buffer mode of USB endpoint 2;buffer mode of USB host IN endpoint"]
pub type RbUep2BufMod_RbUhEpRbufModR = crate::BitReader;
#[doc = "Field `RB_UEP2_BUF_MOD__RB_UH_EP_RBUF_MOD` writer - buffer mode of USB endpoint 2;buffer mode of USB host IN endpoint"]
pub type RbUep2BufMod_RbUhEpRbufModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP2_TX_EN` reader - enable USB endpoint 2 transmittal (IN)"]
pub type RbUep2TxEnR = crate::BitReader;
#[doc = "Field `RB_UEP2_TX_EN` writer - enable USB endpoint 2 transmittal (IN)"]
pub type RbUep2TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP2_RX_EN__RB_UH_EP_RX_EN` reader - enable USB endpoint 2 receiving (OUT);enable USB host IN endpoint receiving"]
pub type RbUep2RxEn_RbUhEpRxEnR = crate::BitReader;
#[doc = "Field `RB_UEP2_RX_EN__RB_UH_EP_RX_EN` writer - enable USB endpoint 2 receiving (OUT);enable USB host IN endpoint receiving"]
pub type RbUep2RxEn_RbUhEpRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP3_BUF_MOD__RB_UH_EP_TBUF_MOD` reader - buffer mode of USB endpoint 3;buffer mode of USB host OUT endpoint"]
pub type RbUep3BufMod_RbUhEpTbufModR = crate::BitReader;
#[doc = "Field `RB_UEP3_BUF_MOD__RB_UH_EP_TBUF_MOD` writer - buffer mode of USB endpoint 3;buffer mode of USB host OUT endpoint"]
pub type RbUep3BufMod_RbUhEpTbufModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP3_TX_EN__RB_UH_EP_TX_EN` reader - enable USB endpoint 3 transmittal (IN);enable USB host OUT endpoint transmittal"]
pub type RbUep3TxEn_RbUhEpTxEnR = crate::BitReader;
#[doc = "Field `RB_UEP3_TX_EN__RB_UH_EP_TX_EN` writer - enable USB endpoint 3 transmittal (IN);enable USB host OUT endpoint transmittal"]
pub type RbUep3TxEn_RbUhEpTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP3_RX_EN` reader - enable USB endpoint 3 receiving (OUT)"]
pub type RbUep3RxEnR = crate::BitReader;
#[doc = "Field `RB_UEP3_RX_EN` writer - enable USB endpoint 3 receiving (OUT)"]
pub type RbUep3RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - buffer mode of USB endpoint 2;buffer mode of USB host IN endpoint"]
    #[inline(always)]
    pub fn rb_uep2_buf_mod__rb_uh_ep_rbuf_mod(&self) -> RbUep2BufMod_RbUhEpRbufModR {
        RbUep2BufMod_RbUhEpRbufModR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 2 transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep2_tx_en(&self) -> RbUep2TxEnR {
        RbUep2TxEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable USB endpoint 2 receiving (OUT);enable USB host IN endpoint receiving"]
    #[inline(always)]
    pub fn rb_uep2_rx_en__rb_uh_ep_rx_en(&self) -> RbUep2RxEn_RbUhEpRxEnR {
        RbUep2RxEn_RbUhEpRxEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 3;buffer mode of USB host OUT endpoint"]
    #[inline(always)]
    pub fn rb_uep3_buf_mod__rb_uh_ep_tbuf_mod(&self) -> RbUep3BufMod_RbUhEpTbufModR {
        RbUep3BufMod_RbUhEpTbufModR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - enable USB endpoint 3 transmittal (IN);enable USB host OUT endpoint transmittal"]
    #[inline(always)]
    pub fn rb_uep3_tx_en__rb_uh_ep_tx_en(&self) -> RbUep3TxEn_RbUhEpTxEnR {
        RbUep3TxEn_RbUhEpTxEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable USB endpoint 3 receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep3_rx_en(&self) -> RbUep3RxEnR {
        RbUep3RxEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - buffer mode of USB endpoint 2;buffer mode of USB host IN endpoint"]
    #[inline(always)]
    pub fn rb_uep2_buf_mod__rb_uh_ep_rbuf_mod(
        &mut self,
    ) -> RbUep2BufMod_RbUhEpRbufModW<'_, R8Uep2_3Mod_R8UhEpModSpec> {
        RbUep2BufMod_RbUhEpRbufModW::new(self, 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 2 transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep2_tx_en(&mut self) -> RbUep2TxEnW<'_, R8Uep2_3Mod_R8UhEpModSpec> {
        RbUep2TxEnW::new(self, 2)
    }
    #[doc = "Bit 3 - enable USB endpoint 2 receiving (OUT);enable USB host IN endpoint receiving"]
    #[inline(always)]
    pub fn rb_uep2_rx_en__rb_uh_ep_rx_en(
        &mut self,
    ) -> RbUep2RxEn_RbUhEpRxEnW<'_, R8Uep2_3Mod_R8UhEpModSpec> {
        RbUep2RxEn_RbUhEpRxEnW::new(self, 3)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 3;buffer mode of USB host OUT endpoint"]
    #[inline(always)]
    pub fn rb_uep3_buf_mod__rb_uh_ep_tbuf_mod(
        &mut self,
    ) -> RbUep3BufMod_RbUhEpTbufModW<'_, R8Uep2_3Mod_R8UhEpModSpec> {
        RbUep3BufMod_RbUhEpTbufModW::new(self, 4)
    }
    #[doc = "Bit 6 - enable USB endpoint 3 transmittal (IN);enable USB host OUT endpoint transmittal"]
    #[inline(always)]
    pub fn rb_uep3_tx_en__rb_uh_ep_tx_en(
        &mut self,
    ) -> RbUep3TxEn_RbUhEpTxEnW<'_, R8Uep2_3Mod_R8UhEpModSpec> {
        RbUep3TxEn_RbUhEpTxEnW::new(self, 6)
    }
    #[doc = "Bit 7 - enable USB endpoint 3 receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep3_rx_en(&mut self) -> RbUep3RxEnW<'_, R8Uep2_3Mod_R8UhEpModSpec> {
        RbUep3RxEnW::new(self, 7)
    }
}
#[doc = "endpoint 2_3 mode;host endpoint mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep2_3_mod__r8_uh_ep_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep2_3_mod__r8_uh_ep_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep2_3Mod_R8UhEpModSpec;
impl crate::RegisterSpec for R8Uep2_3Mod_R8UhEpModSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep2_3_mod__r8_uh_ep_mod::R`](R) reader structure"]
impl crate::Readable for R8Uep2_3Mod_R8UhEpModSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep2_3_mod__r8_uh_ep_mod::W`](W) writer structure"]
impl crate::Writable for R8Uep2_3Mod_R8UhEpModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP2_3_MOD__R8_UH_EP_MOD to value 0"]
impl crate::Resettable for R8Uep2_3Mod_R8UhEpModSpec {}
