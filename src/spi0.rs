#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_spi0_ctrl_mod: R8Spi0CtrlMod,
    r8_spi0_ctrl_cfg: R8Spi0CtrlCfg,
    r8_spi0_inter_en: R8Spi0InterEn,
    r8_spi0_clock_div__r8_spi0_slave_pre: R8Spi0ClockDiv_R8Spi0SlavePre,
    r8_spi0_buffer: R8Spi0Buffer,
    r8_spi0_run_flag: R8Spi0RunFlag,
    r8_spi0_int_flag: R8Spi0IntFlag,
    r8_spi0_fifo_count: R8Spi0FifoCount,
    _reserved8: [u8; 0x04],
    r16_spi0_total_cnt: R16Spi0TotalCnt,
    _reserved9: [u8; 0x02],
    r8_spi0_fifo: R8Spi0Fifo,
    _reserved10: [u8; 0x02],
    r8_spi0_fifo_count1: R8Spi0FifoCount1,
    r16_spi0_dma_now: R16Spi0DmaNow,
    _reserved12: [u8; 0x02],
    r16_spi0_dma_beg: R16Spi0DmaBeg,
    _reserved13: [u8; 0x02],
    r16_spi0_dma_end: R16Spi0DmaEnd,
}
impl RegisterBlock {
    #[doc = "0x00 - RW, SPI0 mode control"]
    #[inline(always)]
    pub const fn r8_spi0_ctrl_mod(&self) -> &R8Spi0CtrlMod {
        &self.r8_spi0_ctrl_mod
    }
    #[doc = "0x01 - RW, SPI0 configuration control"]
    #[inline(always)]
    pub const fn r8_spi0_ctrl_cfg(&self) -> &R8Spi0CtrlCfg {
        &self.r8_spi0_ctrl_cfg
    }
    #[doc = "0x02 - RW, SPI0 interrupt enable"]
    #[inline(always)]
    pub const fn r8_spi0_inter_en(&self) -> &R8Spi0InterEn {
        &self.r8_spi0_inter_en
    }
    #[doc = "0x03 - RW, SPI0 master clock divisor;RW, SPI0 slave preset value"]
    #[inline(always)]
    pub const fn r8_spi0_clock_div__r8_spi0_slave_pre(&self) -> &R8Spi0ClockDiv_R8Spi0SlavePre {
        &self.r8_spi0_clock_div__r8_spi0_slave_pre
    }
    #[doc = "0x04 - RW, SPI0 data buffer"]
    #[inline(always)]
    pub const fn r8_spi0_buffer(&self) -> &R8Spi0Buffer {
        &self.r8_spi0_buffer
    }
    #[doc = "0x05 - RO, SPI0 work flag"]
    #[inline(always)]
    pub const fn r8_spi0_run_flag(&self) -> &R8Spi0RunFlag {
        &self.r8_spi0_run_flag
    }
    #[doc = "0x06 - RW1, SPI0 interrupt flag"]
    #[inline(always)]
    pub const fn r8_spi0_int_flag(&self) -> &R8Spi0IntFlag {
        &self.r8_spi0_int_flag
    }
    #[doc = "0x07 - RO, SPI0 FIFO count status"]
    #[inline(always)]
    pub const fn r8_spi0_fifo_count(&self) -> &R8Spi0FifoCount {
        &self.r8_spi0_fifo_count
    }
    #[doc = "0x0c - RW, SPI0 total byte count, only low 12 bit"]
    #[inline(always)]
    pub const fn r16_spi0_total_cnt(&self) -> &R16Spi0TotalCnt {
        &self.r16_spi0_total_cnt
    }
    #[doc = "0x10 - RO/WO, SPI0 FIFO register"]
    #[inline(always)]
    pub const fn r8_spi0_fifo(&self) -> &R8Spi0Fifo {
        &self.r8_spi0_fifo
    }
    #[doc = "0x13 - RO, SPI0 FIFO count status"]
    #[inline(always)]
    pub const fn r8_spi0_fifo_count1(&self) -> &R8Spi0FifoCount1 {
        &self.r8_spi0_fifo_count1
    }
    #[doc = "0x14 - RW, SPI0 DMA current address"]
    #[inline(always)]
    pub const fn r16_spi0_dma_now(&self) -> &R16Spi0DmaNow {
        &self.r16_spi0_dma_now
    }
    #[doc = "0x18 - RW, SPI0 DMA begin address"]
    #[inline(always)]
    pub const fn r16_spi0_dma_beg(&self) -> &R16Spi0DmaBeg {
        &self.r16_spi0_dma_beg
    }
    #[doc = "0x1c - RW, SPI0 DMA end address"]
    #[inline(always)]
    pub const fn r16_spi0_dma_end(&self) -> &R16Spi0DmaEnd {
        &self.r16_spi0_dma_end
    }
}
#[doc = "R8_SPI0_CTRL_MOD (rw) register accessor: RW, SPI0 mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_ctrl_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_ctrl_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_ctrl_mod`] module"]
#[doc(alias = "R8_SPI0_CTRL_MOD")]
pub type R8Spi0CtrlMod = crate::Reg<r8_spi0_ctrl_mod::R8Spi0CtrlModSpec>;
#[doc = "RW, SPI0 mode control"]
pub mod r8_spi0_ctrl_mod;
#[doc = "R8_SPI0_CTRL_CFG (rw) register accessor: RW, SPI0 configuration control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_ctrl_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_ctrl_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_ctrl_cfg`] module"]
#[doc(alias = "R8_SPI0_CTRL_CFG")]
pub type R8Spi0CtrlCfg = crate::Reg<r8_spi0_ctrl_cfg::R8Spi0CtrlCfgSpec>;
#[doc = "RW, SPI0 configuration control"]
pub mod r8_spi0_ctrl_cfg;
#[doc = "R8_SPI0_INTER_EN (rw) register accessor: RW, SPI0 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_inter_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_inter_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_inter_en`] module"]
#[doc(alias = "R8_SPI0_INTER_EN")]
pub type R8Spi0InterEn = crate::Reg<r8_spi0_inter_en::R8Spi0InterEnSpec>;
#[doc = "RW, SPI0 interrupt enable"]
pub mod r8_spi0_inter_en;
#[doc = "R8_SPI0_CLOCK_DIV__R8_SPI0_SLAVE_PRE (rw) register accessor: RW, SPI0 master clock divisor;RW, SPI0 slave preset value\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_clock_div__r8_spi0_slave_pre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_clock_div__r8_spi0_slave_pre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_clock_div__r8_spi0_slave_pre`] module"]
#[doc(alias = "R8_SPI0_CLOCK_DIV__R8_SPI0_SLAVE_PRE")]
pub type R8Spi0ClockDiv_R8Spi0SlavePre =
    crate::Reg<r8_spi0_clock_div__r8_spi0_slave_pre::R8Spi0ClockDiv_R8Spi0SlavePreSpec>;
