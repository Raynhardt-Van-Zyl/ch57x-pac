#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_uart2_mcr: R8Uart2Mcr,
    r8_uart2_ier: R8Uart2Ier,
    r8_uart2_fcr: R8Uart2Fcr,
    r8_uart2_lcr: R8Uart2Lcr,
    r8_uart2_iir: R8Uart2Iir,
    r8_uart2_lsr: R8Uart2Lsr,
    _reserved6: [u8; 0x02],
    _reserved_6_r8_uart2: [u8; 0x01],
    _reserved7: [u8; 0x01],
    r8_uart2_rfc: R8Uart2Rfc,
    r8_uart2_tfc: R8Uart2Tfc,
    r16_uart2_dl: R16Uart2Dl,
    r8_uart2_div: R8Uart2Div,
}
impl RegisterBlock {
    #[doc = "0x00 - RW, UART2 modem control"]
    #[inline(always)]
    pub const fn r8_uart2_mcr(&self) -> &R8Uart2Mcr {
        &self.r8_uart2_mcr
    }
    #[doc = "0x01 - RW, UART2 interrupt enable"]
    #[inline(always)]
    pub const fn r8_uart2_ier(&self) -> &R8Uart2Ier {
        &self.r8_uart2_ier
    }
    #[doc = "0x02 - RW, UART2 FIFO control"]
    #[inline(always)]
    pub const fn r8_uart2_fcr(&self) -> &R8Uart2Fcr {
        &self.r8_uart2_fcr
    }
    #[doc = "0x03 - RW, UART2 line control"]
    #[inline(always)]
    pub const fn r8_uart2_lcr(&self) -> &R8Uart2Lcr {
        &self.r8_uart2_lcr
    }
    #[doc = "0x04 - RO, UART2 interrupt identification"]
    #[inline(always)]
    pub const fn r8_uart2_iir(&self) -> &R8Uart2Iir {
        &self.r8_uart2_iir
    }
    #[doc = "0x05 - RO, UART2 line status"]
    #[inline(always)]
    pub const fn r8_uart2_lsr(&self) -> &R8Uart2Lsr {
        &self.r8_uart2_lsr
    }
    #[doc = "0x08 - WO, UART2 transmitter holding, transmittal byte"]
    #[inline(always)]
    pub const fn r8_uart2_thr(&self) -> &R8Uart2Thr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - RO, UART2 receiver buffer, receiving byte"]
    #[inline(always)]
    pub const fn r8_uart2_rbr(&self) -> &R8Uart2Rbr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0a - RO, UART2 receiver FIFO count"]
    #[inline(always)]
    pub const fn r8_uart2_rfc(&self) -> &R8Uart2Rfc {
        &self.r8_uart2_rfc
    }
    #[doc = "0x0b - RO, UART2 transmitter FIFO count"]
    #[inline(always)]
    pub const fn r8_uart2_tfc(&self) -> &R8Uart2Tfc {
        &self.r8_uart2_tfc
    }
    #[doc = "0x0c - RW, UART2 divisor latch"]
    #[inline(always)]
    pub const fn r16_uart2_dl(&self) -> &R16Uart2Dl {
        &self.r16_uart2_dl
    }
    #[doc = "0x0e - RW, UART2 pre-divisor latch byte, only low 7 bit, from 1 to 128"]
    #[inline(always)]
    pub const fn r8_uart2_div(&self) -> &R8Uart2Div {
        &self.r8_uart2_div
    }
}
#[doc = "R8_UART2_MCR (rw) register accessor: RW, UART2 modem control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart2_mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart2_mcr`] module"]
#[doc(alias = "R8_UART2_MCR")]
pub type R8Uart2Mcr = crate::Reg<r8_uart2_mcr::R8Uart2McrSpec>;
#[doc = "RW, UART2 modem control"]
pub mod r8_uart2_mcr;
#[doc = "R8_UART2_IER (rw) register accessor: RW, UART2 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart2_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart2_ier`] module"]
#[doc(alias = "R8_UART2_IER")]
pub type R8Uart2Ier = crate::Reg<r8_uart2_ier::R8Uart2IerSpec>;
#[doc = "RW, UART2 interrupt enable"]
pub mod r8_uart2_ier;
#[doc = "R8_UART2_FCR (rw) register accessor: RW, UART2 FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart2_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart2_fcr`] module"]
#[doc(alias = "R8_UART2_FCR")]
pub type R8Uart2Fcr = crate::Reg<r8_uart2_fcr::R8Uart2FcrSpec>;
#[doc = "RW, UART2 FIFO control"]
pub mod r8_uart2_fcr;
#[doc = "R8_UART2_LCR (rw) register accessor: RW, UART2 line control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart2_lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart2_lcr`] module"]
#[doc(alias = "R8_UART2_LCR")]
pub type R8Uart2Lcr = crate::Reg<r8_uart2_lcr::R8Uart2LcrSpec>;
#[doc = "RW, UART2 line control"]
pub mod r8_uart2_lcr;
#[doc = "R8_UART2_IIR (r) register accessor: RO, UART2 interrupt identification\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_iir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart2_iir`] module"]
#[doc(alias = "R8_UART2_IIR")]
pub type R8Uart2Iir = crate::Reg<r8_uart2_iir::R8Uart2IirSpec>;
#[doc = "RO, UART2 interrupt identification"]
pub mod r8_uart2_iir;
#[doc = "R8_UART2_LSR (r) register accessor: RO, UART2 line status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart2_lsr`] module"]
#[doc(alias = "R8_UART2_LSR")]
pub type R8Uart2Lsr = crate::Reg<r8_uart2_lsr::R8Uart2LsrSpec>;
#[doc = "RO, UART2 line status"]
pub mod r8_uart2_lsr;
#[doc = "R8_UART2_RBR (r) register accessor: RO, UART2 receiver buffer, receiving byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_rbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart2_rbr`] module"]
#[doc(alias = "R8_UART2_RBR")]
pub type R8Uart2Rbr = crate::Reg<r8_uart2_rbr::R8Uart2RbrSpec>;
#[doc = "RO, UART2 receiver buffer, receiving byte"]
pub mod r8_uart2_rbr;
#[doc = "R8_UART2_THR (w) register accessor: WO, UART2 transmitter holding, transmittal byte\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart2_thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart2_thr`] module"]
#[doc(alias = "R8_UART2_THR")]
pub type R8Uart2Thr = crate::Reg<r8_uart2_thr::R8Uart2ThrSpec>;
#[doc = "WO, UART2 transmitter holding, transmittal byte"]
pub mod r8_uart2_thr;
#[doc = "R8_UART2_RFC (r) register accessor: RO, UART2 receiver FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_rfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart2_rfc`] module"]
#[doc(alias = "R8_UART2_RFC")]
pub type R8Uart2Rfc = crate::Reg<r8_uart2_rfc::R8Uart2RfcSpec>;
#[doc = "RO, UART2 receiver FIFO count"]
pub mod r8_uart2_rfc;
#[doc = "R8_UART2_TFC (r) register accessor: RO, UART2 transmitter FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_tfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart2_tfc`] module"]
#[doc(alias = "R8_UART2_TFC")]
pub type R8Uart2Tfc = crate::Reg<r8_uart2_tfc::R8Uart2TfcSpec>;
#[doc = "RO, UART2 transmitter FIFO count"]
pub mod r8_uart2_tfc;
#[doc = "R16_UART2_DL (rw) register accessor: RW, UART2 divisor latch\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uart2_dl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uart2_dl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uart2_dl`] module"]
#[doc(alias = "R16_UART2_DL")]
pub type R16Uart2Dl = crate::Reg<r16_uart2_dl::R16Uart2DlSpec>;
#[doc = "RW, UART2 divisor latch"]
pub mod r16_uart2_dl;
#[doc = "R8_UART2_DIV (rw) register accessor: RW, UART2 pre-divisor latch byte, only low 7 bit, from 1 to 128\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart2_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart2_div`] module"]
#[doc(alias = "R8_UART2_DIV")]
pub type R8Uart2Div = crate::Reg<r8_uart2_div::R8Uart2DivSpec>;
#[doc = "RW, UART2 pre-divisor latch byte, only low 7 bit, from 1 to 128"]
pub mod r8_uart2_div;
