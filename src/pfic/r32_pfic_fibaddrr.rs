#[doc = "Register `R32_PFIC_FIBADDRR` reader"]
pub type R = crate::R<R32PficFibaddrrSpec>;
#[doc = "Register `R32_PFIC_FIBADDRR` writer"]
pub type W = crate::W<R32PficFibaddrrSpec>;
#[doc = "Field `BASEADDR` reader - BASEADDR"]
pub type BaseaddrR = crate::FieldReader;
#[doc = "Field `BASEADDR` writer - BASEADDR"]
pub type BaseaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 28:31 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BaseaddrR {
        BaseaddrR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&mut self) -> BaseaddrW<'_, R32PficFibaddrrSpec> {
        BaseaddrW::new(self, 28)
    }
}
#[doc = "RW,Interrupt Fast Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_fibaddrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_fibaddrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficFibaddrrSpec;
impl crate::RegisterSpec for R32PficFibaddrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_fibaddrr::R`](R) reader structure"]
impl crate::Readable for R32PficFibaddrrSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_fibaddrr::W`](W) writer structure"]
impl crate::Writable for R32PficFibaddrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_FIBADDRR to value 0"]
impl crate::Resettable for R32PficFibaddrrSpec {}
