use hemu::cpu;
use hemu::monitor::*;
use std::path::PathBuf;

#[test]
fn test_add() {
  // prepare the log file
  let log = PathBuf::from("logs/add-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/add-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}
