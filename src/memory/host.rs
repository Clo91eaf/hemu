pub fn host_read(addr: u64, len: i32) -> u64 {
  match len {
    1 => unsafe { *(addr as *const u8) as u64 },
    2 => unsafe { *(addr as *const u16) as u64 },
    4 => unsafe { *(addr as *const u32) as u64 },
    8 => unsafe { *(addr as *const u64) },
    _ => panic!("invalid read length {}", len),
  }
}

pub fn host_write(addr: u64, len: i32, data: u64) {
  match len {
    1 => unsafe { *(addr as *mut u8) = data as u8 },
    2 => unsafe { *(addr as *mut u16) = data as u16 },
    4 => unsafe { *(addr as *mut u32) = data as u32 },
    8 => unsafe { *(addr as *mut u64) = data },
    _ => panic!("invalid write length {}", len),
  }
}