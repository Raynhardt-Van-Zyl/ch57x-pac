#[doc = "Register `R32_PFIC_FIFOADDRR1` reader"]
pub type R = crate::R<R32PficFifoaddrr1Spec>;
#[doc = "Register `R32_PFIC_FIFOADDRR1` writer"]
pub type W = crate::W<R32PficFifoaddrr1Spec>;
#[doc = "Field `OFFADDR1` reader - RW,OFFADDR1"]
pub type Offaddr1R = crate::FieldReader<u32>;
#[doc = "Field `OFFADDR1` writer - RW,OFFADDR1"]
pub type Offaddr1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `IRQID1` reader - RW,IRQID1"]
pub type Irqid1R = crate::FieldReader;
#[doc = "Field `IRQID1` writer - RW,IRQID1"]
pub type Irqid1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - RW,OFFADDR1"]
    #[inline(always)]
    pub fn offaddr1(&self) -> Offaddr1R {
        Offaddr1R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - RW,IRQID1"]
    #[inline(always)]
    pub fn irqid1(&self) -> Irqid1R {
        Irqid1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - RW,OFFADDR1"]
    #[inline(always)]
    pub fn offaddr1(&mut self) -> Offaddr1W<'_, R32PficFifoaddrr1Spec> {
        Offaddr1W::new(self, 0)
    }
    #[doc = "Bits 24:31 - RW,IRQID1"]
    #[inline(always)]
    pub fn irqid1(&mut self) -> Irqid1W<'_, R32PficFifoaddrr1Spec> {
        Irqid1W::new(self, 24)
    }
}
#[doc = "Interrupt 1 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_fifoaddrr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_fifoaddrr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficFifoaddrr1Spec;
impl crate::RegisterSpec for R32PficFifoaddrr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_fifoaddrr1::R`](R) reader structure"]
impl crate::Readable for R32PficFifoaddrr1Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_fifoaddrr1::W`](W) writer structure"]
impl crate::Writable for R32PficFifoaddrr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_FIFOADDRR1 to value 0"]
impl crate::Resettable for R32PficFifoaddrr1Spec {}
