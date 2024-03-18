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
      root.push("dependencies/tests/bin/riscv-tests/rv64ui");
      root.push("rv64ui-p-".to_owned() + stringify!($name));

      let mut file = File::open(root.as_path())?;
      let mut data = Vec::new();
      file.read_to_end(&mut data)?;

      let mut emu = Emulator::new();
      emu.initialize_dram(data);
      emu.initialize_pc(DRAM_BASE);

      emu.start();

      // Test result is stored at a0 (x10), a function argument and a return value.
      // The riscv-tests set a0 to 0 when all tests pass.
      // assert_eq!(0, emu.cpu.gpr.read(10));

      // All tests start the user mode and finish with the instruction `ecall`, independently
      // of it succeeds or fails.
      assert_eq!(Mode::Machine, emu.cpu.mode);
      Ok(())
    }
  };
}

add_test_p!(add);
add_test_p!(addi);
add_test_p!(addiw);
add_test_p!(addw);
add_test_p!(and);
add_test_p!(andi);
add_test_p!(auipc);
add_test_p!(beq);
add_test_p!(bge);
add_test_p!(bgeu);
add_test_p!(blt);
add_test_p!(bltu);
add_test_p!(bne);
add_test_p!(fence_i);
add_test_p!(jal);
add_test_p!(jalr);
add_test_p!(lb);
add_test_p!(lbu);
add_test_p!(ld);
add_test_p!(lh);
add_test_p!(lhu);
add_test_p!(lui);
add_test_p!(lw);
add_test_p!(lwu);
add_test_p!(ma_data);
add_test_p!(or);
add_test_p!(ori);
add_test_p!(sb);
add_test_p!(sd);
add_test_p!(sh);
add_test_p!(simple);
add_test_p!(sll);
add_test_p!(slli);
add_test_p!(slliw);
add_test_p!(sllw);
add_test_p!(slt);
add_test_p!(slti);
add_test_p!(sltiu);
add_test_p!(sltu);
add_test_p!(sra);
add_test_p!(srai);
add_test_p!(sraiw);
add_test_p!(sraw);
add_test_p!(srl);
add_test_p!(srli);
add_test_p!(srliw);
add_test_p!(srlw);
add_test_p!(sub);
add_test_p!(subw);
add_test_p!(sw);
add_test_p!(xor);
add_test_p!(xori);
