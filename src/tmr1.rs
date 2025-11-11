#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_tmr1_ctrl_mod: R8Tmr1CtrlMod,
    r8_tmr1_ctrl_dma: R8Tmr1CtrlDma,
    r8_tmr1_inter_en: R8Tmr1InterEn,
    _reserved3: [u8; 0x03],
    r8_tmr1_int_flag: R8Tmr1IntFlag,
    r8_tmr1_fifo_count: R8Tmr1FifoCount,
    r32_tmr1_count: R32Tmr1Count,
    r32_tmr1_cnt_end: R32Tmr1CntEnd,
    r32_tmr1_fifo: R32Tmr1Fifo,
    r16_tmr1_dma_now: R16Tmr1DmaNow,
    _reserved9: [u8; 0x02],
    r16_tmr1_dma_beg: R16Tmr1DmaBeg,
    _reserved10: [u8; 0x02],
    r16_tmr1_dma_end: R16Tmr1DmaEnd,
}
impl RegisterBlock {
    #[doc = "0x00 - RW, TMR1 mode control"]
    #[inline(always)]
    pub const fn r8_tmr1_ctrl_mod(&self) -> &R8Tmr1CtrlMod {
        &self.r8_tmr1_ctrl_mod
    }
    #[doc = "0x01 - RW, TMR1 DMA control"]
    #[inline(always)]
    pub const fn r8_tmr1_ctrl_dma(&self) -> &R8Tmr1CtrlDma {
        &self.r8_tmr1_ctrl_dma
    }
    #[doc = "0x02 - RW, TMR1 interrupt enable"]
    #[inline(always)]
    pub const fn r8_tmr1_inter_en(&self) -> &R8Tmr1InterEn {
        &self.r8_tmr1_inter_en
    }
    #[doc = "0x06 - RW1, TMR1 interrupt flag"]
    #[inline(always)]
    pub const fn r8_tmr1_int_flag(&self) -> &R8Tmr1IntFlag {
        &self.r8_tmr1_int_flag
    }
    #[doc = "0x07 - RO, TMR1 FIFO count status"]
    #[inline(always)]
    pub const fn r8_tmr1_fifo_count(&self) -> &R8Tmr1FifoCount {
        &self.r8_tmr1_fifo_count
    }
    #[doc = "0x08 - RO, TMR1 current count"]
    #[inline(always)]
    pub const fn r32_tmr1_count(&self) -> &R32Tmr1Count {
        &self.r32_tmr1_count
    }
    #[doc = "0x0c - RW, TMR1 end count value, only low 26 bit"]
    #[inline(always)]
    pub const fn r32_tmr1_cnt_end(&self) -> &R32Tmr1CntEnd {
        &self.r32_tmr1_cnt_end
    }
    #[doc = "0x10 - RO, TMR1 FIFO register, only low 26 bit"]
    #[inline(always)]
    pub const fn r32_tmr1_fifo(&self) -> &R32Tmr1Fifo {
        &self.r32_tmr1_fifo
    }
    #[doc = "0x14 - RO, TMR1 DMA current address"]
    #[inline(always)]
    pub const fn r16_tmr1_dma_now(&self) -> &R16Tmr1DmaNow {
        &self.r16_tmr1_dma_now
    }
    #[doc = "0x18 - RW, TMR1 DMA begin address"]
    #[inline(always)]
    pub const fn r16_tmr1_dma_beg(&self) -> &R16Tmr1DmaBeg {
        &self.r16_tmr1_dma_beg
    }
    #[doc = "0x1c - RW, TMR1 DMA end address"]
    #[inline(always)]
    pub const fn r16_tmr1_dma_end(&self) -> &R16Tmr1DmaEnd {
        &self.r16_tmr1_dma_end
    }
}
#[doc = "R8_TMR1_CTRL_MOD (rw) register accessor: RW, TMR1 mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr1_ctrl_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr1_ctrl_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr1_ctrl_mod`] module"]
#[doc(alias = "R8_TMR1_CTRL_MOD")]
pub type R8Tmr1CtrlMod = crate::Reg<r8_tmr1_ctrl_mod::R8Tmr1CtrlModSpec>;
#[doc = "RW, TMR1 mode control"]
pub mod r8_tmr1_ctrl_mod;
#[doc = "R8_TMR1_CTRL_DMA (rw) register accessor: RW, TMR1 DMA control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr1_ctrl_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr1_ctrl_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr1_ctrl_dma`] module"]
#[doc(alias = "R8_TMR1_CTRL_DMA")]
pub type R8Tmr1CtrlDma = crate::Reg<r8_tmr1_ctrl_dma::R8Tmr1CtrlDmaSpec>;
#[doc = "RW, TMR1 DMA control"]
pub mod r8_tmr1_ctrl_dma;
#[doc = "R8_TMR1_INTER_EN (rw) register accessor: RW, TMR1 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr1_inter_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr1_inter_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr1_inter_en`] module"]
#[doc(alias = "R8_TMR1_INTER_EN")]
pub type R8Tmr1InterEn = crate::Reg<r8_tmr1_inter_en::R8Tmr1InterEnSpec>;
#[doc = "RW, TMR1 interrupt enable"]
pub mod r8_tmr1_inter_en;
#[doc = "R8_TMR1_INT_FLAG (rw) register accessor: RW1, TMR1 interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr1_int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr1_int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr1_int_flag`] module"]
#[doc(alias = "R8_TMR1_INT_FLAG")]
pub type R8Tmr1IntFlag = crate::Reg<r8_tmr1_int_flag::R8Tmr1IntFlagSpec>;
#[doc = "RW1, TMR1 interrupt flag"]
pub mod r8_tmr1_int_flag;
#[doc = "R8_TMR1_FIFO_COUNT (r) register accessor: RO, TMR1 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr1_fifo_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr1_fifo_count`] module"]
#[doc(alias = "R8_TMR1_FIFO_COUNT")]
pub type R8Tmr1FifoCount = crate::Reg<r8_tmr1_fifo_count::R8Tmr1FifoCountSpec>;
#[doc = "RO, TMR1 FIFO count status"]
pub mod r8_tmr1_fifo_count;
#[doc = "R32_TMR1_COUNT (r) register accessor: RO, TMR1 current count\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr1_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr1_count`] module"]
#[doc(alias = "R32_TMR1_COUNT")]
pub type R32Tmr1Count = crate::Reg<r32_tmr1_count::R32Tmr1CountSpec>;
#[doc = "RO, TMR1 current count"]
pub mod r32_tmr1_count;
#[doc = "R32_TMR1_CNT_END (rw) register accessor: RW, TMR1 end count value, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr1_cnt_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr1_cnt_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr1_cnt_end`] module"]
#[doc(alias = "R32_TMR1_CNT_END")]
pub type R32Tmr1CntEnd = crate::Reg<r32_tmr1_cnt_end::R32Tmr1CntEndSpec>;
#[doc = "RW, TMR1 end count value, only low 26 bit"]
pub mod r32_tmr1_cnt_end;
#[doc = "R32_TMR1_FIFO (r) register accessor: RO, TMR1 FIFO register, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr1_fifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr1_fifo`] module"]
#[doc(alias = "R32_TMR1_FIFO")]
pub type R32Tmr1Fifo = crate::Reg<r32_tmr1_fifo::R32Tmr1FifoSpec>;
#[doc = "RO, TMR1 FIFO register, only low 26 bit"]
pub mod r32_tmr1_fifo;
#[doc = "R16_TMR1_DMA_NOW (r) register accessor: RO, TMR1 DMA current address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_tmr1_dma_now::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_tmr1_dma_now`] module"]
#[doc(alias = "R16_TMR1_DMA_NOW")]
pub type R16Tmr1DmaNow = crate::Reg<r16_tmr1_dma_now::R16Tmr1DmaNowSpec>;
#[doc = "RO, TMR1 DMA current address"]
pub mod r16_tmr1_dma_now;
#[doc = "R16_TMR1_DMA_BEG (rw) register accessor: RW, TMR1 DMA begin address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_tmr1_dma_beg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_tmr1_dma_beg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_tmr1_dma_beg`] module"]
#[doc(alias = "R16_TMR1_DMA_BEG")]
pub type R16Tmr1DmaBeg = crate::Reg<r16_tmr1_dma_beg::R16Tmr1DmaBegSpec>;
#[doc = "RW, TMR1 DMA begin address"]
pub mod r16_tmr1_dma_beg;
#[doc = "R16_TMR1_DMA_END (rw) register accessor: RW, TMR1 DMA end address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_tmr1_dma_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_tmr1_dma_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_tmr1_dma_end`] module"]
#[doc(alias = "R16_TMR1_DMA_END")]
pub type R16Tmr1DmaEnd = crate::Reg<r16_tmr1_dma_end::R16Tmr1DmaEndSpec>;
#[doc = "RW, TMR1 DMA end address"]
pub mod r16_tmr1_dma_end;
