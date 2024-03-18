mod rv64ui_p {

  use std::fs::File;
  use std::io::prelude::*;
  use std::path::PathBuf;
  
  use hemu::{bus::DRAM_BASE, cpu::Mode, emulator::Emulator};
  
  #[macro_export]
  macro_rules! add_test {
    ($name: ident) => {
      #[test]
      fn $name() -> anyhow::Result<()> {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.push("dependencies/tests/bin/am-tests");
        root.push(stringify!($name).to_owned().replace("_", "-"));
  
        let mut file = File::open(root)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
  
        let mut emu = Emulator::new();
        emu.initialize_dram(data);
        emu.initialize_pc(DRAM_BASE);
  
        emu.start();
  
        // Test result is stored at a0 (x10), a function argument and a return value.
        // The riscv-tests set a0 to 0 when all tests pass.
        assert_eq!(0, emu.cpu.gpr.read(10));
  
        // All tests start the user mode and finish with the instruction `ecall`, independently
        // of it succeeds or fails.
        assert_eq!(Mode::Machine, emu.cpu.mode);
        Ok(())
      }
    };
  }
  
  add_test!(add_longlong);
  add_test!(add);
  add_test!(bit);
  add_test!(bubble_sort);
  add_test!(div);
  add_test!(dummy);
  add_test!(fact);
  add_test!(fib);
  add_test!(goldbach);
  add_test!(if_else);
  add_test!(leap_year);
  add_test!(load_store);
  add_test!(matrix_mul);
  add_test!(max);
  add_test!(min3);
  add_test!(mov_c);
  add_test!(movsx);
  add_test!(mul_longlong);
  add_test!(pascal);
  add_test!(prime);
  add_test!(quick_sort);
  add_test!(recursion);
  add_test!(select_sort);
  add_test!(shift);
  add_test!(shuixianhua);
  add_test!(sub_longlong);
  add_test!(sum);
  add_test!(switch);
  add_test!(to_lower_case);
  add_test!(unalign);
  add_test!(wanshu);
}

mod rv64ui_v {

}