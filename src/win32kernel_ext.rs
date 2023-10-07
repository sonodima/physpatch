use std::mem::{size_of, MaybeUninit};

use anyhow::Result;

use memflow::prelude::*;
use memflow::types::Address;
use memflow_win32::win32::Win32Kernel;

pub trait Win32KernelExt {
    unsafe fn phys_read_unchecked<T>(&mut self, address: Address) -> Result<T>;
}

impl<P, V> Win32KernelExt for Win32Kernel<P, V>
where
    P: 'static + PhysicalMemory,
    V: 'static + VirtualTranslate2,
{
    /// Reads any structure from the given physical memory address.
    ///
    /// This is used to read the structures defined in the x86 crate, which are not Pod.
    unsafe fn phys_read_unchecked<T: Sized>(&mut self, address: Address) -> Result<T> {
        let mut data: T = MaybeUninit::uninit().assume_init();
        let slice = std::slice::from_raw_parts_mut(&mut data as *mut T as _, size_of::<T>());
        self.phys_view().read_raw_into(address, slice)?;
        Ok(data)
    }
}
