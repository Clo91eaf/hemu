pub mod sdb;

use crate::debug::debug_init;
use crate::constants::*;
use crate::memory::paddr::guest_to_host;
use std::io::{Read, Seek, SeekFrom};

fn welcome() {
  log::info!("Welcome to riscv64-HEMU!",);
  log::info!("For help, type \"help\"");
}

// load img to memory(mmap)
fn load_img(img_file: String) -> Result<usize, Box<dyn std::error::Error>> {
  // open img file
  log::info!("img file:{}", img_file);
  let mut file = std::fs::File::open(img_file)?;

  // get img size
  let size = file.seek(SeekFrom::End(0))?;
  log::info!("img size:{}", size);

  // read img to buffer
  file.seek(SeekFrom::Start(0))?;
  let mut buffer = vec![0; size as usize];
  file.read_exact(&mut buffer)?;

  // copy img to memory
  let dst = guest_to_host(RESET_VECTOR);
  let src = buffer.as_ptr() as *const u8;
  unsafe {
    std::ptr::copy_nonoverlapping(src, dst, size as usize);
  }

  Ok(size as usize)
}

pub fn init_monitor() -> Result<(), Box<dyn std::error::Error>> {
  debug_init();

  let img_file =
    String::from("./tests/build/dummy-riscv64-nemu.bin");

  #[allow(unused_variables)]
  let img_size = load_img(img_file).unwrap();

  welcome();

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;

  #[test]
  fn test_load_img() {
    let file_path =
      PathBuf::from("system-tests/cpu-tests/build/dummy-riscv64-hemu.bin");
    let result = load_img(file_path.to_str().unwrap().to_string()).unwrap();
    println!("result:{}", result)
  }
}
