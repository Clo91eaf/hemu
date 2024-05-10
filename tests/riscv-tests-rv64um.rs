mod rv64um_p {

  use std::fs::File;
  use std::io::prelude::*;
  use std::path::PathBuf;

  use hemu::{bus::DRAM_BASE, cpu::Mode, emulator::Emulator};
  #[macro_export]
  macro_rules! add_test_p {
    ($name: ident) => {
      #[test]
      fn $name() -> anyhow::Result<()> {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.push("dependencies/tests/bin/riscv-tests/rv64um");
        root.push(("rv64um-p-".to_owned() + stringify!($name)).replace("__", ""));

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
  add_test_p!(div);
  add_test_p!(divu);
  add_test_p!(divuw);
  add_test_p!(divw);
  add_test_p!(mul);
  add_test_p!(mulh);
  add_test_p!(mulhsu);
  add_test_p!(mulhu);
  add_test_p!(mulw);
  add_test_p!(rem);
  add_test_p!(remu);
  add_test_p!(remuw);
  add_test_p!(remw);
}

mod rv64um_v {
  use std::fs::File;
  use std::io::prelude::*;
  use std::path::PathBuf;

  use hemu::{bus::DRAM_BASE, cpu::Mode, emulator::Emulator};
  #[macro_export]
  macro_rules! add_test_v {
    ($name: ident) => {
      #[test]
      fn $name() -> anyhow::Result<()> {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.push("dependencies/tests/bin/riscv-tests/rv64um");
        root.push(("rv64um-v-".to_owned() + stringify!($name)).replace("__", ""));

        let mut file = File::open(root.as_path())?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let mut emu = Emulator::new(false, false);
        emu.initialize_dram(data);
        emu.initialize_pc(DRAM_BASE);

        emu.start();

        // https://github.com/riscv/riscv-test-env/blob/4fabfb4e0d3eacc1dc791da70e342e4b68ea7e46/v/riscv_test.h#L45
        // #define RVTEST_PASS li a0, 1; scall
        // when all tests pass, a0 is set to 1
        // #define RVTEST_FAIL sll a0, TESTNUM, 1; 1:beqz a0, 1b; or a0, a0, 1; scall;
        // when test x fails, a0 is set to 2 * x + 1
        // and if no test is run, the test will loo.
        assert_eq!(1, emu.cpu.gpr.read(10));

        assert_eq!(Mode::Supervisor, emu.cpu.mode);
        Ok(())
      }
    };
  }
  add_test_v!(div);
  add_test_v!(divu);
  add_test_v!(divuw);
  add_test_v!(divw);
  add_test_v!(mul);
  add_test_v!(mulh);
  add_test_v!(mulhsu);
  add_test_v!(mulhu);
  add_test_v!(mulw);
  add_test_v!(rem);
  add_test_v!(remu);
  add_test_v!(remuw);
  add_test_v!(remw);
}
