use crate::memory::paddr::paddr_read;

pub fn vaddr_ifetch(addr: u64, len: i32) -> u64 {
  paddr_read(addr, len)
}