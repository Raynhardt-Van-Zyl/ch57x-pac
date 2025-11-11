#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_tmr3_ctrl_mod: R8Tmr3CtrlMod,
    _reserved1: [u8; 0x01],
    r8_tmr3_inter_en: R8Tmr3InterEn,
    _reserved2: [u8; 0x03],
    r8_tmr3_int_flag: R8Tmr3IntFlag,
    r8_tmr3_fifo_count: R8Tmr3FifoCount,
    r32_tmr3_count: R32Tmr3Count,
    r32_tmr3_cnt_end: R32Tmr3CntEnd,
    r32_tmr3_fifo: R32Tmr3Fifo,
}
impl RegisterBlock {
    #[doc = "0x00 - RW, TMR3 mode control"]
    #[inline(always)]
    pub const fn r8_tmr3_ctrl_mod(&self) -> &R8Tmr3CtrlMod {
        &self.r8_tmr3_ctrl_mod
    }
    #[doc = "0x02 - RW, TMR3 interrupt enable"]
    #[inline(always)]
    pub const fn r8_tmr3_inter_en(&self) -> &R8Tmr3InterEn {
        &self.r8_tmr3_inter_en
    }
    #[doc = "0x06 - RW1, TMR3 interrupt flag"]
    #[inline(always)]
    pub const fn r8_tmr3_int_flag(&self) -> &R8Tmr3IntFlag {
        &self.r8_tmr3_int_flag
    }
    #[doc = "0x07 - RO, TMR3 FIFO count status"]
    #[inline(always)]
    pub const fn r8_tmr3_fifo_count(&self) -> &R8Tmr3FifoCount {
        &self.r8_tmr3_fifo_count
    }
    #[doc = "0x08 - RO, TMR3 current count"]
    #[inline(always)]
    pub const fn r32_tmr3_count(&self) -> &R32Tmr3Count {
        &self.r32_tmr3_count
    }
    #[doc = "0x0c - RW, TMR3 end count value, only low 26 bit"]
    #[inline(always)]
    pub const fn r32_tmr3_cnt_end(&self) -> &R32Tmr3CntEnd {
        &self.r32_tmr3_cnt_end
    }
    #[doc = "0x10 - RO/WO, TMR3 FIFO register, only low 26 bit"]
    #[inline(always)]
    pub const fn r32_tmr3_fifo(&self) -> &R32Tmr3Fifo {
        &self.r32_tmr3_fifo
    }
}
#[doc = "R8_TMR3_CTRL_MOD (rw) register accessor: RW, TMR3 mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr3_ctrl_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr3_ctrl_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr3_ctrl_mod`] module"]
#[doc(alias = "R8_TMR3_CTRL_MOD")]
pub type R8Tmr3CtrlMod = crate::Reg<r8_tmr3_ctrl_mod::R8Tmr3CtrlModSpec>;
#[doc = "RW, TMR3 mode control"]
pub mod r8_tmr3_ctrl_mod;
#[doc = "R8_TMR3_INTER_EN (rw) register accessor: RW, TMR3 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr3_inter_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr3_inter_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr3_inter_en`] module"]
#[doc(alias = "R8_TMR3_INTER_EN")]
pub type R8Tmr3InterEn = crate::Reg<r8_tmr3_inter_en::R8Tmr3InterEnSpec>;
#[doc = "RW, TMR3 interrupt enable"]
pub mod r8_tmr3_inter_en;
#[doc = "R8_TMR3_INT_FLAG (rw) register accessor: RW1, TMR3 interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr3_int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr3_int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr3_int_flag`] module"]
#[doc(alias = "R8_TMR3_INT_FLAG")]
pub type R8Tmr3IntFlag = crate::Reg<r8_tmr3_int_flag::R8Tmr3IntFlagSpec>;
#[doc = "RW1, TMR3 interrupt flag"]
pub mod r8_tmr3_int_flag;
#[doc = "R8_TMR3_FIFO_COUNT (r) register accessor: RO, TMR3 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr3_fifo_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr3_fifo_count`] module"]
#[doc(alias = "R8_TMR3_FIFO_COUNT")]
pub type R8Tmr3FifoCount = crate::Reg<r8_tmr3_fifo_count::R8Tmr3FifoCountSpec>;
#[doc = "RO, TMR3 FIFO count status"]
pub mod r8_tmr3_fifo_count;
#[doc = "R32_TMR3_COUNT (r) register accessor: RO, TMR3 current count\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr3_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr3_count`] module"]
#[doc(alias = "R32_TMR3_COUNT")]
pub type R32Tmr3Count = crate::Reg<r32_tmr3_count::R32Tmr3CountSpec>;
#[doc = "RO, TMR3 current count"]
pub mod r32_tmr3_count;
#[doc = "R32_TMR3_CNT_END (rw) register accessor: RW, TMR3 end count value, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr3_cnt_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr3_cnt_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr3_cnt_end`] module"]
#[doc(alias = "R32_TMR3_CNT_END")]
pub type R32Tmr3CntEnd = crate::Reg<r32_tmr3_cnt_end::R32Tmr3CntEndSpec>;
#[doc = "RW, TMR3 end count value, only low 26 bit"]
pub mod r32_tmr3_cnt_end;
#[doc = "R32_TMR3_FIFO (rw) register accessor: RO/WO, TMR3 FIFO register, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr3_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr3_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr3_fifo`] module"]
#[doc(alias = "R32_TMR3_FIFO")]
pub type R32Tmr3Fifo = crate::Reg<r32_tmr3_fifo::R32Tmr3FifoSpec>;
#[doc = "RO/WO, TMR3 FIFO register, only low 26 bit"]
pub mod r32_tmr3_fifo;
