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
      root.push("tests/resources/riscv-tests/rv64ui");
      root.push("rv64ui-p-".to_owned() + stringify!($name) + ".bin");

      println!("root: {:?}", root);

      let mut file = File::open(root.as_path())?;
      let mut data = Vec::new();
      file.read_to_end(&mut data)?;

      let mut emu = Emulator::new();
      emu.initialize_dram(data);
      emu.initialize_pc(DRAM_BASE);

      emu.start();

      // Test result is stored at a0 (x10), a function argument and a return value.
      // The riscv-tests set a0 to 0 when all tests pass.
      assert_eq!(0, emu.cpu.xregs.read(10));

      // All tests start the user mode and finish with the instruction `ecall`, independently
      // of it succeeds or fails.
      assert_eq!(Mode::Machine, emu.cpu.mode);
      Ok(())
    }
  };
}

add_test!(add);
add_test!(addi);
add_test!(addiw);
add_test!(addw);
add_test!(and);
add_test!(andi);
add_test!(auipc);
add_test!(beq);
add_test!(bge);
add_test!(bgeu);
add_test!(blt);
add_test!(bltu);
add_test!(bne);
add_test!(fence_i);
add_test!(jal);
add_test!(jalr);
add_test!(lb);
add_test!(lbu);
add_test!(ld);
add_test!(lh);
add_test!(lhu);
add_test!(lui);
add_test!(lw);
add_test!(lwu);
add_test!(ma_data);
add_test!(or);
add_test!(ori);
add_test!(sb);
add_test!(sd);
add_test!(sh);
add_test!(simple);
add_test!(sll);
add_test!(slli);
add_test!(slliw);
add_test!(sllw);
add_test!(slt);
add_test!(slti);
add_test!(sltiu);
add_test!(sltu);
add_test!(sra);
add_test!(srai);
add_test!(sraiw);
add_test!(sraw);
add_test!(srl);
add_test!(srli);
add_test!(srliw);
add_test!(srlw);
add_test!(sub);
add_test!(subw);
add_test!(sw);
add_test!(xor);
add_test!(xori);
