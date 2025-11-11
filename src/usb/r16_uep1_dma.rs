#[doc = "Register `R16_UEP1_DMA` reader"]
pub type R = crate::R<R16Uep1DmaSpec>;
#[doc = "Register `R16_UEP1_DMA` writer"]
pub type W = crate::W<R16Uep1DmaSpec>;
#[doc = "Field `R16_UEP1_DMA` reader - RW,endpoint 1 DMA buffer address"]
pub type R16Uep1DmaR = crate::FieldReader<u16>;
#[doc = "Field `R16_UEP1_DMA` writer - RW,endpoint 1 DMA buffer address"]
pub type R16Uep1DmaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RW,endpoint 1 DMA buffer address"]
    #[inline(always)]
    pub fn r16_uep1_dma(&self) -> R16Uep1DmaR {
        R16Uep1DmaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RW,endpoint 1 DMA buffer address"]
    #[inline(always)]
    pub fn r16_uep1_dma(&mut self) -> R16Uep1DmaW<'_, R16Uep1DmaSpec> {
        R16Uep1DmaW::new(self, 0)
    }
}
#[doc = "endpoint 1 DMA buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep1_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep1_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uep1DmaSpec;
impl crate::RegisterSpec for R16Uep1DmaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uep1_dma::R`](R) reader structure"]
impl crate::Readable for R16Uep1DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uep1_dma::W`](W) writer structure"]
impl crate::Writable for R16Uep1DmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UEP1_DMA to value 0"]
impl crate::Resettable for R16Uep1DmaSpec {}
