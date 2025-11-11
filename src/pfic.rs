#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r32_pfic_isr1: R32PficIsr1,
    r32_pfic_isr2: R32PficIsr2,
    _reserved2: [u8; 0x18],
    r32_pfic_ipr1: R32PficIpr1,
    r32_pfic_ipr2: R32PficIpr2,
    _reserved4: [u8; 0x18],
    r32_pfic_ithresdr: R32PficIthresdr,
    r32_pfic_fibaddrr: R32PficFibaddrr,
    r32_pfic_cfgr: R32PficCfgr,
    r32_pfic_gisr: R32PficGisr,
    _reserved8: [u8; 0x10],
    r32_pfic_fifoaddrr0: R32PficFifoaddrr0,
    r32_pfic_fifoaddrr1: R32PficFifoaddrr1,
    r32_pfic_fifoaddrr2: R32PficFifoaddrr2,
    r32_pfic_fifoaddrr3: R32PficFifoaddrr3,
    _reserved12: [u8; 0x90],
    r32_pfic_ienr1: R32PficIenr1,
    r32_pfic_ienr2: R32PficIenr2,
    _reserved14: [u8; 0x78],
    r32_pfic_irer1: R32PficIrer1,
    r32_pfic_irer2: R32PficIrer2,
    _reserved16: [u8; 0x78],
    r32_pfic_ipsr1: R32PficIpsr1,
    r32_pfic_ipsr2: R32PficIpsr2,
    _reserved18: [u8; 0x78],
    r32_pfic_iprr1: R32PficIprr1,
    r32_pfic_iprr2: R32PficIprr2,
    _reserved20: [u8; 0x78],
    r32_pfic_iactr1: R32PficIactr1,
    r32_pfic_iactr2: R32PficIactr2,
    _reserved22: [u8; 0xf8],
    r32_pfic_iprior0: R32PficIprior0,
    _reserved23: [u8; 0x1c],
    r32_pfic_iprior1: R32PficIprior1,
    _reserved24: [u8; 0x1c],
    r32_pfic_iprior2: R32PficIprior2,
    _reserved25: [u8; 0x1c],
    r32_pfic_iprior3: R32PficIprior3,
    _reserved26: [u8; 0x1c],
    r32_pfic_iprior4: R32PficIprior4,
    _reserved27: [u8; 0x1c],
    r32_pfic_iprior5: R32PficIprior5,
    _reserved28: [u8; 0x1c],
    r32_pfic_iprior6: R32PficIprior6,
    _reserved29: [u8; 0x1c],
    r32_pfic_iprior7: R32PficIprior7,
    _reserved30: [u8; 0x1c],
    r32_pfic_iprior8: R32PficIprior8,
    _reserved31: [u8; 0x1c],
    r32_pfic_iprior9: R32PficIprior9,
    _reserved32: [u8; 0x1c],
    r32_pfic_iprior10: R32PficIprior10,
    _reserved33: [u8; 0x1c],
    r32_pfic_iprior11: R32PficIprior11,
    _reserved34: [u8; 0x1c],
    r32_pfic_iprior12: R32PficIprior12,
    _reserved35: [u8; 0x1c],
    r32_pfic_iprior13: R32PficIprior13,
    _reserved36: [u8; 0x1c],
    r32_pfic_iprior14: R32PficIprior14,
    _reserved37: [u8; 0x1c],
    r32_pfic_iprior15: R32PficIprior15,
    _reserved38: [u8; 0x1c],
    r32_pfic_iprior16: R32PficIprior16,
    _reserved39: [u8; 0x1c],
    r32_pfic_iprior17: R32PficIprior17,
    _reserved40: [u8; 0x1c],
    r32_pfic_iprior18: R32PficIprior18,
    _reserved41: [u8; 0x1c],
    r32_pfic_iprior19: R32PficIprior19,
    _reserved42: [u8; 0x1c],
    r32_pfic_iprior20: R32PficIprior20,
    _reserved43: [u8; 0x1c],
    r32_pfic_iprior21: R32PficIprior21,
    _reserved44: [u8; 0x1c],
    r32_pfic_iprior22: R32PficIprior22,
    _reserved45: [u8; 0x1c],
    r32_pfic_iprior23: R32PficIprior23,
    _reserved46: [u8; 0x1c],
    r32_pfic_iprior24: R32PficIprior24,
    _reserved47: [u8; 0x1c],
    r32_pfic_iprior25: R32PficIprior25,
    _reserved48: [u8; 0x1c],
    r32_pfic_iprior26: R32PficIprior26,
    _reserved49: [u8; 0x1c],
    r32_pfic_iprior27: R32PficIprior27,
    _reserved50: [u8; 0x1c],
    r32_pfic_iprior28: R32PficIprior28,
    _reserved51: [u8; 0x1c],
    r32_pfic_iprior29: R32PficIprior29,
    _reserved52: [u8; 0x1c],
    r32_pfic_iprior30: R32PficIprior30,
    _reserved53: [u8; 0x1c],
    r32_pfic_iprior31: R32PficIprior31,
    _reserved54: [u8; 0x1c],
    r32_pfic_iprior32: R32PficIprior32,
    _reserved55: [u8; 0x1c],
    r32_pfic_iprior33: R32PficIprior33,
    _reserved56: [u8; 0x1c],
    r32_pfic_iprior34: R32PficIprior34,
    _reserved57: [u8; 0x1c],
    r32_pfic_iprior35: R32PficIprior35,
    _reserved58: [u8; 0x1c],
    r32_pfic_iprior36: R32PficIprior36,
    _reserved59: [u8; 0x1c],
    r32_pfic_iprior37: R32PficIprior37,
    _reserved60: [u8; 0x1c],
    r32_pfic_iprior38: R32PficIprior38,
    _reserved61: [u8; 0x1c],
    r32_pfic_iprior39: R32PficIprior39,
    _reserved62: [u8; 0x1c],
    r32_pfic_iprior40: R32PficIprior40,
    _reserved63: [u8; 0x1c],
    r32_pfic_iprior41: R32PficIprior41,
    _reserved64: [u8; 0x1c],
    r32_pfic_iprior42: R32PficIprior42,
    _reserved65: [u8; 0x1c],
    r32_pfic_iprior43: R32PficIprior43,
    _reserved66: [u8; 0x1c],
    r32_pfic_iprior44: R32PficIprior44,
    _reserved67: [u8; 0x1c],
    r32_pfic_iprior45: R32PficIprior45,
    _reserved68: [u8; 0x1c],
    r32_pfic_iprior46: R32PficIprior46,
    _reserved69: [u8; 0x1c],
    r32_pfic_iprior47: R32PficIprior47,
    _reserved70: [u8; 0x1c],
    r32_pfic_iprior48: R32PficIprior48,
    _reserved71: [u8; 0x1c],
    r32_pfic_iprior49: R32PficIprior49,
    _reserved72: [u8; 0x1c],
    r32_pfic_iprior50: R32PficIprior50,
    _reserved73: [u8; 0x1c],
    r32_pfic_iprior51: R32PficIprior51,
    _reserved74: [u8; 0x1c],
    r32_pfic_iprior52: R32PficIprior52,
    _reserved75: [u8; 0x1c],
    r32_pfic_iprior53: R32PficIprior53,
    _reserved76: [u8; 0x2c],
    r32_pfic_iprior54: R32PficIprior54,
    _reserved77: [u8; 0x0c],
    r32_pfic_iprior55: R32PficIprior55,
    _reserved78: [u8; 0x1c],
    r32_pfic_iprior56: R32PficIprior56,
    _reserved79: [u8; 0x1c],
    r32_pfic_iprior57: R32PficIprior57,
    _reserved80: [u8; 0x1c],
    r32_pfic_iprior58: R32PficIprior58,
    _reserved81: [u8; 0x1c],
    r32_pfic_iprior59: R32PficIprior59,
    _reserved82: [u8; 0x1c],
    r32_pfic_iprior60: R32PficIprior60,
    _reserved83: [u8; 0x1c],
    r32_pfic_iprior61: R32PficIprior61,
    _reserved84: [u8; 0x3c],
    r32_pfic_iprior62: R32PficIprior62,
    _reserved85: [u8; 0x1c],
    r32_pfic_iprior63: R32PficIprior63,
    _reserved86: [u8; 0x010c],
    r32_pfic_sctlr: R32PficSctlr,
    r32_pfic_vtctlr: R32PficVtctlr,
}
impl RegisterBlock {
    #[doc = "0x00 - RO,Interrupt Status Register"]
    #[inline(always)]
    pub const fn r32_pfic_isr1(&self) -> &R32PficIsr1 {
        &self.r32_pfic_isr1
    }
    #[doc = "0x04 - RO,Interrupt Status Register"]
    #[inline(always)]
    pub const fn r32_pfic_isr2(&self) -> &R32PficIsr2 {
        &self.r32_pfic_isr2
    }
    #[doc = "0x20 - RO,Interrupt Pending Register"]
    #[inline(always)]
    pub const fn r32_pfic_ipr1(&self) -> &R32PficIpr1 {
        &self.r32_pfic_ipr1
    }
    #[doc = "0x24 - RO,Interrupt Pending Register"]
    #[inline(always)]
    pub const fn r32_pfic_ipr2(&self) -> &R32PficIpr2 {
        &self.r32_pfic_ipr2
    }
    #[doc = "0x40 - RW,Interrupt Priority Register"]
    #[inline(always)]
    pub const fn r32_pfic_ithresdr(&self) -> &R32PficIthresdr {
        &self.r32_pfic_ithresdr
    }
    #[doc = "0x44 - RW,Interrupt Fast Address Register"]
    #[inline(always)]
    pub const fn r32_pfic_fibaddrr(&self) -> &R32PficFibaddrr {
        &self.r32_pfic_fibaddrr
    }
    #[doc = "0x48 - Interrupt Config Register"]
    #[inline(always)]
    pub const fn r32_pfic_cfgr(&self) -> &R32PficCfgr {
        &self.r32_pfic_cfgr
    }
    #[doc = "0x4c - Interrupt Global Register"]
    #[inline(always)]
    pub const fn r32_pfic_gisr(&self) -> &R32PficGisr {
        &self.r32_pfic_gisr
    }
    #[doc = "0x60 - Interrupt 0 address Register"]
    #[inline(always)]
    pub const fn r32_pfic_fifoaddrr0(&self) -> &R32PficFifoaddrr0 {
        &self.r32_pfic_fifoaddrr0
    }
    #[doc = "0x64 - Interrupt 1 address Register"]
    #[inline(always)]
    pub const fn r32_pfic_fifoaddrr1(&self) -> &R32PficFifoaddrr1 {
        &self.r32_pfic_fifoaddrr1
    }
    #[doc = "0x68 - Interrupt 2 address Register"]
    #[inline(always)]
    pub const fn r32_pfic_fifoaddrr2(&self) -> &R32PficFifoaddrr2 {
        &self.r32_pfic_fifoaddrr2
    }
    #[doc = "0x6c - Interrupt 3 address Register"]
    #[inline(always)]
    pub const fn r32_pfic_fifoaddrr3(&self) -> &R32PficFifoaddrr3 {
        &self.r32_pfic_fifoaddrr3
    }
    #[doc = "0x100 - Interrupt Setting Register"]
    #[inline(always)]
    pub const fn r32_pfic_ienr1(&self) -> &R32PficIenr1 {
        &self.r32_pfic_ienr1
    }
    #[doc = "0x104 - Interrupt Setting Register"]
    #[inline(always)]
    pub const fn r32_pfic_ienr2(&self) -> &R32PficIenr2 {
        &self.r32_pfic_ienr2
    }
    #[doc = "0x180 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn r32_pfic_irer1(&self) -> &R32PficIrer1 {
        &self.r32_pfic_irer1
    }
    #[doc = "0x184 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn r32_pfic_irer2(&self) -> &R32PficIrer2 {
        &self.r32_pfic_irer2
    }
    #[doc = "0x200 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn r32_pfic_ipsr1(&self) -> &R32PficIpsr1 {
        &self.r32_pfic_ipsr1
    }
    #[doc = "0x204 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn r32_pfic_ipsr2(&self) -> &R32PficIpsr2 {
        &self.r32_pfic_ipsr2
    }
    #[doc = "0x280 - Interrupt Pending Clear Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprr1(&self) -> &R32PficIprr1 {
        &self.r32_pfic_iprr1
    }
    #[doc = "0x284 - Interrupt Pending Clear Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprr2(&self) -> &R32PficIprr2 {
        &self.r32_pfic_iprr2
    }
    #[doc = "0x300 - Interrupt ACTIVE Register"]
    #[inline(always)]
    pub const fn r32_pfic_iactr1(&self) -> &R32PficIactr1 {
        &self.r32_pfic_iactr1
    }
    #[doc = "0x304 - Interrupt ACTIVE Register"]
    #[inline(always)]
    pub const fn r32_pfic_iactr2(&self) -> &R32PficIactr2 {
        &self.r32_pfic_iactr2
    }
    #[doc = "0x400 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior0(&self) -> &R32PficIprior0 {
        &self.r32_pfic_iprior0
    }
    #[doc = "0x420 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior1(&self) -> &R32PficIprior1 {
        &self.r32_pfic_iprior1
    }
    #[doc = "0x440 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior2(&self) -> &R32PficIprior2 {
        &self.r32_pfic_iprior2
    }
    #[doc = "0x460 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior3(&self) -> &R32PficIprior3 {
        &self.r32_pfic_iprior3
    }
    #[doc = "0x480 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior4(&self) -> &R32PficIprior4 {
        &self.r32_pfic_iprior4
    }
    #[doc = "0x4a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior5(&self) -> &R32PficIprior5 {
        &self.r32_pfic_iprior5
    }
    #[doc = "0x4c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior6(&self) -> &R32PficIprior6 {
        &self.r32_pfic_iprior6
    }
    #[doc = "0x4e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior7(&self) -> &R32PficIprior7 {
        &self.r32_pfic_iprior7
    }
    #[doc = "0x500 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior8(&self) -> &R32PficIprior8 {
        &self.r32_pfic_iprior8
    }
    #[doc = "0x520 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior9(&self) -> &R32PficIprior9 {
        &self.r32_pfic_iprior9
    }
    #[doc = "0x540 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior10(&self) -> &R32PficIprior10 {
        &self.r32_pfic_iprior10
    }
    #[doc = "0x560 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior11(&self) -> &R32PficIprior11 {
        &self.r32_pfic_iprior11
    }
    #[doc = "0x580 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior12(&self) -> &R32PficIprior12 {
        &self.r32_pfic_iprior12
    }
    #[doc = "0x5a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior13(&self) -> &R32PficIprior13 {
        &self.r32_pfic_iprior13
    }
    #[doc = "0x5c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior14(&self) -> &R32PficIprior14 {
        &self.r32_pfic_iprior14
    }
    #[doc = "0x5e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior15(&self) -> &R32PficIprior15 {
        &self.r32_pfic_iprior15
    }
    #[doc = "0x600 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior16(&self) -> &R32PficIprior16 {
        &self.r32_pfic_iprior16
    }
    #[doc = "0x620 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior17(&self) -> &R32PficIprior17 {
        &self.r32_pfic_iprior17
    }
    #[doc = "0x640 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior18(&self) -> &R32PficIprior18 {
        &self.r32_pfic_iprior18
    }
    #[doc = "0x660 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior19(&self) -> &R32PficIprior19 {
        &self.r32_pfic_iprior19
    }
    #[doc = "0x680 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior20(&self) -> &R32PficIprior20 {
        &self.r32_pfic_iprior20
    }
    #[doc = "0x6a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior21(&self) -> &R32PficIprior21 {
        &self.r32_pfic_iprior21
    }
    #[doc = "0x6c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior22(&self) -> &R32PficIprior22 {
        &self.r32_pfic_iprior22
    }
    #[doc = "0x6e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior23(&self) -> &R32PficIprior23 {
        &self.r32_pfic_iprior23
    }
    #[doc = "0x700 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior24(&self) -> &R32PficIprior24 {
        &self.r32_pfic_iprior24
    }
    #[doc = "0x720 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior25(&self) -> &R32PficIprior25 {
        &self.r32_pfic_iprior25
    }
    #[doc = "0x740 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior26(&self) -> &R32PficIprior26 {
        &self.r32_pfic_iprior26
    }
    #[doc = "0x760 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior27(&self) -> &R32PficIprior27 {
        &self.r32_pfic_iprior27
    }
    #[doc = "0x780 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior28(&self) -> &R32PficIprior28 {
        &self.r32_pfic_iprior28
    }
    #[doc = "0x7a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior29(&self) -> &R32PficIprior29 {
        &self.r32_pfic_iprior29
    }
    #[doc = "0x7c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior30(&self) -> &R32PficIprior30 {
        &self.r32_pfic_iprior30
    }
    #[doc = "0x7e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior31(&self) -> &R32PficIprior31 {
        &self.r32_pfic_iprior31
    }
    #[doc = "0x800 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior32(&self) -> &R32PficIprior32 {
        &self.r32_pfic_iprior32
    }
    #[doc = "0x820 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior33(&self) -> &R32PficIprior33 {
        &self.r32_pfic_iprior33
    }
    #[doc = "0x840 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior34(&self) -> &R32PficIprior34 {
        &self.r32_pfic_iprior34
    }
    #[doc = "0x860 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior35(&self) -> &R32PficIprior35 {
        &self.r32_pfic_iprior35
    }
    #[doc = "0x880 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior36(&self) -> &R32PficIprior36 {
        &self.r32_pfic_iprior36
    }
    #[doc = "0x8a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior37(&self) -> &R32PficIprior37 {
        &self.r32_pfic_iprior37
    }
    #[doc = "0x8c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior38(&self) -> &R32PficIprior38 {
        &self.r32_pfic_iprior38
    }
    #[doc = "0x8e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior39(&self) -> &R32PficIprior39 {
        &self.r32_pfic_iprior39
    }
    #[doc = "0x900 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior40(&self) -> &R32PficIprior40 {
        &self.r32_pfic_iprior40
    }
    #[doc = "0x920 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior41(&self) -> &R32PficIprior41 {
        &self.r32_pfic_iprior41
    }
    #[doc = "0x940 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior42(&self) -> &R32PficIprior42 {
        &self.r32_pfic_iprior42
    }
    #[doc = "0x960 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior43(&self) -> &R32PficIprior43 {
        &self.r32_pfic_iprior43
    }
    #[doc = "0x980 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior44(&self) -> &R32PficIprior44 {
        &self.r32_pfic_iprior44
    }
    #[doc = "0x9a0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior45(&self) -> &R32PficIprior45 {
        &self.r32_pfic_iprior45
    }
    #[doc = "0x9c0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior46(&self) -> &R32PficIprior46 {
        &self.r32_pfic_iprior46
    }
    #[doc = "0x9e0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior47(&self) -> &R32PficIprior47 {
        &self.r32_pfic_iprior47
    }
    #[doc = "0xa00 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior48(&self) -> &R32PficIprior48 {
        &self.r32_pfic_iprior48
    }
    #[doc = "0xa20 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior49(&self) -> &R32PficIprior49 {
        &self.r32_pfic_iprior49
    }
    #[doc = "0xa40 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior50(&self) -> &R32PficIprior50 {
        &self.r32_pfic_iprior50
    }
    #[doc = "0xa60 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior51(&self) -> &R32PficIprior51 {
        &self.r32_pfic_iprior51
    }
    #[doc = "0xa80 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior52(&self) -> &R32PficIprior52 {
        &self.r32_pfic_iprior52
    }
    #[doc = "0xaa0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior53(&self) -> &R32PficIprior53 {
        &self.r32_pfic_iprior53
    }
    #[doc = "0xad0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior54(&self) -> &R32PficIprior54 {
        &self.r32_pfic_iprior54
    }
    #[doc = "0xae0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior55(&self) -> &R32PficIprior55 {
        &self.r32_pfic_iprior55
    }
    #[doc = "0xb00 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior56(&self) -> &R32PficIprior56 {
        &self.r32_pfic_iprior56
    }
    #[doc = "0xb20 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior57(&self) -> &R32PficIprior57 {
        &self.r32_pfic_iprior57
    }
    #[doc = "0xb40 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior58(&self) -> &R32PficIprior58 {
        &self.r32_pfic_iprior58
    }
    #[doc = "0xb60 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior59(&self) -> &R32PficIprior59 {
        &self.r32_pfic_iprior59
    }
    #[doc = "0xb80 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior60(&self) -> &R32PficIprior60 {
        &self.r32_pfic_iprior60
    }
    #[doc = "0xba0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior61(&self) -> &R32PficIprior61 {
        &self.r32_pfic_iprior61
    }
    #[doc = "0xbe0 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior62(&self) -> &R32PficIprior62 {
        &self.r32_pfic_iprior62
    }
    #[doc = "0xc00 - Interrupt Priority configuration Register"]
    #[inline(always)]
    pub const fn r32_pfic_iprior63(&self) -> &R32PficIprior63 {
        &self.r32_pfic_iprior63
    }
    #[doc = "0xd10 - System Control Register"]
    #[inline(always)]
    pub const fn r32_pfic_sctlr(&self) -> &R32PficSctlr {
        &self.r32_pfic_sctlr
    }
    #[doc = "0xd14 - System Control Register"]
    #[inline(always)]
    pub const fn r32_pfic_vtctlr(&self) -> &R32PficVtctlr {
        &self.r32_pfic_vtctlr
    }
}
#[doc = "R32_PFIC_ISR1 (rw) register accessor: RO,Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_isr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_isr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_isr1`] module"]
#[doc(alias = "R32_PFIC_ISR1")]
pub type R32PficIsr1 = crate::Reg<r32_pfic_isr1::R32PficIsr1Spec>;
#[doc = "RO,Interrupt Status Register"]
pub mod r32_pfic_isr1;
#[doc = "R32_PFIC_ISR2 (rw) register accessor: RO,Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_isr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_isr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_isr2`] module"]
#[doc(alias = "R32_PFIC_ISR2")]
pub type R32PficIsr2 = crate::Reg<r32_pfic_isr2::R32PficIsr2Spec>;
#[doc = "RO,Interrupt Status Register"]
pub mod r32_pfic_isr2;
#[doc = "R32_PFIC_IPR1 (rw) register accessor: RO,Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ipr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_ipr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_ipr1`] module"]
#[doc(alias = "R32_PFIC_IPR1")]
pub type R32PficIpr1 = crate::Reg<r32_pfic_ipr1::R32PficIpr1Spec>;
#[doc = "RO,Interrupt Pending Register"]
pub mod r32_pfic_ipr1;
#[doc = "R32_PFIC_IPR2 (r) register accessor: RO,Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ipr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_ipr2`] module"]
#[doc(alias = "R32_PFIC_IPR2")]
pub type R32PficIpr2 = crate::Reg<r32_pfic_ipr2::R32PficIpr2Spec>;
#[doc = "RO,Interrupt Pending Register"]
pub mod r32_pfic_ipr2;
#[doc = "R32_PFIC_ITHRESDR (rw) register accessor: RW,Interrupt Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ithresdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_ithresdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_ithresdr`] module"]
#[doc(alias = "R32_PFIC_ITHRESDR")]
pub type R32PficIthresdr = crate::Reg<r32_pfic_ithresdr::R32PficIthresdrSpec>;
#[doc = "RW,Interrupt Priority Register"]
pub mod r32_pfic_ithresdr;
#[doc = "R32_PFIC_FIBADDRR (rw) register accessor: RW,Interrupt Fast Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_fibaddrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_fibaddrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_fibaddrr`] module"]
#[doc(alias = "R32_PFIC_FIBADDRR")]
pub type R32PficFibaddrr = crate::Reg<r32_pfic_fibaddrr::R32PficFibaddrrSpec>;
#[doc = "RW,Interrupt Fast Address Register"]
pub mod r32_pfic_fibaddrr;
#[doc = "R32_PFIC_CFGR (rw) register accessor: Interrupt Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_cfgr`] module"]
#[doc(alias = "R32_PFIC_CFGR")]
pub type R32PficCfgr = crate::Reg<r32_pfic_cfgr::R32PficCfgrSpec>;
#[doc = "Interrupt Config Register"]
pub mod r32_pfic_cfgr;
#[doc = "R32_PFIC_GISR (r) register accessor: Interrupt Global Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_gisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_gisr`] module"]
#[doc(alias = "R32_PFIC_GISR")]
pub type R32PficGisr = crate::Reg<r32_pfic_gisr::R32PficGisrSpec>;
#[doc = "Interrupt Global Register"]
pub mod r32_pfic_gisr;
#[doc = "R32_PFIC_FIFOADDRR0 (rw) register accessor: Interrupt 0 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_fifoaddrr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_fifoaddrr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_fifoaddrr0`] module"]
#[doc(alias = "R32_PFIC_FIFOADDRR0")]
pub type R32PficFifoaddrr0 = crate::Reg<r32_pfic_fifoaddrr0::R32PficFifoaddrr0Spec>;
#[doc = "Interrupt 0 address Register"]
pub mod r32_pfic_fifoaddrr0;
#[doc = "R32_PFIC_FIFOADDRR1 (rw) register accessor: Interrupt 1 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_fifoaddrr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_fifoaddrr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_fifoaddrr1`] module"]
#[doc(alias = "R32_PFIC_FIFOADDRR1")]
pub type R32PficFifoaddrr1 = crate::Reg<r32_pfic_fifoaddrr1::R32PficFifoaddrr1Spec>;
#[doc = "Interrupt 1 address Register"]
pub mod r32_pfic_fifoaddrr1;
#[doc = "R32_PFIC_FIFOADDRR2 (rw) register accessor: Interrupt 2 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_fifoaddrr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_fifoaddrr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_fifoaddrr2`] module"]
#[doc(alias = "R32_PFIC_FIFOADDRR2")]
pub type R32PficFifoaddrr2 = crate::Reg<r32_pfic_fifoaddrr2::R32PficFifoaddrr2Spec>;
#[doc = "Interrupt 2 address Register"]
pub mod r32_pfic_fifoaddrr2;
#[doc = "R32_PFIC_FIFOADDRR3 (rw) register accessor: Interrupt 3 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_fifoaddrr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_fifoaddrr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_fifoaddrr3`] module"]
#[doc(alias = "R32_PFIC_FIFOADDRR3")]
pub type R32PficFifoaddrr3 = crate::Reg<r32_pfic_fifoaddrr3::R32PficFifoaddrr3Spec>;
#[doc = "Interrupt 3 address Register"]
pub mod r32_pfic_fifoaddrr3;
#[doc = "R32_PFIC_IENR1 (rw) register accessor: Interrupt Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ienr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_ienr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_ienr1`] module"]
#[doc(alias = "R32_PFIC_IENR1")]
pub type R32PficIenr1 = crate::Reg<r32_pfic_ienr1::R32PficIenr1Spec>;
#[doc = "Interrupt Setting Register"]
pub mod r32_pfic_ienr1;
#[doc = "R32_PFIC_IENR2 (rw) register accessor: Interrupt Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ienr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_ienr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_ienr2`] module"]
#[doc(alias = "R32_PFIC_IENR2")]
pub type R32PficIenr2 = crate::Reg<r32_pfic_ienr2::R32PficIenr2Spec>;
#[doc = "Interrupt Setting Register"]
pub mod r32_pfic_ienr2;
#[doc = "R32_PFIC_IRER1 (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_irer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_irer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_irer1`] module"]
#[doc(alias = "R32_PFIC_IRER1")]
pub type R32PficIrer1 = crate::Reg<r32_pfic_irer1::R32PficIrer1Spec>;
#[doc = "Interrupt Clear Register"]
pub mod r32_pfic_irer1;
#[doc = "R32_PFIC_IRER2 (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_irer2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_irer2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_irer2`] module"]
#[doc(alias = "R32_PFIC_IRER2")]
pub type R32PficIrer2 = crate::Reg<r32_pfic_irer2::R32PficIrer2Spec>;
#[doc = "Interrupt Clear Register"]
pub mod r32_pfic_irer2;
#[doc = "R32_PFIC_IPSR1 (rw) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ipsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_ipsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_ipsr1`] module"]
#[doc(alias = "R32_PFIC_IPSR1")]
pub type R32PficIpsr1 = crate::Reg<r32_pfic_ipsr1::R32PficIpsr1Spec>;
#[doc = "Interrupt Pending Register"]
pub mod r32_pfic_ipsr1;
#[doc = "R32_PFIC_IPSR2 (rw) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ipsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_ipsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_ipsr2`] module"]
#[doc(alias = "R32_PFIC_IPSR2")]
pub type R32PficIpsr2 = crate::Reg<r32_pfic_ipsr2::R32PficIpsr2Spec>;
#[doc = "Interrupt Pending Register"]
pub mod r32_pfic_ipsr2;
#[doc = "R32_PFIC_IPRR1 (rw) register accessor: Interrupt Pending Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprr1`] module"]
#[doc(alias = "R32_PFIC_IPRR1")]
pub type R32PficIprr1 = crate::Reg<r32_pfic_iprr1::R32PficIprr1Spec>;
#[doc = "Interrupt Pending Clear Register"]
pub mod r32_pfic_iprr1;
#[doc = "R32_PFIC_IPRR2 (rw) register accessor: Interrupt Pending Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprr2`] module"]
#[doc(alias = "R32_PFIC_IPRR2")]
pub type R32PficIprr2 = crate::Reg<r32_pfic_iprr2::R32PficIprr2Spec>;
#[doc = "Interrupt Pending Clear Register"]
pub mod r32_pfic_iprr2;
#[doc = "R32_PFIC_IACTR1 (rw) register accessor: Interrupt ACTIVE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iactr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iactr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iactr1`] module"]
#[doc(alias = "R32_PFIC_IACTR1")]
pub type R32PficIactr1 = crate::Reg<r32_pfic_iactr1::R32PficIactr1Spec>;
#[doc = "Interrupt ACTIVE Register"]
pub mod r32_pfic_iactr1;
#[doc = "R32_PFIC_IACTR2 (rw) register accessor: Interrupt ACTIVE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iactr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iactr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iactr2`] module"]
#[doc(alias = "R32_PFIC_IACTR2")]
pub type R32PficIactr2 = crate::Reg<r32_pfic_iactr2::R32PficIactr2Spec>;
#[doc = "Interrupt ACTIVE Register"]
pub mod r32_pfic_iactr2;
#[doc = "R32_PFIC_IPRIOR0 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior0`] module"]
#[doc(alias = "R32_PFIC_IPRIOR0")]
pub type R32PficIprior0 = crate::Reg<r32_pfic_iprior0::R32PficIprior0Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior0;
#[doc = "R32_PFIC_IPRIOR1 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior1`] module"]
#[doc(alias = "R32_PFIC_IPRIOR1")]
pub type R32PficIprior1 = crate::Reg<r32_pfic_iprior1::R32PficIprior1Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior1;
#[doc = "R32_PFIC_IPRIOR2 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior2`] module"]
#[doc(alias = "R32_PFIC_IPRIOR2")]
pub type R32PficIprior2 = crate::Reg<r32_pfic_iprior2::R32PficIprior2Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior2;
#[doc = "R32_PFIC_IPRIOR3 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior3`] module"]
#[doc(alias = "R32_PFIC_IPRIOR3")]
pub type R32PficIprior3 = crate::Reg<r32_pfic_iprior3::R32PficIprior3Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior3;
#[doc = "R32_PFIC_IPRIOR4 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior4`] module"]
#[doc(alias = "R32_PFIC_IPRIOR4")]
pub type R32PficIprior4 = crate::Reg<r32_pfic_iprior4::R32PficIprior4Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior4;
#[doc = "R32_PFIC_IPRIOR5 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior5`] module"]
#[doc(alias = "R32_PFIC_IPRIOR5")]
pub type R32PficIprior5 = crate::Reg<r32_pfic_iprior5::R32PficIprior5Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior5;
#[doc = "R32_PFIC_IPRIOR6 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior6`] module"]
#[doc(alias = "R32_PFIC_IPRIOR6")]
pub type R32PficIprior6 = crate::Reg<r32_pfic_iprior6::R32PficIprior6Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior6;
#[doc = "R32_PFIC_IPRIOR7 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior7`] module"]
#[doc(alias = "R32_PFIC_IPRIOR7")]
pub type R32PficIprior7 = crate::Reg<r32_pfic_iprior7::R32PficIprior7Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior7;
#[doc = "R32_PFIC_IPRIOR8 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior8`] module"]
#[doc(alias = "R32_PFIC_IPRIOR8")]
pub type R32PficIprior8 = crate::Reg<r32_pfic_iprior8::R32PficIprior8Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior8;
#[doc = "R32_PFIC_IPRIOR9 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior9`] module"]
#[doc(alias = "R32_PFIC_IPRIOR9")]
pub type R32PficIprior9 = crate::Reg<r32_pfic_iprior9::R32PficIprior9Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior9;
#[doc = "R32_PFIC_IPRIOR10 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior10`] module"]
#[doc(alias = "R32_PFIC_IPRIOR10")]
pub type R32PficIprior10 = crate::Reg<r32_pfic_iprior10::R32PficIprior10Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior10;
#[doc = "R32_PFIC_IPRIOR11 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior11`] module"]
#[doc(alias = "R32_PFIC_IPRIOR11")]
pub type R32PficIprior11 = crate::Reg<r32_pfic_iprior11::R32PficIprior11Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior11;
#[doc = "R32_PFIC_IPRIOR12 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior12`] module"]
#[doc(alias = "R32_PFIC_IPRIOR12")]
pub type R32PficIprior12 = crate::Reg<r32_pfic_iprior12::R32PficIprior12Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior12;
#[doc = "R32_PFIC_IPRIOR13 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior13`] module"]
#[doc(alias = "R32_PFIC_IPRIOR13")]
pub type R32PficIprior13 = crate::Reg<r32_pfic_iprior13::R32PficIprior13Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior13;
#[doc = "R32_PFIC_IPRIOR14 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior14`] module"]
#[doc(alias = "R32_PFIC_IPRIOR14")]
pub type R32PficIprior14 = crate::Reg<r32_pfic_iprior14::R32PficIprior14Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior14;
#[doc = "R32_PFIC_IPRIOR15 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior15`] module"]
#[doc(alias = "R32_PFIC_IPRIOR15")]
pub type R32PficIprior15 = crate::Reg<r32_pfic_iprior15::R32PficIprior15Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior15;
#[doc = "R32_PFIC_IPRIOR16 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior16`] module"]
#[doc(alias = "R32_PFIC_IPRIOR16")]
pub type R32PficIprior16 = crate::Reg<r32_pfic_iprior16::R32PficIprior16Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior16;
#[doc = "R32_PFIC_IPRIOR17 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior17`] module"]
#[doc(alias = "R32_PFIC_IPRIOR17")]
pub type R32PficIprior17 = crate::Reg<r32_pfic_iprior17::R32PficIprior17Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior17;
#[doc = "R32_PFIC_IPRIOR18 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior18`] module"]
#[doc(alias = "R32_PFIC_IPRIOR18")]
pub type R32PficIprior18 = crate::Reg<r32_pfic_iprior18::R32PficIprior18Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior18;
#[doc = "R32_PFIC_IPRIOR19 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior19`] module"]
#[doc(alias = "R32_PFIC_IPRIOR19")]
pub type R32PficIprior19 = crate::Reg<r32_pfic_iprior19::R32PficIprior19Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior19;
#[doc = "R32_PFIC_IPRIOR20 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior20`] module"]
#[doc(alias = "R32_PFIC_IPRIOR20")]
pub type R32PficIprior20 = crate::Reg<r32_pfic_iprior20::R32PficIprior20Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior20;
#[doc = "R32_PFIC_IPRIOR21 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior21`] module"]
#[doc(alias = "R32_PFIC_IPRIOR21")]
pub type R32PficIprior21 = crate::Reg<r32_pfic_iprior21::R32PficIprior21Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior21;
#[doc = "R32_PFIC_IPRIOR22 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior22`] module"]
#[doc(alias = "R32_PFIC_IPRIOR22")]
pub type R32PficIprior22 = crate::Reg<r32_pfic_iprior22::R32PficIprior22Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior22;
#[doc = "R32_PFIC_IPRIOR23 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior23`] module"]
#[doc(alias = "R32_PFIC_IPRIOR23")]
pub type R32PficIprior23 = crate::Reg<r32_pfic_iprior23::R32PficIprior23Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior23;
#[doc = "R32_PFIC_IPRIOR24 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior24`] module"]
#[doc(alias = "R32_PFIC_IPRIOR24")]
pub type R32PficIprior24 = crate::Reg<r32_pfic_iprior24::R32PficIprior24Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior24;
#[doc = "R32_PFIC_IPRIOR25 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior25`] module"]
#[doc(alias = "R32_PFIC_IPRIOR25")]
pub type R32PficIprior25 = crate::Reg<r32_pfic_iprior25::R32PficIprior25Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior25;
#[doc = "R32_PFIC_IPRIOR26 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior26`] module"]
#[doc(alias = "R32_PFIC_IPRIOR26")]
pub type R32PficIprior26 = crate::Reg<r32_pfic_iprior26::R32PficIprior26Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior26;
#[doc = "R32_PFIC_IPRIOR27 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior27`] module"]
#[doc(alias = "R32_PFIC_IPRIOR27")]
pub type R32PficIprior27 = crate::Reg<r32_pfic_iprior27::R32PficIprior27Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior27;
#[doc = "R32_PFIC_IPRIOR28 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior28`] module"]
#[doc(alias = "R32_PFIC_IPRIOR28")]
pub type R32PficIprior28 = crate::Reg<r32_pfic_iprior28::R32PficIprior28Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior28;
#[doc = "R32_PFIC_IPRIOR29 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior29`] module"]
#[doc(alias = "R32_PFIC_IPRIOR29")]
pub type R32PficIprior29 = crate::Reg<r32_pfic_iprior29::R32PficIprior29Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior29;
#[doc = "R32_PFIC_IPRIOR30 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior30`] module"]
#[doc(alias = "R32_PFIC_IPRIOR30")]
pub type R32PficIprior30 = crate::Reg<r32_pfic_iprior30::R32PficIprior30Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior30;
#[doc = "R32_PFIC_IPRIOR31 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior31`] module"]
#[doc(alias = "R32_PFIC_IPRIOR31")]
pub type R32PficIprior31 = crate::Reg<r32_pfic_iprior31::R32PficIprior31Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior31;
#[doc = "R32_PFIC_IPRIOR32 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior32`] module"]
#[doc(alias = "R32_PFIC_IPRIOR32")]
pub type R32PficIprior32 = crate::Reg<r32_pfic_iprior32::R32PficIprior32Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior32;
#[doc = "R32_PFIC_IPRIOR33 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior33`] module"]
#[doc(alias = "R32_PFIC_IPRIOR33")]
pub type R32PficIprior33 = crate::Reg<r32_pfic_iprior33::R32PficIprior33Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior33;
#[doc = "R32_PFIC_IPRIOR34 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior34`] module"]
#[doc(alias = "R32_PFIC_IPRIOR34")]
pub type R32PficIprior34 = crate::Reg<r32_pfic_iprior34::R32PficIprior34Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior34;
#[doc = "R32_PFIC_IPRIOR35 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior35`] module"]
#[doc(alias = "R32_PFIC_IPRIOR35")]
pub type R32PficIprior35 = crate::Reg<r32_pfic_iprior35::R32PficIprior35Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior35;
#[doc = "R32_PFIC_IPRIOR36 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior36`] module"]
#[doc(alias = "R32_PFIC_IPRIOR36")]
pub type R32PficIprior36 = crate::Reg<r32_pfic_iprior36::R32PficIprior36Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior36;
#[doc = "R32_PFIC_IPRIOR37 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior37`] module"]
#[doc(alias = "R32_PFIC_IPRIOR37")]
pub type R32PficIprior37 = crate::Reg<r32_pfic_iprior37::R32PficIprior37Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior37;
#[doc = "R32_PFIC_IPRIOR38 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior38`] module"]
#[doc(alias = "R32_PFIC_IPRIOR38")]
pub type R32PficIprior38 = crate::Reg<r32_pfic_iprior38::R32PficIprior38Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior38;
#[doc = "R32_PFIC_IPRIOR39 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior39`] module"]
#[doc(alias = "R32_PFIC_IPRIOR39")]
pub type R32PficIprior39 = crate::Reg<r32_pfic_iprior39::R32PficIprior39Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior39;
#[doc = "R32_PFIC_IPRIOR40 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior40`] module"]
#[doc(alias = "R32_PFIC_IPRIOR40")]
pub type R32PficIprior40 = crate::Reg<r32_pfic_iprior40::R32PficIprior40Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior40;
#[doc = "R32_PFIC_IPRIOR41 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior41`] module"]
#[doc(alias = "R32_PFIC_IPRIOR41")]
pub type R32PficIprior41 = crate::Reg<r32_pfic_iprior41::R32PficIprior41Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior41;
#[doc = "R32_PFIC_IPRIOR42 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior42`] module"]
#[doc(alias = "R32_PFIC_IPRIOR42")]
pub type R32PficIprior42 = crate::Reg<r32_pfic_iprior42::R32PficIprior42Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior42;
#[doc = "R32_PFIC_IPRIOR43 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior43`] module"]
#[doc(alias = "R32_PFIC_IPRIOR43")]
pub type R32PficIprior43 = crate::Reg<r32_pfic_iprior43::R32PficIprior43Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior43;
#[doc = "R32_PFIC_IPRIOR44 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior44`] module"]
#[doc(alias = "R32_PFIC_IPRIOR44")]
pub type R32PficIprior44 = crate::Reg<r32_pfic_iprior44::R32PficIprior44Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior44;
#[doc = "R32_PFIC_IPRIOR45 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior45`] module"]
#[doc(alias = "R32_PFIC_IPRIOR45")]
pub type R32PficIprior45 = crate::Reg<r32_pfic_iprior45::R32PficIprior45Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior45;
#[doc = "R32_PFIC_IPRIOR46 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior46`] module"]
#[doc(alias = "R32_PFIC_IPRIOR46")]
pub type R32PficIprior46 = crate::Reg<r32_pfic_iprior46::R32PficIprior46Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior46;
#[doc = "R32_PFIC_IPRIOR47 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior47`] module"]
#[doc(alias = "R32_PFIC_IPRIOR47")]
pub type R32PficIprior47 = crate::Reg<r32_pfic_iprior47::R32PficIprior47Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior47;
#[doc = "R32_PFIC_IPRIOR48 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior48`] module"]
#[doc(alias = "R32_PFIC_IPRIOR48")]
pub type R32PficIprior48 = crate::Reg<r32_pfic_iprior48::R32PficIprior48Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior48;
#[doc = "R32_PFIC_IPRIOR49 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior49`] module"]
#[doc(alias = "R32_PFIC_IPRIOR49")]
pub type R32PficIprior49 = crate::Reg<r32_pfic_iprior49::R32PficIprior49Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior49;
#[doc = "R32_PFIC_IPRIOR50 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior50`] module"]
#[doc(alias = "R32_PFIC_IPRIOR50")]
pub type R32PficIprior50 = crate::Reg<r32_pfic_iprior50::R32PficIprior50Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior50;
#[doc = "R32_PFIC_IPRIOR51 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior51`] module"]
#[doc(alias = "R32_PFIC_IPRIOR51")]
pub type R32PficIprior51 = crate::Reg<r32_pfic_iprior51::R32PficIprior51Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior51;
#[doc = "R32_PFIC_IPRIOR52 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior52`] module"]
#[doc(alias = "R32_PFIC_IPRIOR52")]
pub type R32PficIprior52 = crate::Reg<r32_pfic_iprior52::R32PficIprior52Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior52;
#[doc = "R32_PFIC_IPRIOR53 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior53`] module"]
#[doc(alias = "R32_PFIC_IPRIOR53")]
pub type R32PficIprior53 = crate::Reg<r32_pfic_iprior53::R32PficIprior53Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior53;
#[doc = "R32_PFIC_IPRIOR54 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior54`] module"]
#[doc(alias = "R32_PFIC_IPRIOR54")]
pub type R32PficIprior54 = crate::Reg<r32_pfic_iprior54::R32PficIprior54Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior54;
#[doc = "R32_PFIC_IPRIOR55 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior55`] module"]
#[doc(alias = "R32_PFIC_IPRIOR55")]
pub type R32PficIprior55 = crate::Reg<r32_pfic_iprior55::R32PficIprior55Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior55;
#[doc = "R32_PFIC_IPRIOR56 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior56`] module"]
#[doc(alias = "R32_PFIC_IPRIOR56")]
pub type R32PficIprior56 = crate::Reg<r32_pfic_iprior56::R32PficIprior56Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior56;
#[doc = "R32_PFIC_IPRIOR57 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior57`] module"]
#[doc(alias = "R32_PFIC_IPRIOR57")]
pub type R32PficIprior57 = crate::Reg<r32_pfic_iprior57::R32PficIprior57Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior57;
#[doc = "R32_PFIC_IPRIOR58 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior58`] module"]
#[doc(alias = "R32_PFIC_IPRIOR58")]
pub type R32PficIprior58 = crate::Reg<r32_pfic_iprior58::R32PficIprior58Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior58;
#[doc = "R32_PFIC_IPRIOR59 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior59`] module"]
#[doc(alias = "R32_PFIC_IPRIOR59")]
pub type R32PficIprior59 = crate::Reg<r32_pfic_iprior59::R32PficIprior59Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior59;
#[doc = "R32_PFIC_IPRIOR60 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior60`] module"]
#[doc(alias = "R32_PFIC_IPRIOR60")]
pub type R32PficIprior60 = crate::Reg<r32_pfic_iprior60::R32PficIprior60Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior60;
#[doc = "R32_PFIC_IPRIOR61 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior61`] module"]
#[doc(alias = "R32_PFIC_IPRIOR61")]
pub type R32PficIprior61 = crate::Reg<r32_pfic_iprior61::R32PficIprior61Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior61;
#[doc = "R32_PFIC_IPRIOR62 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior62`] module"]
#[doc(alias = "R32_PFIC_IPRIOR62")]
pub type R32PficIprior62 = crate::Reg<r32_pfic_iprior62::R32PficIprior62Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior62;
#[doc = "R32_PFIC_IPRIOR63 (rw) register accessor: Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_iprior63`] module"]
#[doc(alias = "R32_PFIC_IPRIOR63")]
pub type R32PficIprior63 = crate::Reg<r32_pfic_iprior63::R32PficIprior63Spec>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior63;
#[doc = "R32_PFIC_SCTLR (rw) register accessor: System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_sctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_sctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_sctlr`] module"]
#[doc(alias = "R32_PFIC_SCTLR")]
pub type R32PficSctlr = crate::Reg<r32_pfic_sctlr::R32PficSctlrSpec>;
#[doc = "System Control Register"]
pub mod r32_pfic_sctlr;
#[doc = "R32_PFIC_VTCTLR (rw) register accessor: System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_vtctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_vtctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pfic_vtctlr`] module"]
#[doc(alias = "R32_PFIC_VTCTLR")]
pub type R32PficVtctlr = crate::Reg<r32_pfic_vtctlr::R32PficVtctlrSpec>;
#[doc = "System Control Register"]
pub mod r32_pfic_vtctlr;
