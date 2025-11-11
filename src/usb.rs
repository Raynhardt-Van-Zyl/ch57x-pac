#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_usb_ctrl: R8UsbCtrl,
    r8_udev_ctrl__r8_uhost_ctrl: R8UdevCtrl_R8UhostCtrl,
    r8_usb_int_en: R8UsbIntEn,
    r8_usb_dev_ad: R8UsbDevAd,
    _reserved4: [u8; 0x01],
    r8_usb_mis_st: R8UsbMisSt,
    r8_usb_int_fg: R8UsbIntFg,
    r8_usb_int_st: R8UsbIntSt,
    r8_usb_rx_len: R8UsbRxLen,
    _reserved8: [u8; 0x03],
    r8_uep4_1_mod: R8Uep4_1Mod,
    r8_uep2_3_mod__r8_uh_ep_mod: R8Uep2_3Mod_R8UhEpMod,
    _reserved10: [u8; 0x02],
    r16_uep0_dma: R16Uep0Dma,
    _reserved11: [u8; 0x02],
    r16_uep1_dma: R16Uep1Dma,
    _reserved12: [u8; 0x02],
    r16_uep2_dma__r16_uh_rx_dma: R16Uep2Dma_R16UhRxDma,
    _reserved13: [u8; 0x02],
    r16_uep3_dma__r16_uh_tx_dma: R16Uep3Dma_R16UhTxDma,
    _reserved14: [u8; 0x02],
    r8_uep0_t_len: R8Uep0TLen,
    _reserved15: [u8; 0x01],
    r8_uep0_ctrl: R8Uep0Ctrl,
    _reserved16: [u8; 0x01],
    r8_uep1_t_len: R8Uep1TLen,
    _reserved17: [u8; 0x01],
    r8_uep1_ctrl__r8_uh_setup: R8Uep1Ctrl_R8UhSetup,
    _reserved18: [u8; 0x01],
    r8_uep2_t_len_r8_uh_ep_pid: R8Uep2TLenR8UhEpPid,
    _reserved19: [u8; 0x01],
    r8_uep2_ctrl_r8_uh_rx_ctrl: R8Uep2CtrlR8UhRxCtrl,
    _reserved20: [u8; 0x01],
    r8_uep3_t_len__r8_uh_tx_len: R8Uep3TLen_R8UhTxLen,
    _reserved21: [u8; 0x01],
    r8_uep3_ctrl__r8_uh_tx_ctrl: R8Uep3Ctrl_R8UhTxCtrl,
    _reserved22: [u8; 0x01],
    r8_uep4_t_len: R8Uep4TLen,
    _reserved23: [u8; 0x01],
    r8_uep4_ctrl: R8Uep4Ctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - USB base control"]
    #[inline(always)]
    pub const fn r8_usb_ctrl(&self) -> &R8UsbCtrl {
        &self.r8_usb_ctrl
    }
    #[doc = "0x01 - USB device physical prot control"]
    #[inline(always)]
    pub const fn r8_udev_ctrl__r8_uhost_ctrl(&self) -> &R8UdevCtrl_R8UhostCtrl {
        &self.r8_udev_ctrl__r8_uhost_ctrl
    }
    #[doc = "0x02 - USB interrupt enable"]
    #[inline(always)]
    pub const fn r8_usb_int_en(&self) -> &R8UsbIntEn {
        &self.r8_usb_int_en
    }
    #[doc = "0x03 - USB device address"]
    #[inline(always)]
    pub const fn r8_usb_dev_ad(&self) -> &R8UsbDevAd {
        &self.r8_usb_dev_ad
    }
    #[doc = "0x05 - USB miscellaneous status"]
    #[inline(always)]
    pub const fn r8_usb_mis_st(&self) -> &R8UsbMisSt {
        &self.r8_usb_mis_st
    }
    #[doc = "0x06 - USB interrupt flag"]
    #[inline(always)]
    pub const fn r8_usb_int_fg(&self) -> &R8UsbIntFg {
        &self.r8_usb_int_fg
    }
    #[doc = "0x07 - USB interrupt status"]
    #[inline(always)]
    pub const fn r8_usb_int_st(&self) -> &R8UsbIntSt {
        &self.r8_usb_int_st
    }
    #[doc = "0x08 - USB receiving length"]
    #[inline(always)]
    pub const fn r8_usb_rx_len(&self) -> &R8UsbRxLen {
        &self.r8_usb_rx_len
    }
    #[doc = "0x0c - endpoint 4/1 mode"]
    #[inline(always)]
    pub const fn r8_uep4_1_mod(&self) -> &R8Uep4_1Mod {
        &self.r8_uep4_1_mod
    }
    #[doc = "0x0d - endpoint 2_3 mode;host endpoint mode"]
    #[inline(always)]
    pub const fn r8_uep2_3_mod__r8_uh_ep_mod(&self) -> &R8Uep2_3Mod_R8UhEpMod {
        &self.r8_uep2_3_mod__r8_uh_ep_mod
    }
    #[doc = "0x10 - endpoint 0 DMA buffer address"]
    #[inline(always)]
    pub const fn r16_uep0_dma(&self) -> &R16Uep0Dma {
        &self.r16_uep0_dma
    }
    #[doc = "0x14 - endpoint 1 DMA buffer address"]
    #[inline(always)]
    pub const fn r16_uep1_dma(&self) -> &R16Uep1Dma {
        &self.r16_uep1_dma
    }
    #[doc = "0x18 - endpoint 2 DMA buffer address;host rx endpoint buffer high address"]
    #[inline(always)]
    pub const fn r16_uep2_dma__r16_uh_rx_dma(&self) -> &R16Uep2Dma_R16UhRxDma {
        &self.r16_uep2_dma__r16_uh_rx_dma
    }
    #[doc = "0x1c - endpoint 3 DMA buffer address;host tx endpoint buffer high address"]
    #[inline(always)]
    pub const fn r16_uep3_dma__r16_uh_tx_dma(&self) -> &R16Uep3Dma_R16UhTxDma {
        &self.r16_uep3_dma__r16_uh_tx_dma
    }
    #[doc = "0x20 - endpoint 0 transmittal length"]
    #[inline(always)]
    pub const fn r8_uep0_t_len(&self) -> &R8Uep0TLen {
        &self.r8_uep0_t_len
    }
    #[doc = "0x22 - endpoint 0 control"]
    #[inline(always)]
    pub const fn r8_uep0_ctrl(&self) -> &R8Uep0Ctrl {
        &self.r8_uep0_ctrl
    }
    #[doc = "0x24 - endpoint 1 transmittal length"]
    #[inline(always)]
    pub const fn r8_uep1_t_len(&self) -> &R8Uep1TLen {
        &self.r8_uep1_t_len
    }
    #[doc = "0x26 - endpoint 1 control;host aux setup"]
    #[inline(always)]
    pub const fn r8_uep1_ctrl__r8_uh_setup(&self) -> &R8Uep1Ctrl_R8UhSetup {
        &self.r8_uep1_ctrl__r8_uh_setup
    }
    #[doc = "0x28 - endpoint 2 transmittal length;host endpoint and PID"]
    #[inline(always)]
    pub const fn r8_uep2_t_len_r8_uh_ep_pid(&self) -> &R8Uep2TLenR8UhEpPid {
        &self.r8_uep2_t_len_r8_uh_ep_pid
    }
    #[doc = "0x2a - endpoint 2 control;host receiver endpoint control"]
    #[inline(always)]
    pub const fn r8_uep2_ctrl_r8_uh_rx_ctrl(&self) -> &R8Uep2CtrlR8UhRxCtrl {
        &self.r8_uep2_ctrl_r8_uh_rx_ctrl
    }
    #[doc = "0x2c - endpoint 3 transmittal length;host transmittal endpoint transmittal length"]
    #[inline(always)]
    pub const fn r8_uep3_t_len__r8_uh_tx_len(&self) -> &R8Uep3TLen_R8UhTxLen {
        &self.r8_uep3_t_len__r8_uh_tx_len
    }
    #[doc = "0x2e - endpoint 3 control;host transmittal endpoint control"]
    #[inline(always)]
    pub const fn r8_uep3_ctrl__r8_uh_tx_ctrl(&self) -> &R8Uep3Ctrl_R8UhTxCtrl {
        &self.r8_uep3_ctrl__r8_uh_tx_ctrl
    }
    #[doc = "0x30 - endpoint 4 transmittal length"]
    #[inline(always)]
    pub const fn r8_uep4_t_len(&self) -> &R8Uep4TLen {
        &self.r8_uep4_t_len
    }
    #[doc = "0x32 - endpoint 4 control"]
    #[inline(always)]
    pub const fn r8_uep4_ctrl(&self) -> &R8Uep4Ctrl {
        &self.r8_uep4_ctrl
    }
}
#[doc = "R8_USB_CTRL (rw) register accessor: USB base control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_ctrl`] module"]
#[doc(alias = "R8_USB_CTRL")]
pub type R8UsbCtrl = crate::Reg<r8_usb_ctrl::R8UsbCtrlSpec>;
#[doc = "USB base control"]
pub mod r8_usb_ctrl;
#[doc = "R8_UDEV_CTRL__R8_UHOST_CTRL (rw) register accessor: USB device physical prot control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_udev_ctrl__r8_uhost_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_udev_ctrl__r8_uhost_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_udev_ctrl__r8_uhost_ctrl`] module"]
#[doc(alias = "R8_UDEV_CTRL__R8_UHOST_CTRL")]
pub type R8UdevCtrl_R8UhostCtrl =
    crate::Reg<r8_udev_ctrl__r8_uhost_ctrl::R8UdevCtrl_R8UhostCtrlSpec>;
