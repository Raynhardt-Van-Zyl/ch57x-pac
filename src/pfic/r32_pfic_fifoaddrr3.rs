#[doc = "Register `R32_PFIC_FIFOADDRR3` reader"]
pub type R = crate::R<R32PficFifoaddrr3Spec>;
#[doc = "Register `R32_PFIC_FIFOADDRR3` writer"]
pub type W = crate::W<R32PficFifoaddrr3Spec>;
#[doc = "Field `OFFADDR3` reader - RW,OFFADDR3"]
pub type Offaddr3R = crate::FieldReader<u32>;
#[doc = "Field `OFFADDR3` writer - RW,OFFADDR3"]
pub type Offaddr3W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `IRQID3` reader - RW,IRQID3"]
pub type Irqid3R = crate::FieldReader;
#[doc = "Field `IRQID3` writer - RW,IRQID3"]
pub type Irqid3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - RW,OFFADDR3"]
    #[inline(always)]
    pub fn offaddr3(&self) -> Offaddr3R {
        Offaddr3R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - RW,IRQID3"]
    #[inline(always)]
    pub fn irqid3(&self) -> Irqid3R {
        Irqid3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - RW,OFFADDR3"]
    #[inline(always)]
    pub fn offaddr3(&mut self) -> Offaddr3W<'_, R32PficFifoaddrr3Spec> {
        Offaddr3W::new(self, 0)
    }
    #[doc = "Bits 24:31 - RW,IRQID3"]
    #[inline(always)]
    pub fn irqid3(&mut self) -> Irqid3W<'_, R32PficFifoaddrr3Spec> {
        Irqid3W::new(self, 24)
    }
}
#[doc = "Interrupt 3 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_fifoaddrr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_fifoaddrr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficFifoaddrr3Spec;
impl crate::RegisterSpec for R32PficFifoaddrr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_fifoaddrr3::R`](R) reader structure"]
impl crate::Readable for R32PficFifoaddrr3Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_fifoaddrr3::W`](W) writer structure"]
impl crate::Writable for R32PficFifoaddrr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_FIFOADDRR3 to value 0"]
impl crate::Resettable for R32PficFifoaddrr3Spec {}
