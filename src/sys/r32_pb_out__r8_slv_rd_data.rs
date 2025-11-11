#[doc = "Register `R32_PB_OUT__R8_SLV_RD_DATA` reader"]
pub type R = crate::R<R32PbOut_R8SlvRdDataSpec>;
#[doc = "Register `R32_PB_OUT__R8_SLV_RD_DATA` writer"]
pub type W = crate::W<R32PbOut_R8SlvRdDataSpec>;
#[doc = "Field `R8_PB_OUT_0` reader - GPIO PB output byte 0"]
pub type R8PbOut0R = crate::FieldReader;
#[doc = "Field `R8_PB_OUT_0` writer - GPIO PB output byte 0"]
pub type R8PbOut0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PB_OUT_1` reader - GPIO PB output byte 1"]
pub type R8PbOut1R = crate::FieldReader;
#[doc = "Field `R8_PB_OUT_1` writer - GPIO PB output byte 1"]
pub type R8PbOut1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PB_OUT_2` reader - GPIO PB output byte 2"]
pub type R8PbOut2R = crate::FieldReader;
#[doc = "Field `R8_PB_OUT_2` writer - GPIO PB output byte 2"]
pub type R8PbOut2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO PB output byte 0"]
    #[inline(always)]
    pub fn r8_pb_out_0(&self) -> R8PbOut0R {
        R8PbOut0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO PB output byte 1"]
    #[inline(always)]
    pub fn r8_pb_out_1(&self) -> R8PbOut1R {
        R8PbOut1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO PB output byte 2"]
    #[inline(always)]
    pub fn r8_pb_out_2(&self) -> R8PbOut2R {
        R8PbOut2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO PB output byte 0"]
    #[inline(always)]
    pub fn r8_pb_out_0(&mut self) -> R8PbOut0W<'_, R32PbOut_R8SlvRdDataSpec> {
        R8PbOut0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO PB output byte 1"]
    #[inline(always)]
    pub fn r8_pb_out_1(&mut self) -> R8PbOut1W<'_, R32PbOut_R8SlvRdDataSpec> {
        R8PbOut1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO PB output byte 2"]
    #[inline(always)]
    pub fn r8_pb_out_2(&mut self) -> R8PbOut2W<'_, R32PbOut_R8SlvRdDataSpec> {
        R8PbOut2W::new(self, 16)
    }
}
#[doc = "RW, GPIO PB output;RW, data for parallel slave read\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_out__r8_slv_rd_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_out__r8_slv_rd_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbOut_R8SlvRdDataSpec;
impl crate::RegisterSpec for R32PbOut_R8SlvRdDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_out__r8_slv_rd_data::R`](R) reader structure"]
impl crate::Readable for R32PbOut_R8SlvRdDataSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pb_out__r8_slv_rd_data::W`](W) writer structure"]
impl crate::Writable for R32PbOut_R8SlvRdDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PB_OUT__R8_SLV_RD_DATA to value 0"]
impl crate::Resettable for R32PbOut_R8SlvRdDataSpec {}
