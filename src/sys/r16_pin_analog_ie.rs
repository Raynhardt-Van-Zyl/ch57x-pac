#[doc = "Register `R16_PIN_ANALOG_IE` reader"]
pub type R = crate::R<R16PinAnalogIeSpec>;
#[doc = "Register `R16_PIN_ANALOG_IE` writer"]
pub type W = crate::W<R16PinAnalogIeSpec>;
#[doc = "Field `RB_PIN_USB_DP_PU` reader - RW,USB UDP internal pullup resistance enable"]
pub type RbPinUsbDpPuR = crate::BitReader;
#[doc = "Field `RB_PIN_USB_DP_PU` writer - RW,USB UDP internal pullup resistance enable"]
pub type RbPinUsbDpPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_USB_IE` reader - RW, USB analog I/O enable: 0=analog I/O disable, 1=analog I/O enable"]
pub type RbPinUsbIeR = crate::BitReader;
#[doc = "Field `RB_PIN_USB_IE` writer - RW, USB analog I/O enable: 0=analog I/O disable, 1=analog I/O enable"]
pub type RbPinUsbIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_ADC8_9_IE` reader - RW, ADC/TouchKey channel 9/8 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc8_9IeR = crate::BitReader;
#[doc = "Field `RB_PIN_ADC8_9_IE` writer - RW, ADC/TouchKey channel 9/8 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc8_9IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_ADC0_IE` reader - RW, ADC/TouchKey channel0 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc0IeR = crate::BitReader;
#[doc = "Field `RB_PIN_ADC0_IE` writer - RW, ADC/TouchKey channel0 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc0IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_ADC1_IE` reader - RW, ADC/TouchKey channel1 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc1IeR = crate::BitReader;
#[doc = "Field `RB_PIN_ADC1_IE` writer - RW, ADC/TouchKey channel1 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc1IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_ADC12_IE` reader - RW, ADC/TouchKey channel12 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc12IeR = crate::BitReader;
#[doc = "Field `RB_PIN_ADC12_IE` writer - RW, ADC/TouchKey channel12 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc12IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_ADC13_IE` reader - RW, ADC/TouchKey channel13 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc13IeR = crate::BitReader;
#[doc = "Field `RB_PIN_ADC13_IE` writer - RW, ADC/TouchKey channel13 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc13IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_XT32K_IE` reader - RW, external 32KHz oscillator digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinXt32kIeR = crate::BitReader;
#[doc = "Field `RB_PIN_XT32K_IE` writer - RW, external 32KHz oscillator digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinXt32kIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_ADC2_3_IE` reader - RW, ADC/TouchKey channel 2/3 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc2_3IeR = crate::BitReader;
#[doc = "Field `RB_PIN_ADC2_3_IE` writer - RW, ADC/TouchKey channel 2/3 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc2_3IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_ADC4_5_IE` reader - RW, ADC/TouchKey channel 4/5 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc4_5IeR = crate::BitReader;
#[doc = "Field `RB_PIN_ADC4_5_IE` writer - RW, ADC/TouchKey channel 4/5 digital input disable: 0=digital input enable, 1=digital input disable"]
pub type RbPinAdc4_5IeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - RW,USB UDP internal pullup resistance enable"]
    #[inline(always)]
    pub fn rb_pin_usb_dp_pu(&self) -> RbPinUsbDpPuR {
        RbPinUsbDpPuR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RW, USB analog I/O enable: 0=analog I/O disable, 1=analog I/O enable"]
    #[inline(always)]
    pub fn rb_pin_usb_ie(&self) -> RbPinUsbIeR {
        RbPinUsbIeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RW, ADC/TouchKey channel 9/8 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc8_9_ie(&self) -> RbPinAdc8_9IeR {
        RbPinAdc8_9IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RW, ADC/TouchKey channel0 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc0_ie(&self) -> RbPinAdc0IeR {
        RbPinAdc0IeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RW, ADC/TouchKey channel1 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc1_ie(&self) -> RbPinAdc1IeR {
        RbPinAdc1IeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RW, ADC/TouchKey channel12 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc12_ie(&self) -> RbPinAdc12IeR {
        RbPinAdc12IeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RW, ADC/TouchKey channel13 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc13_ie(&self) -> RbPinAdc13IeR {
        RbPinAdc13IeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RW, external 32KHz oscillator digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_xt32k_ie(&self) -> RbPinXt32kIeR {
        RbPinXt32kIeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RW, ADC/TouchKey channel 2/3 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc2_3_ie(&self) -> RbPinAdc2_3IeR {
        RbPinAdc2_3IeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RW, ADC/TouchKey channel 4/5 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc4_5_ie(&self) -> RbPinAdc4_5IeR {
        RbPinAdc4_5IeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - RW,USB UDP internal pullup resistance enable"]
    #[inline(always)]
    pub fn rb_pin_usb_dp_pu(&mut self) -> RbPinUsbDpPuW<'_, R16PinAnalogIeSpec> {
        RbPinUsbDpPuW::new(self, 6)
    }
    #[doc = "Bit 7 - RW, USB analog I/O enable: 0=analog I/O disable, 1=analog I/O enable"]
    #[inline(always)]
    pub fn rb_pin_usb_ie(&mut self) -> RbPinUsbIeW<'_, R16PinAnalogIeSpec> {
        RbPinUsbIeW::new(self, 7)
    }
    #[doc = "Bit 8 - RW, ADC/TouchKey channel 9/8 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc8_9_ie(&mut self) -> RbPinAdc8_9IeW<'_, R16PinAnalogIeSpec> {
        RbPinAdc8_9IeW::new(self, 8)
    }
    #[doc = "Bit 9 - RW, ADC/TouchKey channel0 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc0_ie(&mut self) -> RbPinAdc0IeW<'_, R16PinAnalogIeSpec> {
        RbPinAdc0IeW::new(self, 9)
    }
    #[doc = "Bit 10 - RW, ADC/TouchKey channel1 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc1_ie(&mut self) -> RbPinAdc1IeW<'_, R16PinAnalogIeSpec> {
        RbPinAdc1IeW::new(self, 10)
    }
    #[doc = "Bit 11 - RW, ADC/TouchKey channel12 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc12_ie(&mut self) -> RbPinAdc12IeW<'_, R16PinAnalogIeSpec> {
        RbPinAdc12IeW::new(self, 11)
    }
    #[doc = "Bit 12 - RW, ADC/TouchKey channel13 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc13_ie(&mut self) -> RbPinAdc13IeW<'_, R16PinAnalogIeSpec> {
        RbPinAdc13IeW::new(self, 12)
    }
    #[doc = "Bit 13 - RW, external 32KHz oscillator digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_xt32k_ie(&mut self) -> RbPinXt32kIeW<'_, R16PinAnalogIeSpec> {
        RbPinXt32kIeW::new(self, 13)
    }
    #[doc = "Bit 14 - RW, ADC/TouchKey channel 2/3 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc2_3_ie(&mut self) -> RbPinAdc2_3IeW<'_, R16PinAnalogIeSpec> {
        RbPinAdc2_3IeW::new(self, 14)
    }
    #[doc = "Bit 15 - RW, ADC/TouchKey channel 4/5 digital input disable: 0=digital input enable, 1=digital input disable"]
    #[inline(always)]
    pub fn rb_pin_adc4_5_ie(&mut self) -> RbPinAdc4_5IeW<'_, R16PinAnalogIeSpec> {
        RbPinAdc4_5IeW::new(self, 15)
    }
}
#[doc = "RW, analog pin enable and digital input disable\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pin_analog_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pin_analog_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16PinAnalogIeSpec;
impl crate::RegisterSpec for R16PinAnalogIeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_pin_analog_ie::R`](R) reader structure"]
impl crate::Readable for R16PinAnalogIeSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_pin_analog_ie::W`](W) writer structure"]
impl crate::Writable for R16PinAnalogIeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_PIN_ANALOG_IE to value 0"]
impl crate::Resettable for R16PinAnalogIeSpec {}
