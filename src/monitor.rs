pub mod sdb;
pub mod expr;

use crate::constants::*;
use crate::log::init_log;
use crate::memory::paddr::guest_to_host;
use std::{
  io::{Read, Seek, SeekFrom},
  path::PathBuf,
};

use sdb::init_sdb;

fn welcome() {
  log::info!("Welcome to riscv64-HEMU!",);
  log::info!("For help, type \"help\"");
}

// load img to memory(mmap)
fn load_img(img_file: PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
  // open img file
  log::info!("img file:{}", img_file.to_string_lossy());
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

pub fn init_monitor(img: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
  init_log();

  init_sdb();

  #[allow(unused_variables)]
  let img_size = load_img(img).unwrap();

  welcome();

  Ok(())
}

