use crate::constants::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
  #[repr(align(4096))]
  static ref PMEM: Mutex<Vec<u8>> = Mutex::new(vec![0; MEM_SIZE as usize]);
}

pub fn guest_to_host(paddr: u64) -> *mut u8 {
  let mut pmem = PMEM.lock().unwrap();
  (pmem.as_mut_ptr() as u64 + paddr - MEM_BASE) as *mut u8
}

#[allow(dead_code)]
fn host_to_guest(haddr: *mut u8) -> u64 {
  let mut pmem = PMEM.lock().unwrap();
  haddr as u64 - pmem.as_mut_ptr() as u64 + MEM_BASE
}

#[allow(dead_code)]
fn in_pmem(addr: u64) -> bool {
  addr >= MEM_BASE && addr + 1 < MEM_BASE + MEM_SIZE
}

#[allow(dead_code)]
fn out_of_bound(addr: u64) -> ! {
  panic!(
    "address = {:016X} is out of bound of pmem [{:016X}, {:016X}) at pc",
    addr,
    MEM_BASE,
    MEM_BASE + MEM_SIZE,
  );
}

pub fn paddr_read(addr: u64, len: i32) -> u64 {
  let haddr = guest_to_host(addr);
  match len {
    1 => unsafe { *haddr as u64 },
    2 => unsafe { *(haddr as *const u16) as u64 },
    4 => unsafe { *(haddr as *const u32) as u64 },
    8 => unsafe { *(haddr as *const u64) },
    _ => panic!("invalid read length {}", len),
  }
}

pub fn paddr_write(addr: u64, len: i32, data: u64) {
  let haddr = guest_to_host(addr);
  match len {
    1 => unsafe { *haddr = data as u8 },
    2 => unsafe { *(haddr as *mut u16) = data as u16 },
    4 => unsafe { *(haddr as *mut u32) = data as u32 },
    8 => unsafe { *(haddr as *mut u64) = data },
    _ => panic!("invalid write length {}", len),
  }
}
