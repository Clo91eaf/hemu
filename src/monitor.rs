pub mod sdb;

use getopt::Opt;
use std::fs::File;
use std::io::Read;
use crate::debug::debug_init;

fn welcome() {
  log::info!("Welcome to riscv64-HEMU!",);
  log::info!("For help, type \"help\"");
}

fn load_img(img_file: String) -> std::io::Result<usize> {
  log::info!("img file:{}", img_file);
  let mut file = File::open(img_file)?;
  let metadata = file.metadata()?;
  let size = metadata.len() as usize;

  let buffer = unsafe {
    let ptr = libc::malloc(size) as *mut u8;
    if ptr.is_null() {
      panic!("Failed to allocate memory!");
    }
    std::slice::from_raw_parts_mut(ptr, size)
  };

  file.read_exact(buffer)?;

  Ok(size)
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
