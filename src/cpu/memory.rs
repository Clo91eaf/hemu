use crate::memory::vaddr;

pub fn read_inst(addr: u64) -> u64 {
  vaddr::vaddr_read(addr, 4)
}

pub fn read_data(addr: u64, len: i32) -> u64 {
  vaddr::vaddr_read(addr, len)
}

pub fn write_data(addr: u64, len: i32, data: u64) {
  todo!();
}