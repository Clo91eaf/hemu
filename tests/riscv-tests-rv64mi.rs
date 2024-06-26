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
      root.push("dependencies/tests/bin/riscv-tests/rv64mi");
      root.push(("rv64mi-p-".to_owned() + stringify!($name)).replace("_", "-"));

      let mut file = File::open(root.as_path())?;
      let mut data = Vec::new();
      file.read_to_end(&mut data)?;

      let mut emu = Emulator::new(false, false);
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

#[macro_export]
macro_rules! add_test_no_replace {
  ($name: ident) => {
    #[test]
    fn $name() -> anyhow::Result<()> {
      let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
      root.push("dependencies/tests/bin/riscv-tests/rv64mi");
      root.push("rv64mi-p-".to_owned() + stringify!($name));

      let mut file = File::open(root.as_path())?;
      let mut data = Vec::new();
      file.read_to_end(&mut data)?;

      let mut emu = Emulator::new(false, false);
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

add_test!(access);
// add_test!(csr);
// add_test!(illegal);
add_test!(ld_misaligned);
add_test!(lh_misaligned);
add_test!(lw_misaligned);
add_test_no_replace!(ma_addr);
add_test_no_replace!(ma_fetch);
add_test!(mcsr);
add_test!(sbreak);
add_test!(scall);
add_test!(sd_misaligned);
add_test!(sh_misaligned);
add_test!(sw_misaligned);
add_test!(zicntr);
