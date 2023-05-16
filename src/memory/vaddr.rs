use crate::memory::paddr::paddr_read;
use crate::memory::paddr::paddr_write;

pub fn vaddr_read(addr: u64, len: i32) -> u64 {
  paddr_read(addr, len)
}

pub fn vaddr_write(addr: u64, len: i32, data: u64) {
  paddr_write(addr, len, data)
}