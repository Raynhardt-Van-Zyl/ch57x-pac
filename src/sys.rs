#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    r8_reset_status__r8_glob_rom_cfg: R8ResetStatus_R8GlobRomCfg,
    _reserved1: [u8; 0x03],
    r16_clk_sys_cfg: R16ClkSysCfg,
    r8_hfck_pwr_ctrl: R8HfckPwrCtrl,
    _reserved3: [u8; 0x01],
    r8_slp_clk_off0: R8SlpClkOff0,
    r8_slp_clk_off1: R8SlpClkOff1,
    r8_slp_wake_ctrl: R8SlpWakeCtrl,
    r8_slp_power_ctrl: R8SlpPowerCtrl,
    _reserved7: [u8; 0x08],
    r16_pin_alternate: R16PinAlternate,
    r16_pin_analog_ie: R16PinAnalogIe,
    _reserved9: [u8; 0x04],
    r16_power_plan: R16PowerPlan,
    r8_aux_power_adj: R8AuxPowerAdj,
    _reserved11: [u8; 0x01],
    r8_bat_det_ctrl: R8BatDetCtrl,
    r8_bat_det_cfg: R8BatDetCfg,
    r8_bat_status: R8BatStatus,
    _reserved14: [u8; 0x05],
    r16_int32k_tune: R16Int32kTune,
    r8_xt32k_tune: R8Xt32kTune,
    r8_ck32k_config: R8Ck32kConfig,
    r8_rtc_flag_ctrl: R8RtcFlagCtrl,
    r8_rtc_mode_ctrl: R8RtcModeCtrl,
    _reserved19: [u8; 0x02],
    r32_rtc_trig: R32RtcTrig,
    r16_rtc_cnt_32k: R16RtcCnt32k,
    r16_rtc_cnt_2s: R16RtcCnt2s,
    r32_rtc_cnt_day: R32RtcCntDay,
    r8_safe_access_sig: R8SafeAccessSig,
    r8_chip_id: R8ChipId,
    r8_safe_access_id: R8SafeAccessId,
    r8_wdog_count: R8WdogCount,
    _reserved27: [u8; 0x01],
    r8_glob_cfg_info: R8GlobCfgInfo,
    r8_rst_wdog_ctrl: R8RstWdogCtrl,
    r8_glob_reset_keep: R8GlobResetKeep,
    _reserved30: [u8; 0x03],
    r8_pll_config: R8PllConfig,
    _reserved31: [u8; 0x02],
    r8_xt32m_tune: R8Xt32mTune,
    _reserved32: [u8; 0x01],
    r16_osc_cal_cnt: R16OscCalCnt,
    r8_osc_cal_ctrl: R8OscCalCtrl,
    _reserved34: [u8; 0x01],
    r8_tkey_count: R8TkeyCount,
    _reserved35: [u8; 0x01],
    r8_tkey_convert: R8TkeyConvert,
    r8_tkey_cfg: R8TkeyCfg,
    r8_adc_channel: R8AdcChannel,
    r8_adc_cfg: R8AdcCfg,
    r8_adc_convert: R8AdcConvert,
    r8_tem_sensor: R8TemSensor,
    r16_adc_data: R16AdcData,
    r8_adc_int_flag: R8AdcIntFlag,
    _reserved43: [u8; 0x01],
    r32_adc_dma_ctrl: R32AdcDmaCtrl,
    r8_adc_ctrl_dma: R8AdcCtrlDma,
    r8_adc_dma_if: R8AdcDmaIf,
    r8_adc_auto_cycle: R8AdcAutoCycle,
    r16_adc_dma_now: R16AdcDmaNow,
    _reserved48: [u8; 0x02],
    r16_adc_dma_beg: R16AdcDmaBeg,
    _reserved49: [u8; 0x02],
    r16_adc_dma_end: R16AdcDmaEnd,
    _reserved50: [u8; 0x22],
    r16_pa_int_en: R16PaIntEn,
    r16_pb_int_en: R16PbIntEn,
    r16_pa_int_mode: R16PaIntMode,
    r16_pb_int_mode: R16PbIntMode,
    _reserved54: [u8; 0x04],
    r16_pa_int_if: R16PaIntIf,
    r16_pb_int_if: R16PbIntIf,
    r32_pa_dir: R32PaDir,
    r32_pa_pin: R32PaPin,
    r32_pa_out: R32PaOut,
    r32_pa_clr: R32PaClr,
    r32_pa_pu: R32PaPu,
    r32_pa_pd_drv: R32PaPdDrv,
    _reserved62: [u8; 0x08],
    r32_pb_dir: R32PbDir,
    r32_pb_pin: R32PbPin,
    r32_pb_out__r8_slv_rd_data: R32PbOut_R8SlvRdData,
    r32_pb_clr: R32PbClr,
    r32_pb_pu: R32PbPu,
    r32_pb_pd_drv: R32PbPdDrv,
}
impl RegisterBlock {
    #[doc = "0x04 - RWA, reset status, SAM or flash ROM configuration"]
    #[inline(always)]
    pub const fn r8_reset_status__r8_glob_rom_cfg(&self) -> &R8ResetStatus_R8GlobRomCfg {
        &self.r8_reset_status__r8_glob_rom_cfg
    }
    #[doc = "0x08 - RWA, system clock configuration, SAM"]
    #[inline(always)]
    pub const fn r16_clk_sys_cfg(&self) -> &R16ClkSysCfg {
        &self.r16_clk_sys_cfg
    }
    #[doc = "0x0a - RWA, high frequency clock module power control, SAM"]
    #[inline(always)]
    pub const fn r8_hfck_pwr_ctrl(&self) -> &R8HfckPwrCtrl {
        &self.r8_hfck_pwr_ctrl
    }
    #[doc = "0x0c - RWA, sleep clock off control byte 0, SAM"]
    #[inline(always)]
    pub const fn r8_slp_clk_off0(&self) -> &R8SlpClkOff0 {
        &self.r8_slp_clk_off0
    }
    #[doc = "0x0d - RWA, sleep clock off control byte 1, SAM"]
    #[inline(always)]
    pub const fn r8_slp_clk_off1(&self) -> &R8SlpClkOff1 {
        &self.r8_slp_clk_off1
    }
    #[doc = "0x0e - RWA, wake control, SAM"]
    #[inline(always)]
    pub const fn r8_slp_wake_ctrl(&self) -> &R8SlpWakeCtrl {
        &self.r8_slp_wake_ctrl
    }
    #[doc = "0x0f - RWA, peripherals power down control, SAM"]
    #[inline(always)]
    pub const fn r8_slp_power_ctrl(&self) -> &R8SlpPowerCtrl {
        &self.r8_slp_power_ctrl
    }
    #[doc = "0x18 - RW, function pin alternate configuration"]
    #[inline(always)]
    pub const fn r16_pin_alternate(&self) -> &R16PinAlternate {
        &self.r16_pin_alternate
    }
    #[doc = "0x1a - RW, analog pin enable and digital input disable"]
    #[inline(always)]
    pub const fn r16_pin_analog_ie(&self) -> &R16PinAnalogIe {
        &self.r16_pin_analog_ie
    }
    #[doc = "0x20 - RWA, power plan before sleep instruction, SAM"]
    #[inline(always)]
    pub const fn r16_power_plan(&self) -> &R16PowerPlan {
        &self.r16_power_plan
    }
    #[doc = "0x22 - RWA, aux power adjust control, SAM"]
    #[inline(always)]
    pub const fn r8_aux_power_adj(&self) -> &R8AuxPowerAdj {
        &self.r8_aux_power_adj
    }
    #[doc = "0x24 - RWA, battery voltage detector control, SAM"]
    #[inline(always)]
    pub const fn r8_bat_det_ctrl(&self) -> &R8BatDetCtrl {
        &self.r8_bat_det_ctrl
    }
    #[doc = "0x25 - RWA, battery voltage detector configuration, SAM"]
    #[inline(always)]
    pub const fn r8_bat_det_cfg(&self) -> &R8BatDetCfg {
        &self.r8_bat_det_cfg
    }
    #[doc = "0x26 - RO, battery status"]
    #[inline(always)]
    pub const fn r8_bat_status(&self) -> &R8BatStatus {
        &self.r8_bat_status
    }
    #[doc = "0x2c - RWA, internal 32KHz oscillator tune control, SAM"]
    #[inline(always)]
    pub const fn r16_int32k_tune(&self) -> &R16Int32kTune {
        &self.r16_int32k_tune
    }
    #[doc = "0x2e - RWA, external 32KHz oscillator tune control, SAM"]
    #[inline(always)]
    pub const fn r8_xt32k_tune(&self) -> &R8Xt32kTune {
        &self.r8_xt32k_tune
    }
    #[doc = "0x2f - RWA, 32KHz oscillator configure"]
    #[inline(always)]
    pub const fn r8_ck32k_config(&self) -> &R8Ck32kConfig {
        &self.r8_ck32k_config
    }
    #[doc = "0x30 - RW, RTC flag and clear control"]
    #[inline(always)]
    pub const fn r8_rtc_flag_ctrl(&self) -> &R8RtcFlagCtrl {
        &self.r8_rtc_flag_ctrl
    }
    #[doc = "0x31 - RWA, RTC mode control, SAM"]
    #[inline(always)]
    pub const fn r8_rtc_mode_ctrl(&self) -> &R8RtcModeCtrl {
        &self.r8_rtc_mode_ctrl
    }
    #[doc = "0x34 - RWA, RTC trigger value, SAM"]
    #[inline(always)]
    pub const fn r32_rtc_trig(&self) -> &R32RtcTrig {
        &self.r32_rtc_trig
    }
    #[doc = "0x38 - RO, RTC count based 32KHz"]
    #[inline(always)]
    pub const fn r16_rtc_cnt_32k(&self) -> &R16RtcCnt32k {
        &self.r16_rtc_cnt_32k
    }
    #[doc = "0x3a - RO, RTC count based 2 second"]
    #[inline(always)]
    pub const fn r16_rtc_cnt_2s(&self) -> &R16RtcCnt2s {
        &self.r16_rtc_cnt_2s
    }
    #[doc = "0x3c - RO, RTC count based one day, only low 14 bit"]
    #[inline(always)]
    pub const fn r32_rtc_cnt_day(&self) -> &R32RtcCntDay {
        &self.r32_rtc_cnt_day
    }
    #[doc = "0x40 - WO, safe accessing sign register, must write SAFE_ACCESS_SIG1 then SAFE_ACCESS_SIG2 to enter safe accessing mode"]
    #[inline(always)]
    pub const fn r8_safe_access_sig(&self) -> &R8SafeAccessSig {
        &self.r8_safe_access_sig
    }
    #[doc = "0x41 - RF, chip ID register, always is ID_CH57*"]
    #[inline(always)]
    pub const fn r8_chip_id(&self) -> &R8ChipId {
        &self.r8_chip_id
    }
    #[doc = "0x42 - RF, safe accessing ID register, always 0x04"]
    #[inline(always)]
    pub const fn r8_safe_access_id(&self) -> &R8SafeAccessId {
        &self.r8_safe_access_id
    }
    #[doc = "0x43 - RW, watch-dog count, count by clock frequency Fsys/131072"]
    #[inline(always)]
    pub const fn r8_wdog_count(&self) -> &R8WdogCount {
        &self.r8_wdog_count
    }
    #[doc = "0x45 - RO, global configuration information and status"]
    #[inline(always)]
    pub const fn r8_glob_cfg_info(&self) -> &R8GlobCfgInfo {
        &self.r8_glob_cfg_info
    }
    #[doc = "0x46 - RWA, reset and watch-dog control, SAM"]
    #[inline(always)]
    pub const fn r8_rst_wdog_ctrl(&self) -> &R8RstWdogCtrl {
        &self.r8_rst_wdog_ctrl
    }
    #[doc = "0x47 - RW, value keeper during global reset"]
    #[inline(always)]
    pub const fn r8_glob_reset_keep(&self) -> &R8GlobResetKeep {
        &self.r8_glob_reset_keep
    }
    #[doc = "0x4b - RWA, PLL configuration control, SAM"]
    #[inline(always)]
    pub const fn r8_pll_config(&self) -> &R8PllConfig {
        &self.r8_pll_config
    }
    #[doc = "0x4e - RWA, external 32MHz oscillator tune control, SAM"]
    #[inline(always)]
    pub const fn r8_xt32m_tune(&self) -> &R8Xt32mTune {
        &self.r8_xt32m_tune
    }
    #[doc = "0x50 - RO, system clock count value for 32KHz 5 cycles"]
    #[inline(always)]
    pub const fn r16_osc_cal_cnt(&self) -> &R16OscCalCnt {
        &self.r16_osc_cal_cnt
    }
    #[doc = "0x52 - RWA, oscillator frequency calibration control, SAM"]
    #[inline(always)]
    pub const fn r8_osc_cal_ctrl(&self) -> &R8OscCalCtrl {
        &self.r8_osc_cal_ctrl
    }
    #[doc = "0x54 - RW, Touchkey charge and discharge count"]
    #[inline(always)]
    pub const fn r8_tkey_count(&self) -> &R8TkeyCount {
        &self.r8_tkey_count
    }
    #[doc = "0x56 - RW, Touchkey convert start control"]
    #[inline(always)]
    pub const fn r8_tkey_convert(&self) -> &R8TkeyConvert {
        &self.r8_tkey_convert
    }
    #[doc = "0x57 - RW, Touchkey configure"]
    #[inline(always)]
    pub const fn r8_tkey_cfg(&self) -> &R8TkeyCfg {
        &self.r8_tkey_cfg
    }
    #[doc = "0x58 - RW, ADC input channel selection"]
    #[inline(always)]
    pub const fn r8_adc_channel(&self) -> &R8AdcChannel {
        &self.r8_adc_channel
    }
    #[doc = "0x59 - RW, ADC configure"]
    #[inline(always)]
    pub const fn r8_adc_cfg(&self) -> &R8AdcCfg {
        &self.r8_adc_cfg
    }
    #[doc = "0x5a - RW, ADC convert control"]
    #[inline(always)]
    pub const fn r8_adc_convert(&self) -> &R8AdcConvert {
        &self.r8_adc_convert
    }
    #[doc = "0x5b - RW, temperature sensor control"]
    #[inline(always)]
    pub const fn r8_tem_sensor(&self) -> &R8TemSensor {
        &self.r8_tem_sensor
    }
    #[doc = "0x5c - RO, ADC data"]
    #[inline(always)]
    pub const fn r16_adc_data(&self) -> &R16AdcData {
        &self.r16_adc_data
    }
    #[doc = "0x5e - RO, ADC interrupt flag register"]
    #[inline(always)]
    pub const fn r8_adc_int_flag(&self) -> &R8AdcIntFlag {
        &self.r8_adc_int_flag
    }
    #[doc = "0x60 - RO, ADC DMA control and status register"]
    #[inline(always)]
    pub const fn r32_adc_dma_ctrl(&self) -> &R32AdcDmaCtrl {
        &self.r32_adc_dma_ctrl
    }
    #[doc = "0x61 - RW, ADC DMA control"]
    #[inline(always)]
    pub const fn r8_adc_ctrl_dma(&self) -> &R8AdcCtrlDma {
        &self.r8_adc_ctrl_dma
    }
    #[doc = "0x62 - RO, ADC interrupt flag"]
    #[inline(always)]
    pub const fn r8_adc_dma_if(&self) -> &R8AdcDmaIf {
        &self.r8_adc_dma_if
    }
    #[doc = "0x63 - RO, ADC interrupt flag"]
    #[inline(always)]
    pub const fn r8_adc_auto_cycle(&self) -> &R8AdcAutoCycle {
        &self.r8_adc_auto_cycle
    }
    #[doc = "0x64 - RO, ADC DMA current address"]
    #[inline(always)]
    pub const fn r16_adc_dma_now(&self) -> &R16AdcDmaNow {
        &self.r16_adc_dma_now
    }
    #[doc = "0x68 - RW, ADC DMA begin address"]
    #[inline(always)]
    pub const fn r16_adc_dma_beg(&self) -> &R16AdcDmaBeg {
        &self.r16_adc_dma_beg
    }
    #[doc = "0x6c - RW, ADC DMA end address"]
    #[inline(always)]
    pub const fn r16_adc_dma_end(&self) -> &R16AdcDmaEnd {
        &self.r16_adc_dma_end
    }
    #[doc = "0x90 - RW, GPIO PA interrupt enable"]
    #[inline(always)]
    pub const fn r16_pa_int_en(&self) -> &R16PaIntEn {
        &self.r16_pa_int_en
    }
    #[doc = "0x92 - RW, GPIO PB interrupt enable"]
    #[inline(always)]
    pub const fn r16_pb_int_en(&self) -> &R16PbIntEn {
        &self.r16_pb_int_en
    }
    #[doc = "0x94 - RW, GPIO PA interrupt mode: 0=level action, 1=edge action"]
    #[inline(always)]
    pub const fn r16_pa_int_mode(&self) -> &R16PaIntMode {
        &self.r16_pa_int_mode
    }
    #[doc = "0x96 - RW, GPIO PB interrupt mode: 0=level action, 1=edge action;RW, status for parallel slave read"]
    #[inline(always)]
    pub const fn r16_pb_int_mode(&self) -> &R16PbIntMode {
        &self.r16_pb_int_mode
    }
    #[doc = "0x9c - RW1, GPIO PA interrupt flag"]
    #[inline(always)]
    pub const fn r16_pa_int_if(&self) -> &R16PaIntIf {
        &self.r16_pa_int_if
    }
    #[doc = "0x9e - RW1, GPIO PB interrupt flag"]
    #[inline(always)]
    pub const fn r16_pb_int_if(&self) -> &R16PbIntIf {
        &self.r16_pb_int_if
    }
    #[doc = "0xa0 - RW, GPIO PA I/O direction: 0=in, 1=out"]
    #[inline(always)]
    pub const fn r32_pa_dir(&self) -> &R32PaDir {
        &self.r32_pa_dir
    }
    #[doc = "0xa4 - RO, GPIO PA input"]
    #[inline(always)]
    pub const fn r32_pa_pin(&self) -> &R32PaPin {
        &self.r32_pa_pin
    }
    #[doc = "0xa8 - RW, GPIO PA output"]
    #[inline(always)]
    pub const fn r32_pa_out(&self) -> &R32PaOut {
        &self.r32_pa_out
    }
    #[doc = "0xac - WZ, GPIO PA clear output: 0=keep, 1=clear"]
    #[inline(always)]
    pub const fn r32_pa_clr(&self) -> &R32PaClr {
        &self.r32_pa_clr
    }
    #[doc = "0xb0 - RW, GPIO PA pullup resistance enable"]
    #[inline(always)]
    pub const fn r32_pa_pu(&self) -> &R32PaPu {
        &self.r32_pa_pu
    }
    #[doc = "0xb4 - RW, PA pulldown for input or PA driving capability for output"]
    #[inline(always)]
    pub const fn r32_pa_pd_drv(&self) -> &R32PaPdDrv {
        &self.r32_pa_pd_drv
    }
    #[doc = "0xc0 - RW, GPIO PB I/O direction: 0=in, 1=out"]
    #[inline(always)]
    pub const fn r32_pb_dir(&self) -> &R32PbDir {
        &self.r32_pb_dir
    }
    #[doc = "0xc4 - RO, GPIO PB input"]
    #[inline(always)]
    pub const fn r32_pb_pin(&self) -> &R32PbPin {
        &self.r32_pb_pin
    }
    #[doc = "0xc8 - RW, GPIO PB output;RW, data for parallel slave read"]
    #[inline(always)]
    pub const fn r32_pb_out__r8_slv_rd_data(&self) -> &R32PbOut_R8SlvRdData {
        &self.r32_pb_out__r8_slv_rd_data
    }
    #[doc = "0xcc - WZ, GPIO PB clear output: 0=keep, 1=clear"]
    #[inline(always)]
    pub const fn r32_pb_clr(&self) -> &R32PbClr {
        &self.r32_pb_clr
    }
    #[doc = "0xd0 - RW, GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub const fn r32_pb_pu(&self) -> &R32PbPu {
        &self.r32_pb_pu
    }
    #[doc = "0xd4 - RW, PB pulldown for input or PB driving capability for output"]
    #[inline(always)]
    pub const fn r32_pb_pd_drv(&self) -> &R32PbPdDrv {
        &self.r32_pb_pd_drv
    }
}
#[doc = "R16_CLK_SYS_CFG (rw) register accessor: RWA, system clock configuration, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_clk_sys_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_clk_sys_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_clk_sys_cfg`] module"]
#[doc(alias = "R16_CLK_SYS_CFG")]
pub type R16ClkSysCfg = crate::Reg<r16_clk_sys_cfg::R16ClkSysCfgSpec>;
#[doc = "RWA, system clock configuration, SAM"]
pub mod r16_clk_sys_cfg;
#[doc = "R8_HFCK_PWR_CTRL (rw) register accessor: RWA, high frequency clock module power control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hfck_pwr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hfck_pwr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_hfck_pwr_ctrl`] module"]
#[doc(alias = "R8_HFCK_PWR_CTRL")]
pub type R8HfckPwrCtrl = crate::Reg<r8_hfck_pwr_ctrl::R8HfckPwrCtrlSpec>;
#[doc = "RWA, high frequency clock module power control, SAM"]
pub mod r8_hfck_pwr_ctrl;
#[doc = "R8_SLP_CLK_OFF0 (rw) register accessor: RWA, sleep clock off control byte 0, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_clk_off0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_clk_off0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_slp_clk_off0`] module"]
#[doc(alias = "R8_SLP_CLK_OFF0")]
pub type R8SlpClkOff0 = crate::Reg<r8_slp_clk_off0::R8SlpClkOff0Spec>;
#[doc = "RWA, sleep clock off control byte 0, SAM"]
pub mod r8_slp_clk_off0;
#[doc = "R8_SLP_CLK_OFF1 (rw) register accessor: RWA, sleep clock off control byte 1, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_clk_off1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_clk_off1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_slp_clk_off1`] module"]
#[doc(alias = "R8_SLP_CLK_OFF1")]
pub type R8SlpClkOff1 = crate::Reg<r8_slp_clk_off1::R8SlpClkOff1Spec>;
#[doc = "RWA, sleep clock off control byte 1, SAM"]
pub mod r8_slp_clk_off1;
#[doc = "R8_SLP_WAKE_CTRL (rw) register accessor: RWA, wake control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_wake_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_wake_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_slp_wake_ctrl`] module"]
#[doc(alias = "R8_SLP_WAKE_CTRL")]
pub type R8SlpWakeCtrl = crate::Reg<r8_slp_wake_ctrl::R8SlpWakeCtrlSpec>;
#[doc = "RWA, wake control, SAM"]
pub mod r8_slp_wake_ctrl;
#[doc = "R8_SLP_POWER_CTRL (rw) register accessor: RWA, peripherals power down control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_slp_power_ctrl`] module"]
#[doc(alias = "R8_SLP_POWER_CTRL")]
pub type R8SlpPowerCtrl = crate::Reg<r8_slp_power_ctrl::R8SlpPowerCtrlSpec>;
#[doc = "RWA, peripherals power down control, SAM"]
pub mod r8_slp_power_ctrl;
#[doc = "R16_PIN_ALTERNATE (rw) register accessor: RW, function pin alternate configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pin_alternate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pin_alternate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_pin_alternate`] module"]
#[doc(alias = "R16_PIN_ALTERNATE")]
pub type R16PinAlternate = crate::Reg<r16_pin_alternate::R16PinAlternateSpec>;
#[doc = "RW, function pin alternate configuration"]
pub mod r16_pin_alternate;
#[doc = "R16_PIN_ANALOG_IE (rw) register accessor: RW, analog pin enable and digital input disable\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pin_analog_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pin_analog_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_pin_analog_ie`] module"]
#[doc(alias = "R16_PIN_ANALOG_IE")]
pub type R16PinAnalogIe = crate::Reg<r16_pin_analog_ie::R16PinAnalogIeSpec>;
#[doc = "RW, analog pin enable and digital input disable"]
pub mod r16_pin_analog_ie;
#[doc = "R16_POWER_PLAN (rw) register accessor: RWA, power plan before sleep instruction, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_power_plan::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_power_plan::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_power_plan`] module"]
#[doc(alias = "R16_POWER_PLAN")]
pub type R16PowerPlan = crate::Reg<r16_power_plan::R16PowerPlanSpec>;
#[doc = "RWA, power plan before sleep instruction, SAM"]
pub mod r16_power_plan;
#[doc = "R8_AUX_POWER_ADJ (rw) register accessor: RWA, aux power adjust control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_aux_power_adj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_aux_power_adj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_aux_power_adj`] module"]
#[doc(alias = "R8_AUX_POWER_ADJ")]
pub type R8AuxPowerAdj = crate::Reg<r8_aux_power_adj::R8AuxPowerAdjSpec>;
#[doc = "RWA, aux power adjust control, SAM"]
pub mod r8_aux_power_adj;
#[doc = "R8_BAT_DET_CTRL (rw) register accessor: RWA, battery voltage detector control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_bat_det_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_bat_det_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_bat_det_ctrl`] module"]
#[doc(alias = "R8_BAT_DET_CTRL")]
pub type R8BatDetCtrl = crate::Reg<r8_bat_det_ctrl::R8BatDetCtrlSpec>;
#[doc = "RWA, battery voltage detector control, SAM"]
pub mod r8_bat_det_ctrl;
#[doc = "R8_BAT_DET_CFG (rw) register accessor: RWA, battery voltage detector configuration, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_bat_det_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_bat_det_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_bat_det_cfg`] module"]
#[doc(alias = "R8_BAT_DET_CFG")]
pub type R8BatDetCfg = crate::Reg<r8_bat_det_cfg::R8BatDetCfgSpec>;
#[doc = "RWA, battery voltage detector configuration, SAM"]
pub mod r8_bat_det_cfg;
#[doc = "R8_BAT_STATUS (r) register accessor: RO, battery status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_bat_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_bat_status`] module"]
#[doc(alias = "R8_BAT_STATUS")]
pub type R8BatStatus = crate::Reg<r8_bat_status::R8BatStatusSpec>;
#[doc = "RO, battery status"]
pub mod r8_bat_status;
#[doc = "R16_INT32K_TUNE (rw) register accessor: RWA, internal 32KHz oscillator tune control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_int32k_tune::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_int32k_tune::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_int32k_tune`] module"]
#[doc(alias = "R16_INT32K_TUNE")]
pub type R16Int32kTune = crate::Reg<r16_int32k_tune::R16Int32kTuneSpec>;
#[doc = "RWA, internal 32KHz oscillator tune control, SAM"]
pub mod r16_int32k_tune;
#[doc = "R8_XT32K_TUNE (rw) register accessor: RWA, external 32KHz oscillator tune control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_xt32k_tune::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_xt32k_tune::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_xt32k_tune`] module"]
#[doc(alias = "R8_XT32K_TUNE")]
pub type R8Xt32kTune = crate::Reg<r8_xt32k_tune::R8Xt32kTuneSpec>;
#[doc = "RWA, external 32KHz oscillator tune control, SAM"]
pub mod r8_xt32k_tune;
#[doc = "R8_CK32K_CONFIG (rw) register accessor: RWA, 32KHz oscillator configure\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_ck32k_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_ck32k_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_ck32k_config`] module"]
#[doc(alias = "R8_CK32K_CONFIG")]
pub type R8Ck32kConfig = crate::Reg<r8_ck32k_config::R8Ck32kConfigSpec>;
#[doc = "RWA, 32KHz oscillator configure"]
pub mod r8_ck32k_config;
#[doc = "R8_RTC_FLAG_CTRL (rw) register accessor: RW, RTC flag and clear control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_rtc_flag_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_rtc_flag_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_rtc_flag_ctrl`] module"]
#[doc(alias = "R8_RTC_FLAG_CTRL")]
pub type R8RtcFlagCtrl = crate::Reg<r8_rtc_flag_ctrl::R8RtcFlagCtrlSpec>;
#[doc = "RW, RTC flag and clear control"]
pub mod r8_rtc_flag_ctrl;
#[doc = "R8_RTC_MODE_CTRL (rw) register accessor: RWA, RTC mode control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_rtc_mode_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_rtc_mode_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_rtc_mode_ctrl`] module"]
#[doc(alias = "R8_RTC_MODE_CTRL")]
pub type R8RtcModeCtrl = crate::Reg<r8_rtc_mode_ctrl::R8RtcModeCtrlSpec>;
#[doc = "RWA, RTC mode control, SAM"]
pub mod r8_rtc_mode_ctrl;
#[doc = "R32_RTC_TRIG (rw) register accessor: RWA, RTC trigger value, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_rtc_trig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_rtc_trig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_rtc_trig`] module"]
#[doc(alias = "R32_RTC_TRIG")]
pub type R32RtcTrig = crate::Reg<r32_rtc_trig::R32RtcTrigSpec>;
#[doc = "RWA, RTC trigger value, SAM"]
pub mod r32_rtc_trig;
#[doc = "R16_RTC_CNT_32K (r) register accessor: RO, RTC count based 32KHz\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_rtc_cnt_32k::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_rtc_cnt_32k`] module"]
#[doc(alias = "R16_RTC_CNT_32K")]
pub type R16RtcCnt32k = crate::Reg<r16_rtc_cnt_32k::R16RtcCnt32kSpec>;
#[doc = "RO, RTC count based 32KHz"]
pub mod r16_rtc_cnt_32k;
#[doc = "R16_RTC_CNT_2S (r) register accessor: RO, RTC count based 2 second\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_rtc_cnt_2s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_rtc_cnt_2s`] module"]
#[doc(alias = "R16_RTC_CNT_2S")]
pub type R16RtcCnt2s = crate::Reg<r16_rtc_cnt_2s::R16RtcCnt2sSpec>;
#[doc = "RO, RTC count based 2 second"]
pub mod r16_rtc_cnt_2s;
#[doc = "R32_RTC_CNT_DAY (r) register accessor: RO, RTC count based one day, only low 14 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_rtc_cnt_day::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_rtc_cnt_day`] module"]
#[doc(alias = "R32_RTC_CNT_DAY")]
pub type R32RtcCntDay = crate::Reg<r32_rtc_cnt_day::R32RtcCntDaySpec>;
#[doc = "RO, RTC count based one day, only low 14 bit"]
pub mod r32_rtc_cnt_day;
#[doc = "R8_SAFE_ACCESS_SIG (rw) register accessor: WO, safe accessing sign register, must write SAFE_ACCESS_SIG1 then SAFE_ACCESS_SIG2 to enter safe accessing mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_safe_access_sig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_safe_access_sig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_safe_access_sig`] module"]
#[doc(alias = "R8_SAFE_ACCESS_SIG")]
pub type R8SafeAccessSig = crate::Reg<r8_safe_access_sig::R8SafeAccessSigSpec>;
#[doc = "WO, safe accessing sign register, must write SAFE_ACCESS_SIG1 then SAFE_ACCESS_SIG2 to enter safe accessing mode"]
pub mod r8_safe_access_sig;
#[doc = "R8_CHIP_ID (r) register accessor: RF, chip ID register, always is ID_CH57*\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_chip_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_chip_id`] module"]
#[doc(alias = "R8_CHIP_ID")]
pub type R8ChipId = crate::Reg<r8_chip_id::R8ChipIdSpec>;
#[doc = "RF, chip ID register, always is ID_CH57*"]
pub mod r8_chip_id;
#[doc = "R8_SAFE_ACCESS_ID (r) register accessor: RF, safe accessing ID register, always 0x04\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_safe_access_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_safe_access_id`] module"]
#[doc(alias = "R8_SAFE_ACCESS_ID")]
pub type R8SafeAccessId = crate::Reg<r8_safe_access_id::R8SafeAccessIdSpec>;
#[doc = "RF, safe accessing ID register, always 0x04"]
pub mod r8_safe_access_id;
#[doc = "R8_WDOG_COUNT (rw) register accessor: RW, watch-dog count, count by clock frequency Fsys/131072\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_wdog_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_wdog_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_wdog_count`] module"]
#[doc(alias = "R8_WDOG_COUNT")]
pub type R8WdogCount = crate::Reg<r8_wdog_count::R8WdogCountSpec>;
#[doc = "RW, watch-dog count, count by clock frequency Fsys/131072"]
pub mod r8_wdog_count;
#[doc = "R8_RESET_STATUS__R8_GLOB_ROM_CFG (r) register accessor: RWA, reset status, SAM or flash ROM configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_reset_status__r8_glob_rom_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_reset_status__r8_glob_rom_cfg`] module"]
#[doc(alias = "R8_RESET_STATUS__R8_GLOB_ROM_CFG")]
pub type R8ResetStatus_R8GlobRomCfg =
    crate::Reg<r8_reset_status__r8_glob_rom_cfg::R8ResetStatus_R8GlobRomCfgSpec>;
