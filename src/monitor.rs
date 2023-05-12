pub mod sdb;

use getopt::Opt;
use std::fs::File;
use std::io::{Read};

fn load_img(img_file: String) -> std::io::Result<usize> {
  let mut file = File::open(img_file)?;
  let metadata = file.metadata()?;
  let size = metadata.len() as usize;

  // 申请一段连续的内存空间作为缓冲区，长度为文件大小
  let buffer = unsafe {
    let ptr = libc::malloc(size) as *mut u8;
    if ptr.is_null() {
      panic!("Failed to allocate memory!");
    }
    std::slice::from_raw_parts_mut(ptr, size)
  };

  // 读取整个文件内容到缓冲区中
  file.read_exact(buffer)?;

  // 将缓冲区地址转换为 usize 类型并返回文件大小
  Ok(size)
}

pub fn init_monitor() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = std::env::args().collect();
  let mut opts = getopt::Parser::new(&args, "f:");

  let mut img_file: String = String::from("../resources/dummy-riscv64-nemu.bin");
  loop {
    match opts.next().transpose()? {
      None => break,
      Some(opt) => match opt {
        Opt('f', Some(file)) => img_file = file,
        _ => unreachable!(),
      },
    }
  }
  #[allow(unused_variables)]
  let img_size = load_img(img_file).unwrap();

  Ok(())
}
