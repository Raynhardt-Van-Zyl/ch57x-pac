#[doc = "Register `R32_PB_DIR` reader"]
pub type R = crate::R<R32PbDirSpec>;
#[doc = "Register `R32_PB_DIR` writer"]
pub type W = crate::W<R32PbDirSpec>;
#[doc = "Field `R8_PB_DIR_0` reader - GPIO PB I/O direction byte 0"]
pub type R8PbDir0R = crate::FieldReader;
#[doc = "Field `R8_PB_DIR_0` writer - GPIO PB I/O direction byte 0"]
pub type R8PbDir0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PB_DIR_1` reader - GPIO PB I/O direction byte 1"]
pub type R8PbDir1R = crate::FieldReader;
#[doc = "Field `R8_PB_DIR_1` writer - GPIO PB I/O direction byte 1"]
pub type R8PbDir1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PB_DIR_2` reader - GPIO PB I/O direction byte 2"]
pub type R8PbDir2R = crate::FieldReader;
#[doc = "Field `R8_PB_DIR_2` writer - GPIO PB I/O direction byte 2"]
pub type R8PbDir2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO PB I/O direction byte 0"]
    #[inline(always)]
    pub fn r8_pb_dir_0(&self) -> R8PbDir0R {
        R8PbDir0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO PB I/O direction byte 1"]
    #[inline(always)]
    pub fn r8_pb_dir_1(&self) -> R8PbDir1R {
        R8PbDir1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO PB I/O direction byte 2"]
    #[inline(always)]
    pub fn r8_pb_dir_2(&self) -> R8PbDir2R {
        R8PbDir2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO PB I/O direction byte 0"]
    #[inline(always)]
    pub fn r8_pb_dir_0(&mut self) -> R8PbDir0W<'_, R32PbDirSpec> {
        R8PbDir0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO PB I/O direction byte 1"]
    #[inline(always)]
    pub fn r8_pb_dir_1(&mut self) -> R8PbDir1W<'_, R32PbDirSpec> {
        R8PbDir1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO PB I/O direction byte 2"]
    #[inline(always)]
    pub fn r8_pb_dir_2(&mut self) -> R8PbDir2W<'_, R32PbDirSpec> {
        R8PbDir2W::new(self, 16)
    }
}
#[doc = "RW, GPIO PB I/O direction: 0=in, 1=out\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbDirSpec;
impl crate::RegisterSpec for R32PbDirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_dir::R`](R) reader structure"]
impl crate::Readable for R32PbDirSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pb_dir::W`](W) writer structure"]
impl crate::Writable for R32PbDirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PB_DIR to value 0"]
impl crate::Resettable for R32PbDirSpec {}