#[doc = "RW, SPI0 master clock divisor;RW, SPI0 slave preset value"]
pub mod r8_spi0_clock_div__r8_spi0_slave_pre;
#[doc = "R8_SPI0_BUFFER (rw) register accessor: RW, SPI0 data buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_buffer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_buffer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_buffer`] module"]
#[doc(alias = "R8_SPI0_BUFFER")]
pub type R8Spi0Buffer = crate::Reg<r8_spi0_buffer::R8Spi0BufferSpec>;
#[doc = "RW, SPI0 data buffer"]
pub mod r8_spi0_buffer;
#[doc = "R8_SPI0_RUN_FLAG (r) register accessor: RO, SPI0 work flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_run_flag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_run_flag`] module"]
#[doc(alias = "R8_SPI0_RUN_FLAG")]
pub type R8Spi0RunFlag = crate::Reg<r8_spi0_run_flag::R8Spi0RunFlagSpec>;
#[doc = "RO, SPI0 work flag"]
pub mod r8_spi0_run_flag;
#[doc = "R8_SPI0_INT_FLAG (rw) register accessor: RW1, SPI0 interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_int_flag`] module"]
#[doc(alias = "R8_SPI0_INT_FLAG")]
pub type R8Spi0IntFlag = crate::Reg<r8_spi0_int_flag::R8Spi0IntFlagSpec>;
#[doc = "RW1, SPI0 interrupt flag"]
pub mod r8_spi0_int_flag;
#[doc = "R8_SPI0_FIFO_COUNT (r) register accessor: RO, SPI0 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_fifo_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_fifo_count`] module"]
#[doc(alias = "R8_SPI0_FIFO_COUNT")]
pub type R8Spi0FifoCount = crate::Reg<r8_spi0_fifo_count::R8Spi0FifoCountSpec>;
#[doc = "RO, SPI0 FIFO count status"]
pub mod r8_spi0_fifo_count;
#[doc = "R16_SPI0_TOTAL_CNT (rw) register accessor: RW, SPI0 total byte count, only low 12 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_spi0_total_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_spi0_total_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_spi0_total_cnt`] module"]
#[doc(alias = "R16_SPI0_TOTAL_CNT")]
pub type R16Spi0TotalCnt = crate::Reg<r16_spi0_total_cnt::R16Spi0TotalCntSpec>;
#[doc = "RW, SPI0 total byte count, only low 12 bit"]
pub mod r16_spi0_total_cnt;
#[doc = "R8_SPI0_FIFO (rw) register accessor: RO/WO, SPI0 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_fifo`] module"]
#[doc(alias = "R8_SPI0_FIFO")]
pub type R8Spi0Fifo = crate::Reg<r8_spi0_fifo::R8Spi0FifoSpec>;
#[doc = "RO/WO, SPI0 FIFO register"]
pub mod r8_spi0_fifo;
#[doc = "R8_SPI0_FIFO_COUNT1 (r) register accessor: RO, SPI0 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_fifo_count1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_fifo_count1`] module"]
#[doc(alias = "R8_SPI0_FIFO_COUNT1")]
pub type R8Spi0FifoCount1 = crate::Reg<r8_spi0_fifo_count1::R8Spi0FifoCount1Spec>;
#[doc = "RO, SPI0 FIFO count status"]
pub mod r8_spi0_fifo_count1;
#[doc = "R16_SPI0_DMA_NOW (rw) register accessor: RW, SPI0 DMA current address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_spi0_dma_now::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_spi0_dma_now::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_spi0_dma_now`] module"]
#[doc(alias = "R16_SPI0_DMA_NOW")]
pub type R16Spi0DmaNow = crate::Reg<r16_spi0_dma_now::R16Spi0DmaNowSpec>;
#[doc = "RW, SPI0 DMA current address"]
pub mod r16_spi0_dma_now;
#[doc = "R16_SPI0_DMA_BEG (rw) register accessor: RW, SPI0 DMA begin address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_spi0_dma_beg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_spi0_dma_beg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_spi0_dma_beg`] module"]
#[doc(alias = "R16_SPI0_DMA_BEG")]
pub type R16Spi0DmaBeg = crate::Reg<r16_spi0_dma_beg::R16Spi0DmaBegSpec>;
#[doc = "RW, SPI0 DMA begin address"]
pub mod r16_spi0_dma_beg;
#[doc = "R16_SPI0_DMA_END (rw) register accessor: RW, SPI0 DMA end address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_spi0_dma_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_spi0_dma_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_spi0_dma_end`] module"]
#[doc(alias = "R16_SPI0_DMA_END")]
pub type R16Spi0DmaEnd = crate::Reg<r16_spi0_dma_end::R16Spi0DmaEndSpec>;
#[doc = "RW, SPI0 DMA end address"]
pub mod r16_spi0_dma_end;
