#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_pwm_out_en: R8PwmOutEn,
    r8_pwm_polar: R8PwmPolar,
    r8_pwm_config: R8PwmConfig,
    r8_pwm_clock_div: R8PwmClockDiv,
    r8_pwm4_data: R8Pwm4Data,
    r8_pwm5_data: R8Pwm5Data,
    r8_pwm6_data: R8Pwm6Data,
    r8_pwm7_data: R8Pwm7Data,
    r8_pwm8_data: R8Pwm8Data,
    r8_pwm9_data: R8Pwm9Data,
    r8_pwm10_data: R8Pwm10Data,
    r8_pwm11_data: R8Pwm11Data,
    r8_pwm_int_ctrl: R8PwmIntCtrl,
}
impl RegisterBlock {
    #[doc = "0x00 - RW, PWM output enable control"]
    #[inline(always)]
    pub const fn r8_pwm_out_en(&self) -> &R8PwmOutEn {
        &self.r8_pwm_out_en
    }
    #[doc = "0x01 - RW, PWM output polarity control"]
    #[inline(always)]
    pub const fn r8_pwm_polar(&self) -> &R8PwmPolar {
        &self.r8_pwm_polar
    }
    #[doc = "0x02 - RW, PWM configuration"]
    #[inline(always)]
    pub const fn r8_pwm_config(&self) -> &R8PwmConfig {
        &self.r8_pwm_config
    }
    #[doc = "0x03 - RW, PWM clock divisor"]
    #[inline(always)]
    pub const fn r8_pwm_clock_div(&self) -> &R8PwmClockDiv {
        &self.r8_pwm_clock_div
    }
    #[doc = "0x04 - RW, PWM4 data holding"]
    #[inline(always)]
    pub const fn r8_pwm4_data(&self) -> &R8Pwm4Data {
        &self.r8_pwm4_data
    }
    #[doc = "0x05 - RW, PWM5 data holding"]
    #[inline(always)]
    pub const fn r8_pwm5_data(&self) -> &R8Pwm5Data {
        &self.r8_pwm5_data
    }
    #[doc = "0x06 - RW, PWM6 data holding"]
    #[inline(always)]
    pub const fn r8_pwm6_data(&self) -> &R8Pwm6Data {
        &self.r8_pwm6_data
    }
    #[doc = "0x07 - RW, PWM7 data holding"]
    #[inline(always)]
    pub const fn r8_pwm7_data(&self) -> &R8Pwm7Data {
        &self.r8_pwm7_data
    }
    #[doc = "0x08 - RW, PWM8 data holding"]
    #[inline(always)]
    pub const fn r8_pwm8_data(&self) -> &R8Pwm8Data {
        &self.r8_pwm8_data
    }
    #[doc = "0x09 - RW, PWM9 data holding"]
    #[inline(always)]
    pub const fn r8_pwm9_data(&self) -> &R8Pwm9Data {
        &self.r8_pwm9_data
    }
    #[doc = "0x0a - RW, PWM10 data holding"]
    #[inline(always)]
    pub const fn r8_pwm10_data(&self) -> &R8Pwm10Data {
        &self.r8_pwm10_data
    }
    #[doc = "0x0b - RW, PWM11 data holding"]
    #[inline(always)]
    pub const fn r8_pwm11_data(&self) -> &R8Pwm11Data {
        &self.r8_pwm11_data
    }
    #[doc = "0x0c - RW, PWM interrupt control"]
    #[inline(always)]
    pub const fn r8_pwm_int_ctrl(&self) -> &R8PwmIntCtrl {
        &self.r8_pwm_int_ctrl
    }
}
#[doc = "R8_PWM_OUT_EN (rw) register accessor: RW, PWM output enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_out_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_out_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm_out_en`] module"]
#[doc(alias = "R8_PWM_OUT_EN")]
pub type R8PwmOutEn = crate::Reg<r8_pwm_out_en::R8PwmOutEnSpec>;
#[doc = "RW, PWM output enable control"]
pub mod r8_pwm_out_en;
#[doc = "R8_PWM_POLAR (rw) register accessor: RW, PWM output polarity control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_polar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_polar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm_polar`] module"]
#[doc(alias = "R8_PWM_POLAR")]
pub type R8PwmPolar = crate::Reg<r8_pwm_polar::R8PwmPolarSpec>;
#[doc = "RW, PWM output polarity control"]
pub mod r8_pwm_polar;
#[doc = "R8_PWM_CONFIG (rw) register accessor: RW, PWM configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm_config`] module"]
#[doc(alias = "R8_PWM_CONFIG")]
pub type R8PwmConfig = crate::Reg<r8_pwm_config::R8PwmConfigSpec>;
#[doc = "RW, PWM configuration"]
pub mod r8_pwm_config;
#[doc = "R8_PWM_CLOCK_DIV (rw) register accessor: RW, PWM clock divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_clock_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_clock_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm_clock_div`] module"]
#[doc(alias = "R8_PWM_CLOCK_DIV")]
pub type R8PwmClockDiv = crate::Reg<r8_pwm_clock_div::R8PwmClockDivSpec>;
#[doc = "RW, PWM clock divisor"]
pub mod r8_pwm_clock_div;
#[doc = "R8_PWM4_DATA (rw) register accessor: RW, PWM4 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm4_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm4_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm4_data`] module"]
#[doc(alias = "R8_PWM4_DATA")]
pub type R8Pwm4Data = crate::Reg<r8_pwm4_data::R8Pwm4DataSpec>;
#[doc = "RW, PWM4 data holding"]
pub mod r8_pwm4_data;
#[doc = "R8_PWM5_DATA (rw) register accessor: RW, PWM5 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm5_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm5_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm5_data`] module"]
#[doc(alias = "R8_PWM5_DATA")]
pub type R8Pwm5Data = crate::Reg<r8_pwm5_data::R8Pwm5DataSpec>;
#[doc = "RW, PWM5 data holding"]
pub mod r8_pwm5_data;
#[doc = "R8_PWM6_DATA (rw) register accessor: RW, PWM6 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm6_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm6_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm6_data`] module"]
#[doc(alias = "R8_PWM6_DATA")]
pub type R8Pwm6Data = crate::Reg<r8_pwm6_data::R8Pwm6DataSpec>;
#[doc = "RW, PWM6 data holding"]
pub mod r8_pwm6_data;
#[doc = "R8_PWM7_DATA (rw) register accessor: RW, PWM7 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm7_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm7_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm7_data`] module"]
#[doc(alias = "R8_PWM7_DATA")]
pub type R8Pwm7Data = crate::Reg<r8_pwm7_data::R8Pwm7DataSpec>;
#[doc = "RW, PWM7 data holding"]
pub mod r8_pwm7_data;
#[doc = "R8_PWM8_DATA (rw) register accessor: RW, PWM8 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm8_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm8_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm8_data`] module"]
#[doc(alias = "R8_PWM8_DATA")]
pub type R8Pwm8Data = crate::Reg<r8_pwm8_data::R8Pwm8DataSpec>;
#[doc = "RW, PWM8 data holding"]
pub mod r8_pwm8_data;
#[doc = "R8_PWM9_DATA (rw) register accessor: RW, PWM9 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm9_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm9_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm9_data`] module"]
#[doc(alias = "R8_PWM9_DATA")]
pub type R8Pwm9Data = crate::Reg<r8_pwm9_data::R8Pwm9DataSpec>;
#[doc = "RW, PWM9 data holding"]
pub mod r8_pwm9_data;
#[doc = "R8_PWM10_DATA (rw) register accessor: RW, PWM10 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm10_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm10_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm10_data`] module"]
#[doc(alias = "R8_PWM10_DATA")]
pub type R8Pwm10Data = crate::Reg<r8_pwm10_data::R8Pwm10DataSpec>;
#[doc = "RW, PWM10 data holding"]
pub mod r8_pwm10_data;
#[doc = "R8_PWM11_DATA (rw) register accessor: RW, PWM11 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm11_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm11_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm11_data`] module"]
#[doc(alias = "R8_PWM11_DATA")]
pub type R8Pwm11Data = crate::Reg<r8_pwm11_data::R8Pwm11DataSpec>;
#[doc = "RW, PWM11 data holding"]
pub mod r8_pwm11_data;
#[doc = "R8_PWM_INT_CTRL (rw) register accessor: RW, PWM interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm_int_ctrl`] module"]
#[doc(alias = "R8_PWM_INT_CTRL")]
pub type R8PwmIntCtrl = crate::Reg<r8_pwm_int_ctrl::R8PwmIntCtrlSpec>;
#[doc = "RW, PWM interrupt control"]
pub mod r8_pwm_int_ctrl;
