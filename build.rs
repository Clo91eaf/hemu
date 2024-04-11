// use std::path::Path;
// use std::path::PathBuf;
// use std::{env, fs};
// use verilator::gen::{Standard, Verilator};
// use verilator::module::ModuleGenerator;

// macro_rules! t {
//   ($e:expr) => {
//     match $e {
//       Ok(e) => e,
//       Err(e) => panic!("{} failed with {}", stringify!($e), e),
//     }
//   };
// }

fn main() {
//   let out_dir = env::var("OUT_DIR").unwrap();
//   let out_dir = PathBuf::from(out_dir);
//   let _ = fs::remove_dir_all(&out_dir);
//   t!(fs::create_dir_all(&out_dir));

//   // Generate CPP shim from RUST
//   let mut module = ModuleGenerator::default();
//   module.generate("src/dut/top.rs");

//   // Generate CPP from Verilog
//   let mut verilator = Verilator::default();

//   let dir = Path::new("myCPU");
//   let files: Vec<_> = fs::read_dir(dir)
//     .expect("Directory not found")
//     .filter_map(|entry| entry.ok().and_then(|e| e.path().to_str().map(|s| String::from(s))))
//     .collect();

//   println!("Files: {:?}", files);

//   verilator.with_coverage(true).with_trace(true);

//   for file in &files {
//     verilator.file_with_standard(file, Standard::Verilog2005);
//   }

//   verilator.file(out_dir.join("mycpu_top.cpp")).build("mycpu_top");
}
