use hemu::cpu;
use hemu::monitor::*;
use std::path::PathBuf;

#[test]
fn test_add() {
  // prepare the img file
  let img = PathBuf::from("resources/riscv-tests/rv64ui/rv64ui-p-add.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(None);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}