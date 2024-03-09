use hemu::cpu;
use hemu::monitor::*;
use std::path::PathBuf;

#[test]
fn test_add() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-add.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_addi() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-addi.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_addiw() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-addiw.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_addw() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-addw.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_and() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-and.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_andi() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-andi.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_auipc() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-auipc.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_beq() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-beq.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_bge() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-bge.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_bgeu() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-bgeu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_blt() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-blt.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_bltu() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-bltu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_bne() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-bne.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_fence_i() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-fence_i.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_jal() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-jal.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_jalr() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-jalr.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_lb() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-lb.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_lbu() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-lbu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_ld() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-ld.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_lh() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-lh.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_lhu() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-lhu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_lui() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-lui.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_lw() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-lw.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_lwu() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-lwu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_ma_data() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-ma_data.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_or() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-or.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_ori() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-ori.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sb() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sb.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sd() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sd.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sh() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sh.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_simple() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-simple.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sll() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sll.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_slli() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-slli.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_slliw() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-slliw.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sllw() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sllw.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_slt() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-slt.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_slti() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-slti.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sltiu() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sltiu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sltu() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sltu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sra() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sra.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_srai() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-srai.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sraiw() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sraiw.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sraw() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sraw.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_srl() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-srl.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_srli() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-srli.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_srliw() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-srliw.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_srlw() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-srlw.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sub() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sub.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_subw() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-subw.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sw() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-sw.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_xor() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-xor.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_xori() {
  // prepare the img file
  let img = PathBuf::from("./resources/riscv-tests/rv64ui/rv64ui-p-xori.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

