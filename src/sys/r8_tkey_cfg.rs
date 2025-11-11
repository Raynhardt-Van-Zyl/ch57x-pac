#[doc = "Register `R8_TKEY_CFG` reader"]
pub type R = crate::R<R8TkeyCfgSpec>;
#[doc = "Register `R8_TKEY_CFG` writer"]
pub type W = crate::W<R8TkeyCfgSpec>;
#[doc = "Field `RB_TKEY_PWR_ON` reader - RW, Touchkey power on"]
pub type RbTkeyPwrOnR = crate::BitReader;
#[doc = "Field `RB_TKEY_PWR_ON` writer - RW, Touchkey power on"]
pub type RbTkeyPwrOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TKEY_CURRENT` reader - RW, Touchkey charge current selection"]
pub type RbTkeyCurrentR = crate::BitReader;
#[doc = "Field `RB_TKEY_CURRENT` writer - RW, Touchkey charge current selection"]
pub type RbTkeyCurrentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TKEY_PGA_ADJ` reader - RW, ADC input PGA speed selection"]
pub type RbTkeyPgaAdjR = crate::BitReader;
#[doc = "Field `RB_TKEY_PGA_ADJ` writer - RW, ADC input PGA speed selection"]
pub type RbTkeyPgaAdjW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RW, Touchkey power on"]
    #[inline(always)]
    pub fn rb_tkey_pwr_on(&self) -> RbTkeyPwrOnR {
        RbTkeyPwrOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW, Touchkey charge current selection"]
    #[inline(always)]
    pub fn rb_tkey_current(&self) -> RbTkeyCurrentR {
        RbTkeyCurrentR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - RW, ADC input PGA speed selection"]
    #[inline(always)]
    pub fn rb_tkey_pga_adj(&self) -> RbTkeyPgaAdjR {
        RbTkeyPgaAdjR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW, Touchkey power on"]
    #[inline(always)]
    pub fn rb_tkey_pwr_on(&mut self) -> RbTkeyPwrOnW<'_, R8TkeyCfgSpec> {
        RbTkeyPwrOnW::new(self, 0)
    }
    #[doc = "Bit 1 - RW, Touchkey charge current selection"]
    #[inline(always)]
    pub fn rb_tkey_current(&mut self) -> RbTkeyCurrentW<'_, R8TkeyCfgSpec> {
        RbTkeyCurrentW::new(self, 1)
    }
    #[doc = "Bit 3 - RW, ADC input PGA speed selection"]
    #[inline(always)]
    pub fn rb_tkey_pga_adj(&mut self) -> RbTkeyPgaAdjW<'_, R8TkeyCfgSpec> {
        RbTkeyPgaAdjW::new(self, 3)
    }
}
#[doc = "RW, Touchkey configure\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tkey_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tkey_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8TkeyCfgSpec;
impl crate::RegisterSpec for R8TkeyCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_tkey_cfg::R`](R) reader structure"]
impl crate::Readable for R8TkeyCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_tkey_cfg::W`](W) writer structure"]
impl crate::Writable for R8TkeyCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_TKEY_CFG to value 0"]
impl crate::Resettable for R8TkeyCfgSpec {}
