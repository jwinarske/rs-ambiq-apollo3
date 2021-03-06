#[doc = "Reader of register DMACFG"]
pub type R = crate::R<u32, super::DMACFG>;
#[doc = "Writer for register DMACFG"]
pub type W = crate::W<u32, super::DMACFG>;
#[doc = "Register DMACFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Power off module after DMA is complete. If this bit is active, the module will request to power off the supply it is attached to. If there are other units still requiring power from the same domain, power down will not be performed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPWROFF_A {
    #[doc = "0: Power off disabled value."]
    DIS = 0,
    #[doc = "1: Power off enabled value."]
    EN = 1,
}
impl From<DPWROFF_A> for bool {
    #[inline(always)]
    fn from(variant: DPWROFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPWROFF`"]
pub type DPWROFF_R = crate::R<bool, DPWROFF_A>;
impl DPWROFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPWROFF_A {
        match self.bits {
            false => DPWROFF_A::DIS,
            true => DPWROFF_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DPWROFF_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DPWROFF_A::EN
    }
}
#[doc = "Write proxy for field `DPWROFF`"]
pub struct DPWROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPWROFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPWROFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power off disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DPWROFF_A::DIS)
    }
    #[doc = "Power off enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DPWROFF_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Sets the Priority of the DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAPRI_A {
    #[doc = "0: Low Priority (service as best effort) value."]
    LOW = 0,
    #[doc = "1: High Priority (service immediately) value."]
    HIGH = 1,
}
impl From<DMAPRI_A> for bool {
    #[inline(always)]
    fn from(variant: DMAPRI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAPRI`"]
pub type DMAPRI_R = crate::R<bool, DMAPRI_A>;
impl DMAPRI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAPRI_A {
        match self.bits {
            false => DMAPRI_A::LOW,
            true => DMAPRI_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DMAPRI_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DMAPRI_A::HIGH
    }
}
#[doc = "Write proxy for field `DMAPRI`"]
pub struct DMAPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAPRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAPRI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low Priority (service as best effort) value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DMAPRI_A::LOW)
    }
    #[doc = "High Priority (service immediately) value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DMAPRI_A::HIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMADIR_A {
    #[doc = "0: Peripheral to Memory (SRAM) transaction.  To be set when doing IOM read operations, ie reading data from external devices. value."]
    P2M = 0,
    #[doc = "1: Memory to Peripheral transaction.  To be set when doing IOM write operations, ie writing data to external devices. value."]
    M2P = 1,
}
impl From<DMADIR_A> for bool {
    #[inline(always)]
    fn from(variant: DMADIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMADIR`"]
pub type DMADIR_R = crate::R<bool, DMADIR_A>;
impl DMADIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADIR_A {
        match self.bits {
            false => DMADIR_A::P2M,
            true => DMADIR_A::M2P,
        }
    }
    #[doc = "Checks if the value of the field is `P2M`"]
    #[inline(always)]
    pub fn is_p2m(&self) -> bool {
        *self == DMADIR_A::P2M
    }
    #[doc = "Checks if the value of the field is `M2P`"]
    #[inline(always)]
    pub fn is_m2p(&self) -> bool {
        *self == DMADIR_A::M2P
    }
}
#[doc = "Write proxy for field `DMADIR`"]
pub struct DMADIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Peripheral to Memory (SRAM) transaction. To be set when doing IOM read operations, ie reading data from external devices. value."]
    #[inline(always)]
    pub fn p2m(self) -> &'a mut W {
        self.variant(DMADIR_A::P2M)
    }
    #[doc = "Memory to Peripheral transaction. To be set when doing IOM write operations, ie writing data to external devices. value."]
    #[inline(always)]
    pub fn m2p(self) -> &'a mut W {
        self.variant(DMADIR_A::M2P)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "DMA Enable. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: Disable DMA Function value."]
    DIS = 0,
    #[doc = "1: Enable DMA Function value."]
    EN = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DIS,
            true => DMAEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMAEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMAEN_A::EN
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable DMA Function value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAEN_A::DIS)
    }
    #[doc = "Enable DMA Function value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAEN_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - Power off module after DMA is complete. If this bit is active, the module will request to power off the supply it is attached to. If there are other units still requiring power from the same domain, power down will not be performed."]
    #[inline(always)]
    pub fn dpwroff(&self) -> DPWROFF_R {
        DPWROFF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sets the Priority of the DMA request"]
    #[inline(always)]
    pub fn dmapri(&self) -> DMAPRI_R {
        DMAPRI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Direction"]
    #[inline(always)]
    pub fn dmadir(&self) -> DMADIR_R {
        DMADIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA Enable. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Power off module after DMA is complete. If this bit is active, the module will request to power off the supply it is attached to. If there are other units still requiring power from the same domain, power down will not be performed."]
    #[inline(always)]
    pub fn dpwroff(&mut self) -> DPWROFF_W {
        DPWROFF_W { w: self }
    }
    #[doc = "Bit 8 - Sets the Priority of the DMA request"]
    #[inline(always)]
    pub fn dmapri(&mut self) -> DMAPRI_W {
        DMAPRI_W { w: self }
    }
    #[doc = "Bit 1 - Direction"]
    #[inline(always)]
    pub fn dmadir(&mut self) -> DMADIR_W {
        DMADIR_W { w: self }
    }
    #[doc = "Bit 0 - DMA Enable. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
}
