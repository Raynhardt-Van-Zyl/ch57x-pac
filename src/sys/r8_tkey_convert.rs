#[doc = "Register `R8_TKEY_CONVERT` reader"]
pub type R = crate::R<R8TkeyConvertSpec>;
#[doc = "Register `R8_TKEY_CONVERT` writer"]
pub type W = crate::W<R8TkeyConvertSpec>;
#[doc = "Field `RB_TKEY_START` reader - RW, Touchkey convert start control"]
pub type RbTkeyStartR = crate::BitReader;
#[doc = "Field `RB_TKEY_START` writer - RW, Touchkey convert start control"]
pub type RbTkeyStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RW, Touchkey convert start control"]
    #[inline(always)]
    pub fn rb_tkey_start(&self) -> RbTkeyStartR {
        RbTkeyStartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW, Touchkey convert start control"]
    #[inline(always)]
    pub fn rb_tkey_start(&mut self) -> RbTkeyStartW<'_, R8TkeyConvertSpec> {
        RbTkeyStartW::new(self, 0)
    }
}
#[doc = "RW, Touchkey convert start control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tkey_convert::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tkey_convert::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8TkeyConvertSpec;
impl crate::RegisterSpec for R8TkeyConvertSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_tkey_convert::R`](R) reader structure"]
impl crate::Readable for R8TkeyConvertSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_tkey_convert::W`](W) writer structure"]
impl crate::Writable for R8TkeyConvertSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_TKEY_CONVERT to value 0"]
impl crate::Resettable for R8TkeyConvertSpec {}
