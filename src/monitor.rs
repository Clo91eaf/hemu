pub mod sdb;
pub mod expr;

use crate::constants::*;
use crate::log::init_log;
use crate::memory::paddr::guest_to_host;
use std::{
  io::{Read, Seek, SeekFrom},
  path::PathBuf,
};

use clap::Parser;
use sdb::init_sdb;

/// A riscv64 monitor write in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Run in batch mode
  #[arg(short, long, default_value = "false")]
  batch: bool,

  /// Log file
  #[arg(short, long, default_value = "tests/build/dummy-riscv64-nemu.log")]
  log: PathBuf,

  /// Diff file
  #[arg(short, long, default_value = "tests/build/dummy-riscv64-nemu.diff")]
  diff: PathBuf,

  /// Diff port
  #[arg(short, long, default_value = "1234")]
  port: u32,

  /// Img file
  #[arg(short='f', long, default_value = "tests/build/dummy-riscv64-nemu.bin")]
  img: PathBuf,
}

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

pub fn init_monitor() -> Result<usize, Box<dyn std::error::Error>> {
  let args = Args::parse();

  init_log();

  init_sdb();

  let img_size = load_img(args.img).unwrap();

  welcome();

  Ok(img_size)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;

  #[test]
  fn test_load_img() {
    let file_path =
      PathBuf::from("system-tests/cpu-tests/build/dummy-riscv64-hemu.bin");
    let result = load_img(file_path).unwrap();
    println!("result:{}", result)
  }
}
