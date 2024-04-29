use std::path::Path;
use std::path::PathBuf;
use std::{env, fs};
use verilator::gen::{Standard, Verilator};
use verilator::module::ModuleGenerator;

macro_rules! t {
  ($e:expr) => {
    match $e {
      Ok(e) => e,
      Err(e) => panic!("{} failed with {}", stringify!($e), e),
    }
  };
}

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let out_dir = PathBuf::from(out_dir);
  let _ = fs::remove_dir_all(&out_dir);
  t!(fs::create_dir_all(&out_dir));

  // Generate CPP shim from RUST
  let mut module = ModuleGenerator::default();
  module.generate("src/dut/top.rs");

  // Generate CPP from Verilog
  let mut verilator = Verilator::default();
  verilator
    .with_coverage(true)
    .with_trace(true)
    .no_warn("fatal")
    .no_warn("WIDTHTRUNC")
    .file_with_standard("dependencies/rtl/PuaCpu.v", Standard::SystemVerilog2012)
    .file_with_standard("dependencies/rtl/top.v", Standard::SystemVerilog2012)
    .file(out_dir.join("top.cpp"))
    .build("top");
}
