use crate::constants::*;
use core::panic;
use lazy_static::lazy_static;
use log::{error, info};
use std::sync::Mutex;
use crate::memory::host::{host_read, host_write};

lazy_static! {
  #[repr(align(4096))]
  static ref PMEM: Mutex<Vec<u8>> = Mutex::new(vec![0; MEM_SIZE as usize]);
}

pub fn guest_to_host(paddr: u64) -> *mut u8 {
  let mut pmem = PMEM.lock().unwrap();
  (pmem.as_mut_ptr() as u64)
    .wrapping_add(paddr)
    .wrapping_sub(MEM_BASE) as *mut u8
}

#[allow(dead_code)]
fn host_to_guest(haddr: *mut u8) -> u64 {
  let mut pmem = PMEM.lock().unwrap();
  (haddr as u64)
    .wrapping_sub(pmem.as_mut_ptr() as u64)
    .wrapping_add(MEM_BASE)
}

#[allow(dead_code)]
fn in_pmem(addr: u64) -> bool {
  addr.wrapping_sub(MEM_BASE) < MEM_SIZE
}

fn out_of_bound(addr: u64) -> ! {
  error!(
    "address = {:08X} is out of bound of pmem [{:08X}, {:08X}) at pc",
    addr,
    MEM_BASE,
    MEM_BASE + MEM_SIZE,
  );
  panic!()
}

pub fn pmem_read(addr: u64, len: i32) -> u64 {
  host_read(guest_to_host(addr) as u64, len)
}

pub fn pmem_write(addr: u64, len: i32, data: u64) {
  host_write(guest_to_host(addr) as u64, len, data)
}

pub fn paddr_read(addr: u64, len: i32) -> u64 {
  if in_pmem(addr) {
    let data = pmem_read(addr, len);
    info!("R addr = {:08x}, len = {}, data = {:08x}", addr, len, data);
    return data;
  }
  out_of_bound(addr)
}

pub fn paddr_write(addr: u64, len: i32, data: u64) {
  if in_pmem(addr) {
    info!("W addr = {:08x}, len = {}, data = {:08x}", addr, len, data);
    return pmem_write(addr, len, data)
  }
  out_of_bound(addr)
}
