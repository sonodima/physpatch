use std::error::Error;
use std::result::Result;

use memflow::prelude::*;
use memflow::types::Address;
use memflow_win32::win32::Win32Kernel;

pub trait MemoryExtension {
    fn phys_read_unchecked<T>(&mut self, address: Address) -> Result<T, Box<dyn Error>>;
}

impl<P, V> MemoryExtension for Win32Kernel<P, V>
where
    P: 'static + PhysicalMemory,
    V: 'static + VirtualTranslate2,
{
    /// Reads any structure from the given physical memory address.
    ///
    /// This is used to read the structures defined in the x86 crate, which are
    /// not Pod.
    fn phys_read_unchecked<T: Sized>(&mut self, address: Address) -> Result<T, Box<dyn Error>> {
        let buffer = self
            .phys_view()
            .read_raw(address, std::mem::size_of::<T>())?;
        let ptr: *const T = buffer.as_ptr() as _;
        unsafe { Ok(ptr.read()) }
    }
}
