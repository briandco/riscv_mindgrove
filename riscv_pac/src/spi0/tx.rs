#[doc = "Register `TX` reader"]
pub type R = crate::R<TxSpec>;
#[doc = "Register `TX` writer"]
pub type W = crate::W<TxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "holds the tx data. This register is written by the AXI write request and once written the data is transferred to TX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxSpec;
impl crate::RegisterSpec for TxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx::R`](R) reader structure"]
impl crate::Readable for TxSpec {}
#[doc = "`write(|w| ..)` method takes [`tx::W`](W) writer structure"]
impl crate::Writable for TxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX to value 0"]
impl crate::Resettable for TxSpec {
    const RESET_VALUE: u32 = 0;
}
