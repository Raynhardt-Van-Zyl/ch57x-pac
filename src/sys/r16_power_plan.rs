#[doc = "Register `R16_POWER_PLAN` reader"]
pub type R = crate::R<R16PowerPlanSpec>;
#[doc = "Register `R16_POWER_PLAN` writer"]
pub type W = crate::W<R16PowerPlanSpec>;
#[doc = "Field `RB_PWR_XROM` reader - RWA, power for retention 2KB SRAM"]
pub type RbPwrXromR = crate::BitReader;
#[doc = "Field `RB_PWR_XROM` writer - RWA, power for retention 2KB SRAM"]
pub type RbPwrXromW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWR_RAM2K` reader - RWA, power for retention 2KB SRAM"]
pub type RbPwrRam2kR = crate::BitReader;
#[doc = "Field `RB_PWR_RAM2K` writer - RWA, power for retention 2KB SRAM"]
pub type RbPwrRam2kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWR_CORE` reader - RWA, power retention for core and base peripherals"]
pub type RbPwrCoreR = crate::BitReader;
#[doc = "Field `RB_PWR_CORE` writer - RWA, power retention for core and base peripherals"]
pub type RbPwrCoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWR_EXTEND` reader - RWA, power retention for USB and BLE"]
pub type RbPwrExtendR = crate::BitReader;
#[doc = "Field `RB_PWR_EXTEND` writer - RWA, power retention for USB and BLE"]
pub type RbPwrExtendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWR_RAM16K` reader - RWA, power for main SRAM"]
pub type RbPwrRam16kR = crate::BitReader;
#[doc = "Field `RB_PWR_RAM16K` writer - RWA, power for main SRAM"]
pub type RbPwrRam16kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWR_SYS_EN` reader - RWA, power for system"]
pub type RbPwrSysEnR = crate::BitReader;
#[doc = "Field `RB_PWR_SYS_EN` writer - RWA, power for system"]
pub type RbPwrSysEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWR_LDO_EN` reader - RWA, LDO enable"]
pub type RbPwrLdoEnR = crate::BitReader;
#[doc = "Field `RB_PWR_LDO_EN` writer - RWA, LDO enable"]
pub type RbPwrLdoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWR_DCDC_EN` reader - RWA, DC/DC converter enable: 0=DC/DC disable and bypass, 1=DC/DC enable"]
pub type RbPwrDcdcEnR = crate::BitReader;
#[doc = "Field `RB_PWR_DCDC_EN` writer - RWA, DC/DC converter enable: 0=DC/DC disable and bypass, 1=DC/DC enable"]
pub type RbPwrDcdcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWR_DCDC_PRE` reader - RWA, DC/DC converter pre-enable"]
pub type RbPwrDcdcPreR = crate::BitReader;
#[doc = "Field `RB_PWR_DCDC_PRE` writer - RWA, DC/DC converter pre-enable"]
pub type RbPwrDcdcPreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWR_MUST_0010` reader - RWA, power plan enable, auto clear after sleep executed"]
pub type RbPwrMust0010R = crate::FieldReader;
#[doc = "Field `RB_PWR_PLAN_EN` reader - RWA, must write 0010"]
pub type RbPwrPlanEnR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RWA, power for retention 2KB SRAM"]
    #[inline(always)]
    pub fn rb_pwr_xrom(&self) -> RbPwrXromR {
        RbPwrXromR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RWA, power for retention 2KB SRAM"]
    #[inline(always)]
    pub fn rb_pwr_ram2k(&self) -> RbPwrRam2kR {
        RbPwrRam2kR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RWA, power retention for core and base peripherals"]
    #[inline(always)]
    pub fn rb_pwr_core(&self) -> RbPwrCoreR {
        RbPwrCoreR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RWA, power retention for USB and BLE"]
    #[inline(always)]
    pub fn rb_pwr_extend(&self) -> RbPwrExtendR {
        RbPwrExtendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RWA, power for main SRAM"]
    #[inline(always)]
    pub fn rb_pwr_ram16k(&self) -> RbPwrRam16kR {
        RbPwrRam16kR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - RWA, power for system"]
    #[inline(always)]
    pub fn rb_pwr_sys_en(&self) -> RbPwrSysEnR {
        RbPwrSysEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RWA, LDO enable"]
    #[inline(always)]
    pub fn rb_pwr_ldo_en(&self) -> RbPwrLdoEnR {
        RbPwrLdoEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RWA, DC/DC converter enable: 0=DC/DC disable and bypass, 1=DC/DC enable"]
    #[inline(always)]
    pub fn rb_pwr_dcdc_en(&self) -> RbPwrDcdcEnR {
        RbPwrDcdcEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWA, DC/DC converter pre-enable"]
    #[inline(always)]
    pub fn rb_pwr_dcdc_pre(&self) -> RbPwrDcdcPreR {
        RbPwrDcdcPreR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - RWA, power plan enable, auto clear after sleep executed"]
    #[inline(always)]
    pub fn rb_pwr_must_0010(&self) -> RbPwrMust0010R {
        RbPwrMust0010R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - RWA, must write 0010"]
    #[inline(always)]
    pub fn rb_pwr_plan_en(&self) -> RbPwrPlanEnR {
        RbPwrPlanEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RWA, power for retention 2KB SRAM"]
    #[inline(always)]
    pub fn rb_pwr_xrom(&mut self) -> RbPwrXromW<'_, R16PowerPlanSpec> {
        RbPwrXromW::new(self, 0)
    }
    #[doc = "Bit 1 - RWA, power for retention 2KB SRAM"]
    #[inline(always)]
    pub fn rb_pwr_ram2k(&mut self) -> RbPwrRam2kW<'_, R16PowerPlanSpec> {
        RbPwrRam2kW::new(self, 1)
    }
    #[doc = "Bit 2 - RWA, power retention for core and base peripherals"]
    #[inline(always)]
    pub fn rb_pwr_core(&mut self) -> RbPwrCoreW<'_, R16PowerPlanSpec> {
        RbPwrCoreW::new(self, 2)
    }
    #[doc = "Bit 3 - RWA, power retention for USB and BLE"]
    #[inline(always)]
    pub fn rb_pwr_extend(&mut self) -> RbPwrExtendW<'_, R16PowerPlanSpec> {
        RbPwrExtendW::new(self, 3)
    }
    #[doc = "Bit 4 - RWA, power for main SRAM"]
    #[inline(always)]
    pub fn rb_pwr_ram16k(&mut self) -> RbPwrRam16kW<'_, R16PowerPlanSpec> {
        RbPwrRam16kW::new(self, 4)
    }
    #[doc = "Bit 7 - RWA, power for system"]
    #[inline(always)]
    pub fn rb_pwr_sys_en(&mut self) -> RbPwrSysEnW<'_, R16PowerPlanSpec> {
        RbPwrSysEnW::new(self, 7)
    }
    #[doc = "Bit 8 - RWA, LDO enable"]
    #[inline(always)]
    pub fn rb_pwr_ldo_en(&mut self) -> RbPwrLdoEnW<'_, R16PowerPlanSpec> {
        RbPwrLdoEnW::new(self, 8)
    }
    #[doc = "Bit 9 - RWA, DC/DC converter enable: 0=DC/DC disable and bypass, 1=DC/DC enable"]
    #[inline(always)]
    pub fn rb_pwr_dcdc_en(&mut self) -> RbPwrDcdcEnW<'_, R16PowerPlanSpec> {
        RbPwrDcdcEnW::new(self, 9)
    }
    #[doc = "Bit 10 - RWA, DC/DC converter pre-enable"]
    #[inline(always)]
    pub fn rb_pwr_dcdc_pre(&mut self) -> RbPwrDcdcPreW<'_, R16PowerPlanSpec> {
        RbPwrDcdcPreW::new(self, 10)
    }
}
#[doc = "RWA, power plan before sleep instruction, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_power_plan::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_power_plan::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16PowerPlanSpec;
impl crate::RegisterSpec for R16PowerPlanSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_power_plan::R`](R) reader structure"]
impl crate::Readable for R16PowerPlanSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_power_plan::W`](W) writer structure"]
impl crate::Writable for R16PowerPlanSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_POWER_PLAN to value 0x11df"]
impl crate::Resettable for R16PowerPlanSpec {
    const RESET_VALUE: u16 = 0x11df;
}
