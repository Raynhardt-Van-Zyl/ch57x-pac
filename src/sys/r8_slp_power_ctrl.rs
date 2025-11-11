#[doc = "Register `R8_SLP_POWER_CTRL` reader"]
pub type R = crate::R<R8SlpPowerCtrlSpec>;
#[doc = "Register `R8_SLP_POWER_CTRL` writer"]
pub type W = crate::W<R8SlpPowerCtrlSpec>;
#[doc = "Field `RB_SLP_CLK_RAMX` reader - RWA, close main SRAM clock"]
pub type RbSlpClkRamxR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_RAMX` writer - RWA, close main SRAM clock"]
pub type RbSlpClkRamxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_RAM2K` reader - RWA, close retention 2KB SRAM clock"]
pub type RbSlpClkRam2kR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_RAM2K` writer - RWA, close retention 2KB SRAM clock"]
pub type RbSlpClkRam2kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_RAM_RET_LV` reader - RWA, SRAM retention voltage selection"]
pub type RbRamRetLvR = crate::BitReader;
#[doc = "Field `RB_RAM_RET_LV` writer - RWA, SRAM retention voltage selection"]
pub type RbRamRetLvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - RWA, close main SRAM clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ramx(&self) -> RbSlpClkRamxR {
        RbSlpClkRamxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RWA, close retention 2KB SRAM clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ram2k(&self) -> RbSlpClkRam2kR {
        RbSlpClkRam2kR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RWA, SRAM retention voltage selection"]
    #[inline(always)]
    pub fn rb_ram_ret_lv(&self) -> RbRamRetLvR {
        RbRamRetLvR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RWA, close main SRAM clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ramx(&mut self) -> RbSlpClkRamxW<'_, R8SlpPowerCtrlSpec> {
        RbSlpClkRamxW::new(self, 4)
    }
    #[doc = "Bit 5 - RWA, close retention 2KB SRAM clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ram2k(&mut self) -> RbSlpClkRam2kW<'_, R8SlpPowerCtrlSpec> {
        RbSlpClkRam2kW::new(self, 5)
    }
    #[doc = "Bit 6 - RWA, SRAM retention voltage selection"]
    #[inline(always)]
    pub fn rb_ram_ret_lv(&mut self) -> RbRamRetLvW<'_, R8SlpPowerCtrlSpec> {
        RbRamRetLvW::new(self, 6)
    }
}
#[doc = "RWA, peripherals power down control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_power_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_power_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8SlpPowerCtrlSpec;
impl crate::RegisterSpec for R8SlpPowerCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_slp_power_ctrl::R`](R) reader structure"]
impl crate::Readable for R8SlpPowerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_slp_power_ctrl::W`](W) writer structure"]
impl crate::Writable for R8SlpPowerCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SLP_POWER_CTRL to value 0"]
impl crate::Resettable for R8SlpPowerCtrlSpec {}
