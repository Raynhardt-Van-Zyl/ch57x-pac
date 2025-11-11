#[doc = "Register `R16_UEP0_DMA` reader"]
pub type R = crate::R<R16Uep0DmaSpec>;
#[doc = "Register `R16_UEP0_DMA` writer"]
pub type W = crate::W<R16Uep0DmaSpec>;
#[doc = "Field `R16_UEP0_DMA` reader - RW,endpoint 0 DMA buffer address"]
pub type R16Uep0DmaR = crate::FieldReader<u16>;
#[doc = "Field `R16_UEP0_DMA` writer - RW,endpoint 0 DMA buffer address"]
pub type R16Uep0DmaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RW,endpoint 0 DMA buffer address"]
    #[inline(always)]
    pub fn r16_uep0_dma(&self) -> R16Uep0DmaR {
        R16Uep0DmaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RW,endpoint 0 DMA buffer address"]
    #[inline(always)]
    pub fn r16_uep0_dma(&mut self) -> R16Uep0DmaW<'_, R16Uep0DmaSpec> {
        R16Uep0DmaW::new(self, 0)
    }
}
#[doc = "endpoint 0 DMA buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep0_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep0_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uep0DmaSpec;
impl crate::RegisterSpec for R16Uep0DmaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uep0_dma::R`](R) reader structure"]
impl crate::Readable for R16Uep0DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uep0_dma::W`](W) writer structure"]
impl crate::Writable for R16Uep0DmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UEP0_DMA to value 0"]
impl crate::Resettable for R16Uep0DmaSpec {}
