#[doc = "Register `R32_PFIC_FIFOADDRR0` reader"]
pub type R = crate::R<R32PficFifoaddrr0Spec>;
#[doc = "Register `R32_PFIC_FIFOADDRR0` writer"]
pub type W = crate::W<R32PficFifoaddrr0Spec>;
#[doc = "Field `OFFADDR0` reader - RW,OFFADDR0"]
pub type Offaddr0R = crate::FieldReader<u32>;
#[doc = "Field `OFFADDR0` writer - RW,OFFADDR0"]
pub type Offaddr0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `IRQID0` reader - RW,IRQID0"]
pub type Irqid0R = crate::FieldReader;
#[doc = "Field `IRQID0` writer - RW,IRQID0"]
pub type Irqid0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - RW,OFFADDR0"]
    #[inline(always)]
    pub fn offaddr0(&self) -> Offaddr0R {
        Offaddr0R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - RW,IRQID0"]
    #[inline(always)]
    pub fn irqid0(&self) -> Irqid0R {
        Irqid0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - RW,OFFADDR0"]
    #[inline(always)]
    pub fn offaddr0(&mut self) -> Offaddr0W<'_, R32PficFifoaddrr0Spec> {
        Offaddr0W::new(self, 0)
    }
    #[doc = "Bits 24:31 - RW,IRQID0"]
    #[inline(always)]
    pub fn irqid0(&mut self) -> Irqid0W<'_, R32PficFifoaddrr0Spec> {
        Irqid0W::new(self, 24)
    }
}
#[doc = "Interrupt 0 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_fifoaddrr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_fifoaddrr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficFifoaddrr0Spec;
impl crate::RegisterSpec for R32PficFifoaddrr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_fifoaddrr0::R`](R) reader structure"]
impl crate::Readable for R32PficFifoaddrr0Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_fifoaddrr0::W`](W) writer structure"]
impl crate::Writable for R32PficFifoaddrr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_FIFOADDRR0 to value 0"]
impl crate::Resettable for R32PficFifoaddrr0Spec {}
