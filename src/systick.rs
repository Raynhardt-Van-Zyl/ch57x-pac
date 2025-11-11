#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r32_stk_ctlr: R32StkCtlr,
    r32_stk_cntl: R32StkCntl,
    r32_stk_cnth: R32StkCnth,
    r32_stk_cmplr: R32StkCmplr,
    r32_stk_cmphr: R32StkCmphr,
    r32_stk_cntfg: R32StkCntfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Systick counter control register"]
    #[inline(always)]
    pub const fn r32_stk_ctlr(&self) -> &R32StkCtlr {
        &self.r32_stk_ctlr
    }
    #[doc = "0x04 - Systick counter low register"]
    #[inline(always)]
    pub const fn r32_stk_cntl(&self) -> &R32StkCntl {
        &self.r32_stk_cntl
    }
    #[doc = "0x08 - Systick counter high register"]
    #[inline(always)]
    pub const fn r32_stk_cnth(&self) -> &R32StkCnth {
        &self.r32_stk_cnth
    }
    #[doc = "0x0c - Systick compare low register"]
    #[inline(always)]
    pub const fn r32_stk_cmplr(&self) -> &R32StkCmplr {
        &self.r32_stk_cmplr
    }
    #[doc = "0x10 - Systick compare high register"]
    #[inline(always)]
    pub const fn r32_stk_cmphr(&self) -> &R32StkCmphr {
        &self.r32_stk_cmphr
    }
    #[doc = "0x14 - Systick counter flag"]
    #[inline(always)]
    pub const fn r32_stk_cntfg(&self) -> &R32StkCntfg {
        &self.r32_stk_cntfg
    }
}
#[doc = "R32_STK_CTLR (rw) register accessor: Systick counter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_stk_ctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_stk_ctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_stk_ctlr`] module"]
#[doc(alias = "R32_STK_CTLR")]
pub type R32StkCtlr = crate::Reg<r32_stk_ctlr::R32StkCtlrSpec>;
#[doc = "Systick counter control register"]
pub mod r32_stk_ctlr;
#[doc = "R32_STK_CNTL (rw) register accessor: Systick counter low register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_stk_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_stk_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_stk_cntl`] module"]
#[doc(alias = "R32_STK_CNTL")]
pub type R32StkCntl = crate::Reg<r32_stk_cntl::R32StkCntlSpec>;
#[doc = "Systick counter low register"]
pub mod r32_stk_cntl;
#[doc = "R32_STK_CNTH (rw) register accessor: Systick counter high register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_stk_cnth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_stk_cnth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_stk_cnth`] module"]
#[doc(alias = "R32_STK_CNTH")]
pub type R32StkCnth = crate::Reg<r32_stk_cnth::R32StkCnthSpec>;
#[doc = "Systick counter high register"]
pub mod r32_stk_cnth;
#[doc = "R32_STK_CMPLR (rw) register accessor: Systick compare low register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_stk_cmplr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_stk_cmplr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_stk_cmplr`] module"]
#[doc(alias = "R32_STK_CMPLR")]
pub type R32StkCmplr = crate::Reg<r32_stk_cmplr::R32StkCmplrSpec>;
#[doc = "Systick compare low register"]
pub mod r32_stk_cmplr;
#[doc = "R32_STK_CMPHR (rw) register accessor: Systick compare high register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_stk_cmphr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_stk_cmphr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_stk_cmphr`] module"]
#[doc(alias = "R32_STK_CMPHR")]
pub type R32StkCmphr = crate::Reg<r32_stk_cmphr::R32StkCmphrSpec>;
#[doc = "Systick compare high register"]
pub mod r32_stk_cmphr;
#[doc = "R32_STK_CNTFG (rw) register accessor: Systick counter flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_stk_cntfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_stk_cntfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_stk_cntfg`] module"]
#[doc(alias = "R32_STK_CNTFG")]
pub type R32StkCntfg = crate::Reg<r32_stk_cntfg::R32StkCntfgSpec>;
#[doc = "Systick counter flag"]
pub mod r32_stk_cntfg;
