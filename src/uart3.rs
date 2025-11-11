#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_uart3_mcr: R8Uart3Mcr,
    r8_uart3_ier: R8Uart3Ier,
    r8_uart3_fcr: R8Uart3Fcr,
    r8_uart3_lcr: R8Uart3Lcr,
    r8_uart3_iir: R8Uart3Iir,
    r8_uart3_lsr: R8Uart3Lsr,
    _reserved6: [u8; 0x02],
    _reserved_6_r8_uart3: [u8; 0x01],
    _reserved7: [u8; 0x01],
    r8_uart3_rfc: R8Uart3Rfc,
    r8_uart3_tfc: R8Uart3Tfc,
    r16_uart3_dl: R16Uart3Dl,
    r8_uart3_div: R8Uart3Div,
}
impl RegisterBlock {
    #[doc = "0x00 - RW, UART3 modem control"]
    #[inline(always)]
    pub const fn r8_uart3_mcr(&self) -> &R8Uart3Mcr {
        &self.r8_uart3_mcr
    }
    #[doc = "0x01 - RW, UART3 interrupt enable"]
    #[inline(always)]
    pub const fn r8_uart3_ier(&self) -> &R8Uart3Ier {
        &self.r8_uart3_ier
    }
    #[doc = "0x02 - RW, UART3 FIFO control"]
    #[inline(always)]
    pub const fn r8_uart3_fcr(&self) -> &R8Uart3Fcr {
        &self.r8_uart3_fcr
    }
    #[doc = "0x03 - RW, UART3 line control"]
    #[inline(always)]
    pub const fn r8_uart3_lcr(&self) -> &R8Uart3Lcr {
        &self.r8_uart3_lcr
    }
    #[doc = "0x04 - RO, UART3 interrupt identification"]
    #[inline(always)]
    pub const fn r8_uart3_iir(&self) -> &R8Uart3Iir {
        &self.r8_uart3_iir
    }
    #[doc = "0x05 - RO, UART3 line status"]
    #[inline(always)]
    pub const fn r8_uart3_lsr(&self) -> &R8Uart3Lsr {
        &self.r8_uart3_lsr
    }
    #[doc = "0x08 - WO, UART3 transmitter holding, transmittal byte"]
    #[inline(always)]
    pub const fn r8_uart3_thr(&self) -> &R8Uart3Thr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - RO, UART3 receiver buffer, receiving byte"]
    #[inline(always)]
    pub const fn r8_uart3_rbr(&self) -> &R8Uart3Rbr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0a - RO, UART3 receiver FIFO count"]
    #[inline(always)]
    pub const fn r8_uart3_rfc(&self) -> &R8Uart3Rfc {
        &self.r8_uart3_rfc
    }
    #[doc = "0x0b - RO, UART3 transmitter FIFO count"]
    #[inline(always)]
    pub const fn r8_uart3_tfc(&self) -> &R8Uart3Tfc {
        &self.r8_uart3_tfc
    }
    #[doc = "0x0c - RW, UART3 divisor latch"]
    #[inline(always)]
    pub const fn r16_uart3_dl(&self) -> &R16Uart3Dl {
        &self.r16_uart3_dl
    }
    #[doc = "0x0e - RW, UART3 pre-divisor latch byte, only low 7 bit, from 1 to 128"]
    #[inline(always)]
    pub const fn r8_uart3_div(&self) -> &R8Uart3Div {
        &self.r8_uart3_div
    }
}
#[doc = "R8_UART3_MCR (rw) register accessor: RW, UART3 modem control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_mcr`] module"]
#[doc(alias = "R8_UART3_MCR")]
pub type R8Uart3Mcr = crate::Reg<r8_uart3_mcr::R8Uart3McrSpec>;
#[doc = "RW, UART3 modem control"]
pub mod r8_uart3_mcr;
#[doc = "R8_UART3_IER (rw) register accessor: RW, UART3 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_ier`] module"]
#[doc(alias = "R8_UART3_IER")]
pub type R8Uart3Ier = crate::Reg<r8_uart3_ier::R8Uart3IerSpec>;
#[doc = "RW, UART3 interrupt enable"]
pub mod r8_uart3_ier;
#[doc = "R8_UART3_FCR (rw) register accessor: RW, UART3 FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_fcr`] module"]
#[doc(alias = "R8_UART3_FCR")]
pub type R8Uart3Fcr = crate::Reg<r8_uart3_fcr::R8Uart3FcrSpec>;
#[doc = "RW, UART3 FIFO control"]
pub mod r8_uart3_fcr;
#[doc = "R8_UART3_LCR (rw) register accessor: RW, UART3 line control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_lcr`] module"]
#[doc(alias = "R8_UART3_LCR")]
pub type R8Uart3Lcr = crate::Reg<r8_uart3_lcr::R8Uart3LcrSpec>;
#[doc = "RW, UART3 line control"]
pub mod r8_uart3_lcr;
#[doc = "R8_UART3_IIR (r) register accessor: RO, UART3 interrupt identification\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_iir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_iir`] module"]
#[doc(alias = "R8_UART3_IIR")]
pub type R8Uart3Iir = crate::Reg<r8_uart3_iir::R8Uart3IirSpec>;
#[doc = "RO, UART3 interrupt identification"]
pub mod r8_uart3_iir;
#[doc = "R8_UART3_LSR (r) register accessor: RO, UART3 line status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_lsr`] module"]
#[doc(alias = "R8_UART3_LSR")]
pub type R8Uart3Lsr = crate::Reg<r8_uart3_lsr::R8Uart3LsrSpec>;
#[doc = "RO, UART3 line status"]
pub mod r8_uart3_lsr;
#[doc = "R8_UART3_RBR (r) register accessor: RO, UART3 receiver buffer, receiving byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_rbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_rbr`] module"]
#[doc(alias = "R8_UART3_RBR")]
pub type R8Uart3Rbr = crate::Reg<r8_uart3_rbr::R8Uart3RbrSpec>;
#[doc = "RO, UART3 receiver buffer, receiving byte"]
pub mod r8_uart3_rbr;
#[doc = "R8_UART3_THR (w) register accessor: WO, UART3 transmitter holding, transmittal byte\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_thr`] module"]
#[doc(alias = "R8_UART3_THR")]
pub type R8Uart3Thr = crate::Reg<r8_uart3_thr::R8Uart3ThrSpec>;
#[doc = "WO, UART3 transmitter holding, transmittal byte"]
pub mod r8_uart3_thr;
#[doc = "R8_UART3_RFC (r) register accessor: RO, UART3 receiver FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_rfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_rfc`] module"]
#[doc(alias = "R8_UART3_RFC")]
pub type R8Uart3Rfc = crate::Reg<r8_uart3_rfc::R8Uart3RfcSpec>;
#[doc = "RO, UART3 receiver FIFO count"]
pub mod r8_uart3_rfc;
#[doc = "R8_UART3_TFC (r) register accessor: RO, UART3 transmitter FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_tfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_tfc`] module"]
#[doc(alias = "R8_UART3_TFC")]
pub type R8Uart3Tfc = crate::Reg<r8_uart3_tfc::R8Uart3TfcSpec>;
#[doc = "RO, UART3 transmitter FIFO count"]
pub mod r8_uart3_tfc;
#[doc = "R16_UART3_DL (rw) register accessor: RW, UART3 divisor latch\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uart3_dl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uart3_dl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uart3_dl`] module"]
#[doc(alias = "R16_UART3_DL")]
pub type R16Uart3Dl = crate::Reg<r16_uart3_dl::R16Uart3DlSpec>;
#[doc = "RW, UART3 divisor latch"]
pub mod r16_uart3_dl;
#[doc = "R8_UART3_DIV (rw) register accessor: RW, UART3 pre-divisor latch byte, only low 7 bit, from 1 to 128\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_div`] module"]
#[doc(alias = "R8_UART3_DIV")]
pub type R8Uart3Div = crate::Reg<r8_uart3_div::R8Uart3DivSpec>;
#[doc = "RW, UART3 pre-divisor latch byte, only low 7 bit, from 1 to 128"]
pub mod r8_uart3_div;
