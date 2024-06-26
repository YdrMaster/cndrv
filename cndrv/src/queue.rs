﻿use crate::{bindings::CNqueue, impl_spore, AsRaw, CurrentCtx};
use std::{marker::PhantomData, ptr::null_mut};

impl_spore!(Queue and QueueSpore by CNqueue);

impl CurrentCtx {
    #[inline]
    pub fn queue(&self) -> Queue {
        let mut queue = null_mut();
        cndrv!(cnCreateQueue(&mut queue, 0));
        Queue(unsafe { self.wrap_raw(queue) }, PhantomData)
    }
}

impl Drop for Queue<'_> {
    #[inline]
    fn drop(&mut self) {
        self.synchronize();
        cndrv!(cnDestroyQueue(self.0.raw));
    }
}

impl AsRaw for Queue<'_> {
    type Raw = CNqueue;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0.raw
    }
}

impl Queue<'_> {
    #[inline]
    pub fn synchronize(&self) {
        cndrv!(cnQueueSync(self.0.raw));
    }
}
