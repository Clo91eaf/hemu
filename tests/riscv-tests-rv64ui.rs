mod rv64ui_p {

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
}

mod rv64ui_v {
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
        root.push("dependencies/tests/bin/riscv-tests/rv64ui");
        root.push("rv64ui-v-".to_owned() + stringify!($name));

        let mut file = File::open(root.as_path())?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let mut emu = Emulator::new(false, false);
        emu.initialize_dram(data);
        emu.initialize_pc(DRAM_BASE);

        emu.start();

        // Test result is stored at a0 (x10), a function argument and a return value.
        // The riscv-tests set a0 to 0 when all tests pass.
        assert_eq!(1, emu.cpu.gpr.read(10));

        // All tests start the user mode and finish with the instruction `ecall`, independently
        // of it succeeds or fails.
        assert_eq!(Mode::Supervisor, emu.cpu.mode);
        Ok(())
      }
    };
  }

  add_test_v!(add);
  add_test_v!(addi);
  add_test_v!(addiw);
  add_test_v!(addw);
  add_test_v!(and);
  add_test_v!(andi);
  add_test_v!(auipc);
  add_test_v!(beq);
  add_test_v!(bge);
  add_test_v!(bgeu);
  add_test_v!(blt);
  add_test_v!(bltu);
  add_test_v!(bne);
  add_test_v!(fence_i);
  add_test_v!(jal);
  add_test_v!(jalr);
  add_test_v!(lb);
  add_test_v!(lbu);
  add_test_v!(ld);
  add_test_v!(lh);
  add_test_v!(lhu);
  add_test_v!(lui);
  add_test_v!(lw);
  add_test_v!(lwu);
  add_test_v!(ma_data);
  add_test_v!(or);
  add_test_v!(ori);
  add_test_v!(sb);
  add_test_v!(sd);
  add_test_v!(sh);
  add_test_v!(simple);
  add_test_v!(sll);
  add_test_v!(slli);
  add_test_v!(slliw);
  add_test_v!(sllw);
  add_test_v!(slt);
  add_test_v!(slti);
  add_test_v!(sltiu);
  add_test_v!(sltu);
  add_test_v!(sra);
  add_test_v!(srai);
  add_test_v!(sraiw);
  add_test_v!(sraw);
  add_test_v!(srl);
  add_test_v!(srli);
  add_test_v!(srliw);
  add_test_v!(srlw);
  add_test_v!(sub);
  add_test_v!(subw);
  add_test_v!(sw);
  add_test_v!(xor);
  add_test_v!(xori);
}
