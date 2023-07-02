use crate::memory::vaddr;

pub fn read_inst(addr: u64) -> u64 {
  read_data(addr, 4)
}

#[allow(dead_code)]
pub fn read_data(addr: u64, len: i32) -> u64 {
  vaddr::vaddr_read(addr, len)
}

pub fn write_data(addr: u64, len: i32, data: u64) {
  vaddr::vaddr_write(addr, len, data)
}