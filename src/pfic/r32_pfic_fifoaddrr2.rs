#[doc = "Register `R32_PFIC_FIFOADDRR2` reader"]
pub type R = crate::R<R32PficFifoaddrr2Spec>;
#[doc = "Register `R32_PFIC_FIFOADDRR2` writer"]
pub type W = crate::W<R32PficFifoaddrr2Spec>;
#[doc = "Field `OFFADDR2` reader - RW,OFFADDR2"]
pub type Offaddr2R = crate::FieldReader<u32>;
#[doc = "Field `OFFADDR2` writer - RW,OFFADDR2"]
pub type Offaddr2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `IRQID2` reader - RW,IRQID2"]
pub type Irqid2R = crate::FieldReader;
#[doc = "Field `IRQID2` writer - RW,IRQID2"]
pub type Irqid2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - RW,OFFADDR2"]
    #[inline(always)]
    pub fn offaddr2(&self) -> Offaddr2R {
        Offaddr2R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - RW,IRQID2"]
    #[inline(always)]
    pub fn irqid2(&self) -> Irqid2R {
        Irqid2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - RW,OFFADDR2"]
    #[inline(always)]
    pub fn offaddr2(&mut self) -> Offaddr2W<'_, R32PficFifoaddrr2Spec> {
        Offaddr2W::new(self, 0)
    }
    #[doc = "Bits 24:31 - RW,IRQID2"]
    #[inline(always)]
    pub fn irqid2(&mut self) -> Irqid2W<'_, R32PficFifoaddrr2Spec> {
        Irqid2W::new(self, 24)
    }
}
#[doc = "Interrupt 2 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_fifoaddrr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_fifoaddrr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficFifoaddrr2Spec;
impl crate::RegisterSpec for R32PficFifoaddrr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_fifoaddrr2::R`](R) reader structure"]
impl crate::Readable for R32PficFifoaddrr2Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_fifoaddrr2::W`](W) writer structure"]
impl crate::Writable for R32PficFifoaddrr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_FIFOADDRR2 to value 0"]
impl crate::Resettable for R32PficFifoaddrr2Spec {}
