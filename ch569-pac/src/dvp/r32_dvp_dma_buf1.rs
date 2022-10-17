#[doc = "Register `R32_DVP_DMA_BUF1` reader"]
pub struct R(crate::R<R32_DVP_DMA_BUF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_DVP_DMA_BUF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_DVP_DMA_BUF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_DVP_DMA_BUF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_DVP_DMA_BUF1` writer"]
pub struct W(crate::W<R32_DVP_DMA_BUF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_DVP_DMA_BUF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<R32_DVP_DMA_BUF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_DVP_DMA_BUF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_DMA_BUF1` reader - the receiving address1 of DMA"]
pub type RB_DVP_DMA_BUF1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RB_DVP_DMA_BUF1` writer - the receiving address1 of DMA"]
pub type RB_DVP_DMA_BUF1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_DVP_DMA_BUF1_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - the receiving address1 of DMA"]
    #[inline(always)]
    pub fn rb_dvp_dma_buf1(&self) -> RB_DVP_DMA_BUF1_R {
        RB_DVP_DMA_BUF1_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - the receiving address1 of DMA"]
    #[inline(always)]
    pub fn rb_dvp_dma_buf1(&mut self) -> RB_DVP_DMA_BUF1_W<0> {
        RB_DVP_DMA_BUF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DVP dma buffer1 addr\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_dvp_dma_buf1](index.html) module"]
pub struct R32_DVP_DMA_BUF1_SPEC;
impl crate::RegisterSpec for R32_DVP_DMA_BUF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_dvp_dma_buf1::R](R) reader structure"]
impl crate::Readable for R32_DVP_DMA_BUF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_dvp_dma_buf1::W](W) writer structure"]
impl crate::Writable for R32_DVP_DMA_BUF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_DVP_DMA_BUF1 to value 0"]
impl crate::Resettable for R32_DVP_DMA_BUF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}