#[doc = "RWA, reset status, SAM or flash ROM configuration"]
pub mod r8_reset_status__r8_glob_rom_cfg;
#[doc = "R8_GLOB_CFG_INFO (r) register accessor: RO, global configuration information and status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_glob_cfg_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_glob_cfg_info`] module"]
#[doc(alias = "R8_GLOB_CFG_INFO")]
pub type R8GlobCfgInfo = crate::Reg<r8_glob_cfg_info::R8GlobCfgInfoSpec>;
#[doc = "RO, global configuration information and status"]
pub mod r8_glob_cfg_info;
#[doc = "R8_RST_WDOG_CTRL (rw) register accessor: RWA, reset and watch-dog control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_rst_wdog_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_rst_wdog_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_rst_wdog_ctrl`] module"]
#[doc(alias = "R8_RST_WDOG_CTRL")]
pub type R8RstWdogCtrl = crate::Reg<r8_rst_wdog_ctrl::R8RstWdogCtrlSpec>;
#[doc = "RWA, reset and watch-dog control, SAM"]
pub mod r8_rst_wdog_ctrl;
#[doc = "R8_GLOB_RESET_KEEP (rw) register accessor: RW, value keeper during global reset\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_glob_reset_keep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_glob_reset_keep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_glob_reset_keep`] module"]
#[doc(alias = "R8_GLOB_RESET_KEEP")]
pub type R8GlobResetKeep = crate::Reg<r8_glob_reset_keep::R8GlobResetKeepSpec>;
#[doc = "RW, value keeper during global reset"]
pub mod r8_glob_reset_keep;
#[doc = "R8_PLL_CONFIG (rw) register accessor: RWA, PLL configuration control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pll_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pll_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pll_config`] module"]
#[doc(alias = "R8_PLL_CONFIG")]
pub type R8PllConfig = crate::Reg<r8_pll_config::R8PllConfigSpec>;
#[doc = "RWA, PLL configuration control, SAM"]
pub mod r8_pll_config;
#[doc = "R8_XT32M_TUNE (rw) register accessor: RWA, external 32MHz oscillator tune control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_xt32m_tune::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_xt32m_tune::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_xt32m_tune`] module"]
#[doc(alias = "R8_XT32M_TUNE")]
pub type R8Xt32mTune = crate::Reg<r8_xt32m_tune::R8Xt32mTuneSpec>;
#[doc = "RWA, external 32MHz oscillator tune control, SAM"]
pub mod r8_xt32m_tune;
#[doc = "R16_OSC_CAL_CNT (r) register accessor: RO, system clock count value for 32KHz 5 cycles\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_osc_cal_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_osc_cal_cnt`] module"]
#[doc(alias = "R16_OSC_CAL_CNT")]
pub type R16OscCalCnt = crate::Reg<r16_osc_cal_cnt::R16OscCalCntSpec>;
#[doc = "RO, system clock count value for 32KHz 5 cycles"]
pub mod r16_osc_cal_cnt;
#[doc = "R8_OSC_CAL_CTRL (rw) register accessor: RWA, oscillator frequency calibration control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_osc_cal_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_osc_cal_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_osc_cal_ctrl`] module"]
#[doc(alias = "R8_OSC_CAL_CTRL")]
pub type R8OscCalCtrl = crate::Reg<r8_osc_cal_ctrl::R8OscCalCtrlSpec>;
#[doc = "RWA, oscillator frequency calibration control, SAM"]
pub mod r8_osc_cal_ctrl;
#[doc = "R8_TKEY_COUNT (rw) register accessor: RW, Touchkey charge and discharge count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tkey_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tkey_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tkey_count`] module"]
#[doc(alias = "R8_TKEY_COUNT")]
pub type R8TkeyCount = crate::Reg<r8_tkey_count::R8TkeyCountSpec>;
#[doc = "RW, Touchkey charge and discharge count"]
pub mod r8_tkey_count;
#[doc = "R8_TKEY_CONVERT (rw) register accessor: RW, Touchkey convert start control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tkey_convert::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tkey_convert::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tkey_convert`] module"]
#[doc(alias = "R8_TKEY_CONVERT")]
pub type R8TkeyConvert = crate::Reg<r8_tkey_convert::R8TkeyConvertSpec>;
#[doc = "RW, Touchkey convert start control"]
pub mod r8_tkey_convert;
#[doc = "R8_TKEY_CFG (rw) register accessor: RW, Touchkey configure\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tkey_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tkey_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tkey_cfg`] module"]
#[doc(alias = "R8_TKEY_CFG")]
pub type R8TkeyCfg = crate::Reg<r8_tkey_cfg::R8TkeyCfgSpec>;
#[doc = "RW, Touchkey configure"]
pub mod r8_tkey_cfg;
#[doc = "R8_ADC_CHANNEL (rw) register accessor: RW, ADC input channel selection\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_channel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_channel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_adc_channel`] module"]
#[doc(alias = "R8_ADC_CHANNEL")]
pub type R8AdcChannel = crate::Reg<r8_adc_channel::R8AdcChannelSpec>;
#[doc = "RW, ADC input channel selection"]
pub mod r8_adc_channel;
#[doc = "R8_ADC_CFG (rw) register accessor: RW, ADC configure\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_adc_cfg`] module"]
#[doc(alias = "R8_ADC_CFG")]
pub type R8AdcCfg = crate::Reg<r8_adc_cfg::R8AdcCfgSpec>;
#[doc = "RW, ADC configure"]
pub mod r8_adc_cfg;
#[doc = "R8_ADC_CONVERT (rw) register accessor: RW, ADC convert control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_convert::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_convert::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_adc_convert`] module"]
#[doc(alias = "R8_ADC_CONVERT")]
pub type R8AdcConvert = crate::Reg<r8_adc_convert::R8AdcConvertSpec>;
#[doc = "RW, ADC convert control"]
pub mod r8_adc_convert;
#[doc = "R8_TEM_SENSOR (rw) register accessor: RW, temperature sensor control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tem_sensor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tem_sensor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tem_sensor`] module"]
#[doc(alias = "R8_TEM_SENSOR")]
pub type R8TemSensor = crate::Reg<r8_tem_sensor::R8TemSensorSpec>;
#[doc = "RW, temperature sensor control"]
pub mod r8_tem_sensor;
#[doc = "R16_ADC_DATA (r) register accessor: RO, ADC data\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_adc_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_adc_data`] module"]
#[doc(alias = "R16_ADC_DATA")]
pub type R16AdcData = crate::Reg<r16_adc_data::R16AdcDataSpec>;
#[doc = "RO, ADC data"]
pub mod r16_adc_data;
#[doc = "R8_ADC_INT_FLAG (r) register accessor: RO, ADC interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_int_flag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_adc_int_flag`] module"]
#[doc(alias = "R8_ADC_INT_FLAG")]
pub type R8AdcIntFlag = crate::Reg<r8_adc_int_flag::R8AdcIntFlagSpec>;
#[doc = "RO, ADC interrupt flag register"]
pub mod r8_adc_int_flag;
#[doc = "R32_ADC_DMA_CTRL (rw) register accessor: RO, ADC DMA control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_adc_dma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_adc_dma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_adc_dma_ctrl`] module"]
#[doc(alias = "R32_ADC_DMA_CTRL")]
pub type R32AdcDmaCtrl = crate::Reg<r32_adc_dma_ctrl::R32AdcDmaCtrlSpec>;
#[doc = "RO, ADC DMA control and status register"]
pub mod r32_adc_dma_ctrl;
#[doc = "R8_ADC_CTRL_DMA (rw) register accessor: RW, ADC DMA control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_ctrl_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_ctrl_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_adc_ctrl_dma`] module"]
#[doc(alias = "R8_ADC_CTRL_DMA")]
pub type R8AdcCtrlDma = crate::Reg<r8_adc_ctrl_dma::R8AdcCtrlDmaSpec>;
#[doc = "RW, ADC DMA control"]
pub mod r8_adc_ctrl_dma;
#[doc = "R8_ADC_DMA_IF (rw) register accessor: RO, ADC interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_dma_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_dma_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_adc_dma_if`] module"]
#[doc(alias = "R8_ADC_DMA_IF")]
pub type R8AdcDmaIf = crate::Reg<r8_adc_dma_if::R8AdcDmaIfSpec>;
#[doc = "RO, ADC interrupt flag"]
pub mod r8_adc_dma_if;
#[doc = "R8_ADC_AUTO_CYCLE (rw) register accessor: RO, ADC interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_auto_cycle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_auto_cycle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_adc_auto_cycle`] module"]
#[doc(alias = "R8_ADC_AUTO_CYCLE")]
pub type R8AdcAutoCycle = crate::Reg<r8_adc_auto_cycle::R8AdcAutoCycleSpec>;
#[doc = "RO, ADC interrupt flag"]
pub mod r8_adc_auto_cycle;
#[doc = "R16_ADC_DMA_NOW (r) register accessor: RO, ADC DMA current address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_adc_dma_now::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_adc_dma_now`] module"]
#[doc(alias = "R16_ADC_DMA_NOW")]
pub type R16AdcDmaNow = crate::Reg<r16_adc_dma_now::R16AdcDmaNowSpec>;
#[doc = "RO, ADC DMA current address"]
pub mod r16_adc_dma_now;
#[doc = "R16_ADC_DMA_BEG (rw) register accessor: RW, ADC DMA begin address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_adc_dma_beg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_adc_dma_beg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_adc_dma_beg`] module"]
#[doc(alias = "R16_ADC_DMA_BEG")]
pub type R16AdcDmaBeg = crate::Reg<r16_adc_dma_beg::R16AdcDmaBegSpec>;
#[doc = "RW, ADC DMA begin address"]
pub mod r16_adc_dma_beg;
#[doc = "R16_ADC_DMA_END (rw) register accessor: RW, ADC DMA end address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_adc_dma_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_adc_dma_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_adc_dma_end`] module"]
#[doc(alias = "R16_ADC_DMA_END")]
pub type R16AdcDmaEnd = crate::Reg<r16_adc_dma_end::R16AdcDmaEndSpec>;
#[doc = "RW, ADC DMA end address"]
pub mod r16_adc_dma_end;
#[doc = "R16_PA_INT_EN (rw) register accessor: RW, GPIO PA interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pa_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pa_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_pa_int_en`] module"]
#[doc(alias = "R16_PA_INT_EN")]
pub type R16PaIntEn = crate::Reg<r16_pa_int_en::R16PaIntEnSpec>;
#[doc = "RW, GPIO PA interrupt enable"]
pub mod r16_pa_int_en;
#[doc = "R16_PB_INT_EN (rw) register accessor: RW, GPIO PB interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pb_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pb_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_pb_int_en`] module"]
#[doc(alias = "R16_PB_INT_EN")]
pub type R16PbIntEn = crate::Reg<r16_pb_int_en::R16PbIntEnSpec>;
#[doc = "RW, GPIO PB interrupt enable"]
pub mod r16_pb_int_en;
#[doc = "R16_PA_INT_MODE (rw) register accessor: RW, GPIO PA interrupt mode: 0=level action, 1=edge action\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pa_int_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pa_int_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_pa_int_mode`] module"]
#[doc(alias = "R16_PA_INT_MODE")]
pub type R16PaIntMode = crate::Reg<r16_pa_int_mode::R16PaIntModeSpec>;
#[doc = "RW, GPIO PA interrupt mode: 0=level action, 1=edge action"]
pub mod r16_pa_int_mode;
#[doc = "R16_PB_INT_MODE (rw) register accessor: RW, GPIO PB interrupt mode: 0=level action, 1=edge action;RW, status for parallel slave read\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pb_int_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pb_int_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_pb_int_mode`] module"]
#[doc(alias = "R16_PB_INT_MODE")]
pub type R16PbIntMode = crate::Reg<r16_pb_int_mode::R16PbIntModeSpec>;
#[doc = "RW, GPIO PB interrupt mode: 0=level action, 1=edge action;RW, status for parallel slave read"]
pub mod r16_pb_int_mode;
#[doc = "R16_PA_INT_IF (rw) register accessor: RW1, GPIO PA interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pa_int_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pa_int_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_pa_int_if`] module"]
#[doc(alias = "R16_PA_INT_IF")]
pub type R16PaIntIf = crate::Reg<r16_pa_int_if::R16PaIntIfSpec>;
#[doc = "RW1, GPIO PA interrupt flag"]
pub mod r16_pa_int_if;
#[doc = "R16_PB_INT_IF (rw) register accessor: RW1, GPIO PB interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pb_int_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pb_int_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_pb_int_if`] module"]
#[doc(alias = "R16_PB_INT_IF")]
pub type R16PbIntIf = crate::Reg<r16_pb_int_if::R16PbIntIfSpec>;
#[doc = "RW1, GPIO PB interrupt flag"]
pub mod r16_pb_int_if;
#[doc = "R32_PA_DIR (rw) register accessor: RW, GPIO PA I/O direction: 0=in, 1=out\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_dir`] module"]
#[doc(alias = "R32_PA_DIR")]
pub type R32PaDir = crate::Reg<r32_pa_dir::R32PaDirSpec>;
#[doc = "RW, GPIO PA I/O direction: 0=in, 1=out"]
pub mod r32_pa_dir;
#[doc = "R32_PA_PIN (r) register accessor: RO, GPIO PA input\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_pin`] module"]
#[doc(alias = "R32_PA_PIN")]
pub type R32PaPin = crate::Reg<r32_pa_pin::R32PaPinSpec>;
#[doc = "RO, GPIO PA input"]
pub mod r32_pa_pin;
#[doc = "R32_PA_OUT (rw) register accessor: RW, GPIO PA output\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_out`] module"]
#[doc(alias = "R32_PA_OUT")]
pub type R32PaOut = crate::Reg<r32_pa_out::R32PaOutSpec>;
#[doc = "RW, GPIO PA output"]
pub mod r32_pa_out;
#[doc = "R32_PA_CLR (rw) register accessor: WZ, GPIO PA clear output: 0=keep, 1=clear\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_clr`] module"]
#[doc(alias = "R32_PA_CLR")]
pub type R32PaClr = crate::Reg<r32_pa_clr::R32PaClrSpec>;
#[doc = "WZ, GPIO PA clear output: 0=keep, 1=clear"]
pub mod r32_pa_clr;
#[doc = "R32_PA_PU (rw) register accessor: RW, GPIO PA pullup resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_pu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_pu`] module"]
#[doc(alias = "R32_PA_PU")]
pub type R32PaPu = crate::Reg<r32_pa_pu::R32PaPuSpec>;
#[doc = "RW, GPIO PA pullup resistance enable"]
pub mod r32_pa_pu;
#[doc = "R32_PA_PD_DRV (rw) register accessor: RW, PA pulldown for input or PA driving capability for output\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pd_drv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_pd_drv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_pd_drv`] module"]
#[doc(alias = "R32_PA_PD_DRV")]
pub type R32PaPdDrv = crate::Reg<r32_pa_pd_drv::R32PaPdDrvSpec>;
#[doc = "RW, PA pulldown for input or PA driving capability for output"]
pub mod r32_pa_pd_drv;
#[doc = "R32_PB_DIR (rw) register accessor: RW, GPIO PB I/O direction: 0=in, 1=out\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_dir`] module"]
#[doc(alias = "R32_PB_DIR")]
pub type R32PbDir = crate::Reg<r32_pb_dir::R32PbDirSpec>;
#[doc = "RW, GPIO PB I/O direction: 0=in, 1=out"]
pub mod r32_pb_dir;
#[doc = "R32_PB_PIN (r) register accessor: RO, GPIO PB input\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_pin`] module"]
#[doc(alias = "R32_PB_PIN")]
pub type R32PbPin = crate::Reg<r32_pb_pin::R32PbPinSpec>;
#[doc = "RO, GPIO PB input"]
pub mod r32_pb_pin;
#[doc = "R32_PB_OUT__R8_SLV_RD_DATA (rw) register accessor: RW, GPIO PB output;RW, data for parallel slave read\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_out__r8_slv_rd_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_out__r8_slv_rd_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_out__r8_slv_rd_data`] module"]
#[doc(alias = "R32_PB_OUT__R8_SLV_RD_DATA")]
pub type R32PbOut_R8SlvRdData = crate::Reg<r32_pb_out__r8_slv_rd_data::R32PbOut_R8SlvRdDataSpec>;
#[doc = "RW, GPIO PB output;RW, data for parallel slave read"]
pub mod r32_pb_out__r8_slv_rd_data;
#[doc = "R32_PB_CLR (rw) register accessor: WZ, GPIO PB clear output: 0=keep, 1=clear\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_clr`] module"]
#[doc(alias = "R32_PB_CLR")]
pub type R32PbClr = crate::Reg<r32_pb_clr::R32PbClrSpec>;
#[doc = "WZ, GPIO PB clear output: 0=keep, 1=clear"]
pub mod r32_pb_clr;
#[doc = "R32_PB_PU (rw) register accessor: RW, GPIO PB pullup resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_pu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_pu`] module"]
#[doc(alias = "R32_PB_PU")]
pub type R32PbPu = crate::Reg<r32_pb_pu::R32PbPuSpec>;
#[doc = "RW, GPIO PB pullup resistance enable"]
pub mod r32_pb_pu;
#[doc = "R32_PB_PD_DRV (rw) register accessor: RW, PB pulldown for input or PB driving capability for output\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pd_drv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_pd_drv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_pd_drv`] module"]
#[doc(alias = "R32_PB_PD_DRV")]
pub type R32PbPdDrv = crate::Reg<r32_pb_pd_drv::R32PbPdDrvSpec>;
#[doc = "RW, PB pulldown for input or PB driving capability for output"]
pub mod r32_pb_pd_drv;
