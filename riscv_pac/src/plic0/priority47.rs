#[doc = "Register `PRIORITY47` reader"]
pub type R = crate::R<Priority47Spec>;
#[doc = "Register `PRIORITY47` writer"]
pub type W = crate::W<Priority47Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 47\n\nYou can [`read`](crate::Reg::read) this register and get [`priority47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority47Spec;
impl crate::RegisterSpec for Priority47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority47::R`](R) reader structure"]
impl crate::Readable for Priority47Spec {}
#[doc = "`write(|w| ..)` method takes [`priority47::W`](W) writer structure"]
impl crate::Writable for Priority47Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY47 to value 0"]
impl crate::Resettable for Priority47Spec {
    const RESET_VALUE: u32 = 0;
}