#[doc = "USB device physical prot control"]
pub mod r8_udev_ctrl__r8_uhost_ctrl;
#[doc = "R8_USB_INT_EN (rw) register accessor: USB interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_int_en`] module"]
#[doc(alias = "R8_USB_INT_EN")]
pub type R8UsbIntEn = crate::Reg<r8_usb_int_en::R8UsbIntEnSpec>;
#[doc = "USB interrupt enable"]
pub mod r8_usb_int_en;
#[doc = "R8_USB_DEV_AD (rw) register accessor: USB device address\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_dev_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_dev_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_dev_ad`] module"]
#[doc(alias = "R8_USB_DEV_AD")]
pub type R8UsbDevAd = crate::Reg<r8_usb_dev_ad::R8UsbDevAdSpec>;
#[doc = "USB device address"]
pub mod r8_usb_dev_ad;
#[doc = "R8_USB_MIS_ST (r) register accessor: USB miscellaneous status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_mis_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_mis_st`] module"]
#[doc(alias = "R8_USB_MIS_ST")]
pub type R8UsbMisSt = crate::Reg<r8_usb_mis_st::R8UsbMisStSpec>;
#[doc = "USB miscellaneous status"]
pub mod r8_usb_mis_st;
#[doc = "R8_USB_INT_FG (rw) register accessor: USB interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_fg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_int_fg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_int_fg`] module"]
#[doc(alias = "R8_USB_INT_FG")]
pub type R8UsbIntFg = crate::Reg<r8_usb_int_fg::R8UsbIntFgSpec>;
#[doc = "USB interrupt flag"]
pub mod r8_usb_int_fg;
#[doc = "R8_USB_INT_ST (r) register accessor: USB interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_int_st`] module"]
#[doc(alias = "R8_USB_INT_ST")]
pub type R8UsbIntSt = crate::Reg<r8_usb_int_st::R8UsbIntStSpec>;
#[doc = "USB interrupt status"]
pub mod r8_usb_int_st;
#[doc = "R8_USB_RX_LEN (r) register accessor: USB receiving length\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_rx_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_rx_len`] module"]
#[doc(alias = "R8_USB_RX_LEN")]
pub type R8UsbRxLen = crate::Reg<r8_usb_rx_len::R8UsbRxLenSpec>;
#[doc = "USB receiving length"]
pub mod r8_usb_rx_len;
#[doc = "R8_UEP4_1_MOD (rw) register accessor: endpoint 4/1 mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep4_1_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep4_1_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep4_1_mod`] module"]
#[doc(alias = "R8_UEP4_1_MOD")]
pub type R8Uep4_1Mod = crate::Reg<r8_uep4_1_mod::R8Uep4_1ModSpec>;
#[doc = "endpoint 4/1 mode"]
pub mod r8_uep4_1_mod;
#[doc = "R8_UEP2_3_MOD__R8_UH_EP_MOD (rw) register accessor: endpoint 2_3 mode;host endpoint mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep2_3_mod__r8_uh_ep_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep2_3_mod__r8_uh_ep_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep2_3_mod__r8_uh_ep_mod`] module"]
#[doc(alias = "R8_UEP2_3_MOD__R8_UH_EP_MOD")]
pub type R8Uep2_3Mod_R8UhEpMod = crate::Reg<r8_uep2_3_mod__r8_uh_ep_mod::R8Uep2_3Mod_R8UhEpModSpec>;
#[doc = "endpoint 2_3 mode;host endpoint mode"]
pub mod r8_uep2_3_mod__r8_uh_ep_mod;
#[doc = "R16_UEP0_DMA (rw) register accessor: endpoint 0 DMA buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep0_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep0_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep0_dma`] module"]
#[doc(alias = "R16_UEP0_DMA")]
pub type R16Uep0Dma = crate::Reg<r16_uep0_dma::R16Uep0DmaSpec>;
#[doc = "endpoint 0 DMA buffer address"]
pub mod r16_uep0_dma;
#[doc = "R16_UEP1_DMA (rw) register accessor: endpoint 1 DMA buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep1_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep1_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep1_dma`] module"]
#[doc(alias = "R16_UEP1_DMA")]
pub type R16Uep1Dma = crate::Reg<r16_uep1_dma::R16Uep1DmaSpec>;
#[doc = "endpoint 1 DMA buffer address"]
pub mod r16_uep1_dma;
#[doc = "R16_UEP2_DMA__R16_UH_RX_DMA (rw) register accessor: endpoint 2 DMA buffer address;host rx endpoint buffer high address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep2_dma__r16_uh_rx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep2_dma__r16_uh_rx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep2_dma__r16_uh_rx_dma`] module"]
#[doc(alias = "R16_UEP2_DMA__R16_UH_RX_DMA")]
pub type R16Uep2Dma_R16UhRxDma = crate::Reg<r16_uep2_dma__r16_uh_rx_dma::R16Uep2Dma_R16UhRxDmaSpec>;
#[doc = "endpoint 2 DMA buffer address;host rx endpoint buffer high address"]
pub mod r16_uep2_dma__r16_uh_rx_dma;
#[doc = "R16_UEP3_DMA__R16_UH_TX_DMA (rw) register accessor: endpoint 3 DMA buffer address;host tx endpoint buffer high address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep3_dma__r16_uh_tx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep3_dma__r16_uh_tx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep3_dma__r16_uh_tx_dma`] module"]
#[doc(alias = "R16_UEP3_DMA__R16_UH_TX_DMA")]
pub type R16Uep3Dma_R16UhTxDma = crate::Reg<r16_uep3_dma__r16_uh_tx_dma::R16Uep3Dma_R16UhTxDmaSpec>;
#[doc = "endpoint 3 DMA buffer address;host tx endpoint buffer high address"]
pub mod r16_uep3_dma__r16_uh_tx_dma;
#[doc = "R8_UEP0_T_LEN (rw) register accessor: endpoint 0 transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep0_t_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep0_t_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep0_t_len`] module"]
#[doc(alias = "R8_UEP0_T_LEN")]
pub type R8Uep0TLen = crate::Reg<r8_uep0_t_len::R8Uep0TLenSpec>;
#[doc = "endpoint 0 transmittal length"]
pub mod r8_uep0_t_len;
#[doc = "R8_UEP0_CTRL (rw) register accessor: endpoint 0 control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep0_ctrl`] module"]
#[doc(alias = "R8_UEP0_CTRL")]
pub type R8Uep0Ctrl = crate::Reg<r8_uep0_ctrl::R8Uep0CtrlSpec>;
#[doc = "endpoint 0 control"]
pub mod r8_uep0_ctrl;
#[doc = "R8_UEP1_T_LEN (rw) register accessor: endpoint 1 transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep1_t_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep1_t_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep1_t_len`] module"]
#[doc(alias = "R8_UEP1_T_LEN")]
pub type R8Uep1TLen = crate::Reg<r8_uep1_t_len::R8Uep1TLenSpec>;
#[doc = "endpoint 1 transmittal length"]
pub mod r8_uep1_t_len;
#[doc = "R8_UEP1_CTRL__R8_UH_SETUP (rw) register accessor: endpoint 1 control;host aux setup\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep1_ctrl__r8_uh_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep1_ctrl__r8_uh_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep1_ctrl__r8_uh_setup`] module"]
#[doc(alias = "R8_UEP1_CTRL__R8_UH_SETUP")]
pub type R8Uep1Ctrl_R8UhSetup = crate::Reg<r8_uep1_ctrl__r8_uh_setup::R8Uep1Ctrl_R8UhSetupSpec>;
#[doc = "endpoint 1 control;host aux setup"]
pub mod r8_uep1_ctrl__r8_uh_setup;
#[doc = "R8_UEP2_T_LEN_R8_UH_EP_PID (rw) register accessor: endpoint 2 transmittal length;host endpoint and PID\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep2_t_len_r8_uh_ep_pid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep2_t_len_r8_uh_ep_pid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep2_t_len_r8_uh_ep_pid`] module"]
#[doc(alias = "R8_UEP2_T_LEN_R8_UH_EP_PID")]
pub type R8Uep2TLenR8UhEpPid = crate::Reg<r8_uep2_t_len_r8_uh_ep_pid::R8Uep2TLenR8UhEpPidSpec>;
#[doc = "endpoint 2 transmittal length;host endpoint and PID"]
pub mod r8_uep2_t_len_r8_uh_ep_pid;
#[doc = "R8_UEP2_CTRL_R8_UH_RX_CTRL (rw) register accessor: endpoint 2 control;host receiver endpoint control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep2_ctrl_r8_uh_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep2_ctrl_r8_uh_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep2_ctrl_r8_uh_rx_ctrl`] module"]
#[doc(alias = "R8_UEP2_CTRL_R8_UH_RX_CTRL")]
pub type R8Uep2CtrlR8UhRxCtrl = crate::Reg<r8_uep2_ctrl_r8_uh_rx_ctrl::R8Uep2CtrlR8UhRxCtrlSpec>;
#[doc = "endpoint 2 control;host receiver endpoint control"]
pub mod r8_uep2_ctrl_r8_uh_rx_ctrl;
#[doc = "R8_UEP3_T_LEN__R8_UH_TX_LEN (rw) register accessor: endpoint 3 transmittal length;host transmittal endpoint transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep3_t_len__r8_uh_tx_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep3_t_len__r8_uh_tx_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep3_t_len__r8_uh_tx_len`] module"]
#[doc(alias = "R8_UEP3_T_LEN__R8_UH_TX_LEN")]
pub type R8Uep3TLen_R8UhTxLen = crate::Reg<r8_uep3_t_len__r8_uh_tx_len::R8Uep3TLen_R8UhTxLenSpec>;
#[doc = "endpoint 3 transmittal length;host transmittal endpoint transmittal length"]
pub mod r8_uep3_t_len__r8_uh_tx_len;
#[doc = "R8_UEP3_CTRL__R8_UH_TX_CTRL (rw) register accessor: endpoint 3 control;host transmittal endpoint control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep3_ctrl__r8_uh_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep3_ctrl__r8_uh_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep3_ctrl__r8_uh_tx_ctrl`] module"]
#[doc(alias = "R8_UEP3_CTRL__R8_UH_TX_CTRL")]
pub type R8Uep3Ctrl_R8UhTxCtrl = crate::Reg<r8_uep3_ctrl__r8_uh_tx_ctrl::R8Uep3Ctrl_R8UhTxCtrlSpec>;
#[doc = "endpoint 3 control;host transmittal endpoint control"]
pub mod r8_uep3_ctrl__r8_uh_tx_ctrl;
#[doc = "R8_UEP4_T_LEN (rw) register accessor: endpoint 4 transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep4_t_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep4_t_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep4_t_len`] module"]
#[doc(alias = "R8_UEP4_T_LEN")]
pub type R8Uep4TLen = crate::Reg<r8_uep4_t_len::R8Uep4TLenSpec>;
#[doc = "endpoint 4 transmittal length"]
pub mod r8_uep4_t_len;
#[doc = "R8_UEP4_CTRL (rw) register accessor: endpoint 4 control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep4_ctrl`] module"]
#[doc(alias = "R8_UEP4_CTRL")]
pub type R8Uep4Ctrl = crate::Reg<r8_uep4_ctrl::R8Uep4CtrlSpec>;
#[doc = "endpoint 4 control"]
pub mod r8_uep4_ctrl;
