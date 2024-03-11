use hemu::cpu;
use hemu::monitor::*;
use std::path::PathBuf;

#[macro_export]
macro_rules! add_test {
  ($name: ident) => {
    #[test]
    fn $name() {
      let mut root = String::from(env!("CARGO_MANIFEST_DIR"));
      root.push_str("/tests/resources/riscv-tests/rv64ui/rv64ui-p-");
      root.push_str(&stringify!($name).replace("_", "-"));

      // prepare the diff file
      let diff = None;

      // prepare the img file
      let img = PathBuf::from(root.to_string() + ".bin");

      println!("img:{:?}", img);

      // start the monitor
      let _ = load_img(img).unwrap();
      let cpu = &mut cpu::Cpu::new(diff);
      sdb::sdb_mainloop(cpu, true);

      // check the result
      assert_eq!(cpu.state, cpu::State::Ended);
      assert_eq!(cpu.halt.ret, 0);
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
