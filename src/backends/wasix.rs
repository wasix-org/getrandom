// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for WASI
use crate::Error;
use core::mem::MaybeUninit;
use wasix::random_get;

pub use crate::util::{inner_u32, inner_u64};

pub fn fill_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    match unsafe { random_get(dest.as_mut_ptr() as *mut _, dest.len()) } {
        Ok(()) => Ok(()),
        Err(num) => Err(Error::new_custom(num.raw())),
    }
}
