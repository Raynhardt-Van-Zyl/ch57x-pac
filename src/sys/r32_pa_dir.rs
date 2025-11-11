#[doc = "Register `R32_PA_DIR` reader"]
pub type R = crate::R<R32PaDirSpec>;
#[doc = "Register `R32_PA_DIR` writer"]
pub type W = crate::W<R32PaDirSpec>;
#[doc = "Field `R8_PA_DIR_0` reader - GPIO PA I/O direction byte 0"]
pub type R8PaDir0R = crate::FieldReader;
#[doc = "Field `R8_PA_DIR_0` writer - GPIO PA I/O direction byte 0"]
pub type R8PaDir0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PA_DIR_1` reader - GPIO PA I/O direction byte 1"]
pub type R8PaDir1R = crate::FieldReader;
#[doc = "Field `R8_PA_DIR_1` writer - GPIO PA I/O direction byte 1"]
pub type R8PaDir1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO PA I/O direction byte 0"]
    #[inline(always)]
    pub fn r8_pa_dir_0(&self) -> R8PaDir0R {
        R8PaDir0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO PA I/O direction byte 1"]
    #[inline(always)]
    pub fn r8_pa_dir_1(&self) -> R8PaDir1R {
        R8PaDir1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO PA I/O direction byte 0"]
    #[inline(always)]
    pub fn r8_pa_dir_0(&mut self) -> R8PaDir0W<'_, R32PaDirSpec> {
        R8PaDir0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO PA I/O direction byte 1"]
    #[inline(always)]
    pub fn r8_pa_dir_1(&mut self) -> R8PaDir1W<'_, R32PaDirSpec> {
        R8PaDir1W::new(self, 8)
    }
}
#[doc = "RW, GPIO PA I/O direction: 0=in, 1=out\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaDirSpec;
impl crate::RegisterSpec for R32PaDirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_dir::R`](R) reader structure"]
impl crate::Readable for R32PaDirSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pa_dir::W`](W) writer structure"]
impl crate::Writable for R32PaDirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_DIR to value 0"]
impl crate::Resettable for R32PaDirSpec {}
