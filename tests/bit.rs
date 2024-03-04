use hemu::cpu;
use hemu::init_monitor;
use hemu::monitor::sdb;
use std::path::PathBuf;

#[test]
fn test_bit() {
  // prepare the log file
  let log = PathBuf::from("logs/bit-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/build/bit-riscv64-nemu.bin");

  // start the monitor
  let _ = init_monitor(img);
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}
