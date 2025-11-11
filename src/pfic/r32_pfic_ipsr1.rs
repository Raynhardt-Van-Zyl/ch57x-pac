#[doc = "Register `R32_PFIC_IPSR1` reader"]
pub type R = crate::R<R32PficIpsr1Spec>;
#[doc = "Register `R32_PFIC_IPSR1` writer"]
pub type W = crate::W<R32PficIpsr1Spec>;
#[doc = "Field `PENDSET` reader - RW1,PENDSET"]
pub type PendsetR = crate::FieldReader<u32>;
#[doc = "Field `PENDSET` writer - RW1,PENDSET"]
pub type PendsetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - RW1,PENDSET"]
    #[inline(always)]
    pub fn pendset(&self) -> PendsetR {
        PendsetR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - RW1,PENDSET"]
    #[inline(always)]
    pub fn pendset(&mut self) -> PendsetW<'_, R32PficIpsr1Spec> {
        PendsetW::new(self, 12)
    }
}
#[doc = "Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ipsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_ipsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIpsr1Spec;
impl crate::RegisterSpec for R32PficIpsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_ipsr1::R`](R) reader structure"]
impl crate::Readable for R32PficIpsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_ipsr1::W`](W) writer structure"]
impl crate::Writable for R32PficIpsr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPSR1 to value 0"]
impl crate::Resettable for R32PficIpsr1Spec {}
