#[doc = "Register `R8_SPI0_CTRL_MOD` reader"]
pub type R = crate::R<R8Spi0CtrlModSpec>;
#[doc = "Register `R8_SPI0_CTRL_MOD` writer"]
pub type W = crate::W<R8Spi0CtrlModSpec>;
#[doc = "Field `RB_SPI_MODE_SLAVE` reader - RW, SPI0 slave mode: 0=master or host, 1=slave or device"]
pub type RbSpiModeSlaveR = crate::BitReader;
#[doc = "Field `RB_SPI_MODE_SLAVE` writer - RW, SPI0 slave mode: 0=master or host, 1=slave or device"]
pub type RbSpiModeSlaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_ALL_CLEAR` reader - RW, force clear SPI FIFO and count"]
pub type RbSpiAllClearR = crate::BitReader;
#[doc = "Field `RB_SPI_ALL_CLEAR` writer - RW, force clear SPI FIFO and count"]
pub type RbSpiAllClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_2WIRE_MOD` reader - RW, SPI0 enable 2 wire mode for slave: 0=3wire(SCK0,MOSI,MISO), 1=2wire(SCK0,MISO=MXSX)"]
pub type RbSpi2wireModR = crate::BitReader;
#[doc = "Field `RB_SPI_2WIRE_MOD` writer - RW, SPI0 enable 2 wire mode for slave: 0=3wire(SCK0,MOSI,MISO), 1=2wire(SCK0,MISO=MXSX)"]
pub type RbSpi2wireModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_MST_SCK_MOD__RB_SPI_SLV_CMD_MOD` reader - RW, SPI master clock mode: 0=mode 0, 1=mode 3;RW, SPI0 slave command mode: 0=byte stream, 1=first byte command"]
pub type RbSpiMstSckMod_RbSpiSlvCmdModR = crate::BitReader;
#[doc = "Field `RB_SPI_MST_SCK_MOD__RB_SPI_SLV_CMD_MOD` writer - RW, SPI master clock mode: 0=mode 0, 1=mode 3;RW, SPI0 slave command mode: 0=byte stream, 1=first byte command"]
pub type RbSpiMstSckMod_RbSpiSlvCmdModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_FIFO_DIR` reader - RW, SPI FIFO direction: 0=out(write @master mode), 1=in(read @master mode)"]
pub type RbSpiFifoDirR = crate::BitReader;
#[doc = "Field `RB_SPI_FIFO_DIR` writer - RW, SPI FIFO direction: 0=out(write @master mode), 1=in(read @master mode)"]
pub type RbSpiFifoDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_SCK_OE` reader - RW, SPI SCK output enable"]
pub type RbSpiSckOeR = crate::BitReader;
#[doc = "Field `RB_SPI_SCK_OE` writer - RW, SPI SCK output enable"]
pub type RbSpiSckOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_MOSI_OE` reader - RW, SPI MOSI output enable"]
pub type RbSpiMosiOeR = crate::BitReader;
#[doc = "Field `RB_SPI_MOSI_OE` writer - RW, SPI MOSI output enable"]
pub type RbSpiMosiOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_MISO_OE` reader - RW, SPI MISO output enable"]
pub type RbSpiMisoOeR = crate::BitReader;
#[doc = "Field `RB_SPI_MISO_OE` writer - RW, SPI MISO output enable"]
pub type RbSpiMisoOeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RW, SPI0 slave mode: 0=master or host, 1=slave or device"]
    #[inline(always)]
    pub fn rb_spi_mode_slave(&self) -> RbSpiModeSlaveR {
        RbSpiModeSlaveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW, force clear SPI FIFO and count"]
    #[inline(always)]
    pub fn rb_spi_all_clear(&self) -> RbSpiAllClearR {
        RbSpiAllClearR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RW, SPI0 enable 2 wire mode for slave: 0=3wire(SCK0,MOSI,MISO), 1=2wire(SCK0,MISO=MXSX)"]
    #[inline(always)]
    pub fn rb_spi_2wire_mod(&self) -> RbSpi2wireModR {
        RbSpi2wireModR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RW, SPI master clock mode: 0=mode 0, 1=mode 3;RW, SPI0 slave command mode: 0=byte stream, 1=first byte command"]
    #[inline(always)]
    pub fn rb_spi_mst_sck_mod__rb_spi_slv_cmd_mod(&self) -> RbSpiMstSckMod_RbSpiSlvCmdModR {
        RbSpiMstSckMod_RbSpiSlvCmdModR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RW, SPI FIFO direction: 0=out(write @master mode), 1=in(read @master mode)"]
    #[inline(always)]
    pub fn rb_spi_fifo_dir(&self) -> RbSpiFifoDirR {
        RbSpiFifoDirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RW, SPI SCK output enable"]
    #[inline(always)]
    pub fn rb_spi_sck_oe(&self) -> RbSpiSckOeR {
        RbSpiSckOeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RW, SPI MOSI output enable"]
    #[inline(always)]
    pub fn rb_spi_mosi_oe(&self) -> RbSpiMosiOeR {
        RbSpiMosiOeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RW, SPI MISO output enable"]
    #[inline(always)]
    pub fn rb_spi_miso_oe(&self) -> RbSpiMisoOeR {
        RbSpiMisoOeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW, SPI0 slave mode: 0=master or host, 1=slave or device"]
    #[inline(always)]
    pub fn rb_spi_mode_slave(&mut self) -> RbSpiModeSlaveW<'_, R8Spi0CtrlModSpec> {
        RbSpiModeSlaveW::new(self, 0)
    }
    #[doc = "Bit 1 - RW, force clear SPI FIFO and count"]
    #[inline(always)]
    pub fn rb_spi_all_clear(&mut self) -> RbSpiAllClearW<'_, R8Spi0CtrlModSpec> {
        RbSpiAllClearW::new(self, 1)
    }
    #[doc = "Bit 2 - RW, SPI0 enable 2 wire mode for slave: 0=3wire(SCK0,MOSI,MISO), 1=2wire(SCK0,MISO=MXSX)"]
    #[inline(always)]
    pub fn rb_spi_2wire_mod(&mut self) -> RbSpi2wireModW<'_, R8Spi0CtrlModSpec> {
        RbSpi2wireModW::new(self, 2)
    }
    #[doc = "Bit 3 - RW, SPI master clock mode: 0=mode 0, 1=mode 3;RW, SPI0 slave command mode: 0=byte stream, 1=first byte command"]
    #[inline(always)]
    pub fn rb_spi_mst_sck_mod__rb_spi_slv_cmd_mod(
        &mut self,
    ) -> RbSpiMstSckMod_RbSpiSlvCmdModW<'_, R8Spi0CtrlModSpec> {
        RbSpiMstSckMod_RbSpiSlvCmdModW::new(self, 3)
    }
    #[doc = "Bit 4 - RW, SPI FIFO direction: 0=out(write @master mode), 1=in(read @master mode)"]
    #[inline(always)]
    pub fn rb_spi_fifo_dir(&mut self) -> RbSpiFifoDirW<'_, R8Spi0CtrlModSpec> {
        RbSpiFifoDirW::new(self, 4)
    }
    #[doc = "Bit 5 - RW, SPI SCK output enable"]
    #[inline(always)]
    pub fn rb_spi_sck_oe(&mut self) -> RbSpiSckOeW<'_, R8Spi0CtrlModSpec> {
        RbSpiSckOeW::new(self, 5)
    }
    #[doc = "Bit 6 - RW, SPI MOSI output enable"]
    #[inline(always)]
    pub fn rb_spi_mosi_oe(&mut self) -> RbSpiMosiOeW<'_, R8Spi0CtrlModSpec> {
        RbSpiMosiOeW::new(self, 6)
    }
    #[doc = "Bit 7 - RW, SPI MISO output enable"]
    #[inline(always)]
    pub fn rb_spi_miso_oe(&mut self) -> RbSpiMisoOeW<'_, R8Spi0CtrlModSpec> {
        RbSpiMisoOeW::new(self, 7)
    }
}
#[doc = "RW, SPI0 mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_ctrl_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_ctrl_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi0CtrlModSpec;
impl crate::RegisterSpec for R8Spi0CtrlModSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi0_ctrl_mod::R`](R) reader structure"]
impl crate::Readable for R8Spi0CtrlModSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi0_ctrl_mod::W`](W) writer structure"]
impl crate::Writable for R8Spi0CtrlModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI0_CTRL_MOD to value 0x02"]
impl crate::Resettable for R8Spi0CtrlModSpec {
    const RESET_VALUE: u8 = 0x02;
}
