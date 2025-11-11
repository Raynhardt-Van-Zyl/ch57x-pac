#[doc = "Register `R8_UART3_LCR` reader"]
pub type R = crate::R<R8Uart3LcrSpec>;
#[doc = "Register `R8_UART3_LCR` writer"]
pub type W = crate::W<R8Uart3LcrSpec>;
#[doc = "Field `RB_LCR_WORD_SZ` reader - RW, UART word bit length: 00-5bit, 01-6bit, 10-7bit, 11-8bit"]
pub type RbLcrWordSzR = crate::FieldReader;
#[doc = "Field `RB_LCR_WORD_SZ` writer - RW, UART word bit length: 00-5bit, 01-6bit, 10-7bit, 11-8bit"]
pub type RbLcrWordSzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_LCR_STOP_BIT` reader - RW, UART stop bit length: 0-1bit, 1-2bit"]
pub type RbLcrStopBitR = crate::BitReader;
#[doc = "Field `RB_LCR_STOP_BIT` writer - RW, UART stop bit length: 0-1bit, 1-2bit"]
pub type RbLcrStopBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_LCR_PAR_EN` reader - RW, UART parity enable"]
pub type RbLcrParEnR = crate::BitReader;
#[doc = "Field `RB_LCR_PAR_EN` writer - RW, UART parity enable"]
pub type RbLcrParEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_LCR_PAR_MOD` reader - RW, UART parity mode: 00-odd, 01-even, 10-mark, 11-space"]
pub type RbLcrParModR = crate::FieldReader;
#[doc = "Field `RB_LCR_PAR_MOD` writer - RW, UART parity mode: 00-odd, 01-even, 10-mark, 11-space"]
pub type RbLcrParModW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_LCR_BREAK_EN` reader - RW, UART break control enable"]
pub type RbLcrBreakEnR = crate::BitReader;
#[doc = "Field `RB_LCR_BREAK_EN` writer - RW, UART break control enable"]
pub type RbLcrBreakEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_LCR_GP_BIT__RB_LCR_DLAB` reader - RW, UART general purpose bit;RW, UART reserved bit"]
pub type RbLcrGpBit_RbLcrDlabR = crate::BitReader;
#[doc = "Field `RB_LCR_GP_BIT__RB_LCR_DLAB` writer - RW, UART general purpose bit;RW, UART reserved bit"]
pub type RbLcrGpBit_RbLcrDlabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - RW, UART word bit length: 00-5bit, 01-6bit, 10-7bit, 11-8bit"]
    #[inline(always)]
    pub fn rb_lcr_word_sz(&self) -> RbLcrWordSzR {
        RbLcrWordSzR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - RW, UART stop bit length: 0-1bit, 1-2bit"]
    #[inline(always)]
    pub fn rb_lcr_stop_bit(&self) -> RbLcrStopBitR {
        RbLcrStopBitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RW, UART parity enable"]
    #[inline(always)]
    pub fn rb_lcr_par_en(&self) -> RbLcrParEnR {
        RbLcrParEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - RW, UART parity mode: 00-odd, 01-even, 10-mark, 11-space"]
    #[inline(always)]
    pub fn rb_lcr_par_mod(&self) -> RbLcrParModR {
        RbLcrParModR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - RW, UART break control enable"]
    #[inline(always)]
    pub fn rb_lcr_break_en(&self) -> RbLcrBreakEnR {
        RbLcrBreakEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RW, UART general purpose bit;RW, UART reserved bit"]
    #[inline(always)]
    pub fn rb_lcr_gp_bit__rb_lcr_dlab(&self) -> RbLcrGpBit_RbLcrDlabR {
        RbLcrGpBit_RbLcrDlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - RW, UART word bit length: 00-5bit, 01-6bit, 10-7bit, 11-8bit"]
    #[inline(always)]
    pub fn rb_lcr_word_sz(&mut self) -> RbLcrWordSzW<'_, R8Uart3LcrSpec> {
        RbLcrWordSzW::new(self, 0)
    }
    #[doc = "Bit 2 - RW, UART stop bit length: 0-1bit, 1-2bit"]
    #[inline(always)]
    pub fn rb_lcr_stop_bit(&mut self) -> RbLcrStopBitW<'_, R8Uart3LcrSpec> {
        RbLcrStopBitW::new(self, 2)
    }
    #[doc = "Bit 3 - RW, UART parity enable"]
    #[inline(always)]
    pub fn rb_lcr_par_en(&mut self) -> RbLcrParEnW<'_, R8Uart3LcrSpec> {
        RbLcrParEnW::new(self, 3)
    }
    #[doc = "Bits 4:5 - RW, UART parity mode: 00-odd, 01-even, 10-mark, 11-space"]
    #[inline(always)]
    pub fn rb_lcr_par_mod(&mut self) -> RbLcrParModW<'_, R8Uart3LcrSpec> {
        RbLcrParModW::new(self, 4)
    }
    #[doc = "Bit 6 - RW, UART break control enable"]
    #[inline(always)]
    pub fn rb_lcr_break_en(&mut self) -> RbLcrBreakEnW<'_, R8Uart3LcrSpec> {
        RbLcrBreakEnW::new(self, 6)
    }
    #[doc = "Bit 7 - RW, UART general purpose bit;RW, UART reserved bit"]
    #[inline(always)]
    pub fn rb_lcr_gp_bit__rb_lcr_dlab(&mut self) -> RbLcrGpBit_RbLcrDlabW<'_, R8Uart3LcrSpec> {
        RbLcrGpBit_RbLcrDlabW::new(self, 7)
    }
}
#[doc = "RW, UART3 line control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart3LcrSpec;
impl crate::RegisterSpec for R8Uart3LcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart3_lcr::R`](R) reader structure"]
impl crate::Readable for R8Uart3LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uart3_lcr::W`](W) writer structure"]
impl crate::Writable for R8Uart3LcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART3_LCR to value 0"]
impl crate::Resettable for R8Uart3LcrSpec {}
