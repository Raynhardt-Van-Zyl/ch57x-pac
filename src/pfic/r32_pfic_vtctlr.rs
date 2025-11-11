#[doc = "Register `R32_PFIC_VTCTLR` reader"]
pub type R = crate::R<R32PficVtctlrSpec>;
#[doc = "Register `R32_PFIC_VTCTLR` writer"]
pub type W = crate::W<R32PficVtctlrSpec>;
#[doc = "Field `VTADDR` reader - VTADDR"]
pub type VtaddrR = crate::BitReader;
#[doc = "Field `VTADDR` writer - VTADDR"]
pub type VtaddrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VTADDR"]
    #[inline(always)]
    pub fn vtaddr(&self) -> VtaddrR {
        VtaddrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VTADDR"]
    #[inline(always)]
    pub fn vtaddr(&mut self) -> VtaddrW<'_, R32PficVtctlrSpec> {
        VtaddrW::new(self, 0)
    }
}
#[doc = "System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_vtctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_vtctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficVtctlrSpec;
impl crate::RegisterSpec for R32PficVtctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_vtctlr::R`](R) reader structure"]
impl crate::Readable for R32PficVtctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_vtctlr::W`](W) writer structure"]
impl crate::Writable for R32PficVtctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_VTCTLR to value 0"]
impl crate::Resettable for R32PficVtctlrSpec {}
