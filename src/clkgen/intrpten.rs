#[doc = "Reader of register INTRPTEN"]
pub type R = crate::R<u32, super::INTRPTEN>;
#[doc = "Writer for register INTRPTEN"]
pub type W = crate::W<u32, super::INTRPTEN>;
#[doc = "Register INTRPTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTRPTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OF`"]
pub type OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OF`"]
pub struct OF_W<'a> {
    w: &'a mut W,
}
impl<'a> OF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ACC`"]
pub type ACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACC`"]
pub struct ACC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACC_W<'a> {
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
#[doc = "Reader of field `ACF`"]
pub type ACF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACF`"]
pub struct ACF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACF_W<'a> {
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
    #[doc = "Bit 2 - XT Oscillator Fail interrupt"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Autocalibration Complete interrupt"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Autocalibration Fail interrupt"]
    #[inline(always)]
    pub fn acf(&self) -> ACF_R {
        ACF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - XT Oscillator Fail interrupt"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W { w: self }
    }
    #[doc = "Bit 1 - Autocalibration Complete interrupt"]
    #[inline(always)]
    pub fn acc(&mut self) -> ACC_W {
        ACC_W { w: self }
    }
    #[doc = "Bit 0 - Autocalibration Fail interrupt"]
    #[inline(always)]
    pub fn acf(&mut self) -> ACF_W {
        ACF_W { w: self }
    }
}
