#[doc = "Register `R32_PFIC_CFGR` reader"]
pub type R = crate::R<R32PficCfgrSpec>;
#[doc = "Register `R32_PFIC_CFGR` writer"]
pub type W = crate::W<R32PficCfgrSpec>;
#[doc = "Field `HWSTKCTRL` reader - RW,HWSTKCTRL"]
pub type HwstkctrlR = crate::BitReader;
#[doc = "Field `HWSTKCTRL` writer - RW,HWSTKCTRL"]
pub type HwstkctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NESTCTRL` reader - RW,NESTCTRL"]
pub type NestctrlR = crate::BitReader;
#[doc = "Field `NESTCTRL` writer - RW,NESTCTRL"]
pub type NestctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMISET` reader - WO,NMISET"]
pub type NmisetR = crate::BitReader;
#[doc = "Field `NMISET` writer - WO,NMISET"]
pub type NmisetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIRESET` reader - WO,NMIRESET"]
pub type NmiresetR = crate::BitReader;
#[doc = "Field `NMIRESET` writer - WO,NMIRESET"]
pub type NmiresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCSET` reader - WO,EXCSET"]
pub type ExcsetR = crate::BitReader;
#[doc = "Field `EXCSET` writer - WO,EXCSET"]
pub type ExcsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCRESET` reader - WO,EXCRESET"]
pub type ExcresetR = crate::BitReader;
#[doc = "Field `EXCRESET` writer - WO,EXCRESET"]
pub type ExcresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFICRESET` reader - WO,PFICRSET"]
pub type PficresetR = crate::BitReader;
#[doc = "Field `PFICRESET` writer - WO,PFICRSET"]
pub type PficresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRESET` reader - WO,SYSRESET"]
pub type SysresetR = crate::BitReader;
#[doc = "Field `SYSRESET` writer - WO,SYSRESET"]
pub type SysresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCODE` reader - WO,KEYCODE"]
pub type KeycodeR = crate::FieldReader<u16>;
#[doc = "Field `KEYCODE` writer - WO,KEYCODE"]
pub type KeycodeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - RW,HWSTKCTRL"]
    #[inline(always)]
    pub fn hwstkctrl(&self) -> HwstkctrlR {
        HwstkctrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW,NESTCTRL"]
    #[inline(always)]
    pub fn nestctrl(&self) -> NestctrlR {
        NestctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WO,NMISET"]
    #[inline(always)]
    pub fn nmiset(&self) -> NmisetR {
        NmisetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WO,NMIRESET"]
    #[inline(always)]
    pub fn nmireset(&self) -> NmiresetR {
        NmiresetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WO,EXCSET"]
    #[inline(always)]
    pub fn excset(&self) -> ExcsetR {
        ExcsetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WO,EXCRESET"]
    #[inline(always)]
    pub fn excreset(&self) -> ExcresetR {
        ExcresetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WO,PFICRSET"]
    #[inline(always)]
    pub fn pficreset(&self) -> PficresetR {
        PficresetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WO,SYSRESET"]
    #[inline(always)]
    pub fn sysreset(&self) -> SysresetR {
        SysresetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - WO,KEYCODE"]
    #[inline(always)]
    pub fn keycode(&self) -> KeycodeR {
        KeycodeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - RW,HWSTKCTRL"]
    #[inline(always)]
    pub fn hwstkctrl(&mut self) -> HwstkctrlW<'_, R32PficCfgrSpec> {
        HwstkctrlW::new(self, 0)
    }
    #[doc = "Bit 1 - RW,NESTCTRL"]
    #[inline(always)]
    pub fn nestctrl(&mut self) -> NestctrlW<'_, R32PficCfgrSpec> {
        NestctrlW::new(self, 1)
    }
    #[doc = "Bit 2 - WO,NMISET"]
    #[inline(always)]
    pub fn nmiset(&mut self) -> NmisetW<'_, R32PficCfgrSpec> {
        NmisetW::new(self, 2)
    }
    #[doc = "Bit 3 - WO,NMIRESET"]
    #[inline(always)]
    pub fn nmireset(&mut self) -> NmiresetW<'_, R32PficCfgrSpec> {
        NmiresetW::new(self, 3)
    }
    #[doc = "Bit 4 - WO,EXCSET"]
    #[inline(always)]
    pub fn excset(&mut self) -> ExcsetW<'_, R32PficCfgrSpec> {
        ExcsetW::new(self, 4)
    }
    #[doc = "Bit 5 - WO,EXCRESET"]
    #[inline(always)]
    pub fn excreset(&mut self) -> ExcresetW<'_, R32PficCfgrSpec> {
        ExcresetW::new(self, 5)
    }
    #[doc = "Bit 6 - WO,PFICRSET"]
    #[inline(always)]
    pub fn pficreset(&mut self) -> PficresetW<'_, R32PficCfgrSpec> {
        PficresetW::new(self, 6)
    }
    #[doc = "Bit 7 - WO,SYSRESET"]
    #[inline(always)]
    pub fn sysreset(&mut self) -> SysresetW<'_, R32PficCfgrSpec> {
        SysresetW::new(self, 7)
    }
    #[doc = "Bits 16:31 - WO,KEYCODE"]
    #[inline(always)]
    pub fn keycode(&mut self) -> KeycodeW<'_, R32PficCfgrSpec> {
        KeycodeW::new(self, 16)
    }
}
#[doc = "Interrupt Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficCfgrSpec;
impl crate::RegisterSpec for R32PficCfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_cfgr::R`](R) reader structure"]
impl crate::Readable for R32PficCfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_cfgr::W`](W) writer structure"]
impl crate::Writable for R32PficCfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_CFGR to value 0"]
impl crate::Resettable for R32PficCfgrSpec {}
