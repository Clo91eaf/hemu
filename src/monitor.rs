pub mod sdb;

use crate::debug::debug_init;
use crate::memory::paddr::guest_to_host;
use getopt::Opt;
use std::fs::File;
use std::io::Read;

fn welcome() {
  log::info!("Welcome to riscv64-HEMU!",);
  log::info!("For help, type \"help\"");
}

fn load_img(img_file: String) -> std::io::Result<usize> {
  log::info!("img file:{}", img_file);
  let mut file = File::open(img_file)?;
  let mut buffer = [0; 0x1000];
  let buffer_ptr = 0x80000000 as u64;

  unsafe {
    loop {
      match file.read(&mut buffer) {
        Ok(0) => return Ok(0),
        Ok(n) => {
          std::ptr::copy_nonoverlapping(
            buffer.as_ptr(),
            guest_to_host(buffer_ptr),
            n,
          );
          return Ok(n);
        }
        Err(_) => panic!("Failed to read file"),
      }
    }
  }
}

pub fn init_monitor() -> Result<(), Box<dyn std::error::Error>> {
  debug_init();

  let args: Vec<String> = std::env::args().collect();
  let mut opts = getopt::Parser::new(&args, "f:");

  let mut img_file: String = String::from("resources/dummy-riscv64-nemu.bin");
  loop {
    match opts.next().transpose()? {
      None => break,
      Some(opt) => match opt {
        Opt('f', Some(file)) => {
          img_file = file;
        }
        _ => unreachable!(),
      },
    }
  }
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
    let file_path = PathBuf::from("resources/dummy-riscv64-nemu.bin");
    let result = load_img(file_path.to_str().unwrap().to_string()).unwrap();
    assert_eq!(result, 57);
  }
}
