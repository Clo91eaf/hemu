use hemu::cpu;
use hemu::monitor::*;
use std::path::PathBuf;

#[test]
fn test_add_longlong() {
  // prepare the log file
  let log = PathBuf::from("logs/add-longlong-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/add-longlong-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

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

#[test]
fn test_bit() {
  // prepare the log file
  let log = PathBuf::from("logs/bit-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/bit-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_bubble_sort() {
  // prepare the log file
  let log = PathBuf::from("logs/bubble-sort-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/bubble-sort-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_div() {
  // prepare the log file
  let log = PathBuf::from("logs/div-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/div-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_dummy() {
  // prepare the log file
  let log = PathBuf::from("logs/dummy-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/dummy-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_fact() {
  // prepare the log file
  let log = PathBuf::from("logs/fact-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/fact-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_fib() {
  // prepare the log file
  let log = PathBuf::from("logs/fib-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/fib-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_goldbach() {
  // prepare the log file
  let log = PathBuf::from("logs/goldbach-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/goldbach-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_hello_str() {
  // prepare the log file
  let log = PathBuf::from("logs/hello-str-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/hello-str-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_if_else() {
  // prepare the log file
  let log = PathBuf::from("logs/if-else-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/if-else-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_leap_year() {
  // prepare the log file
  let log = PathBuf::from("logs/leap-year-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/leap-year-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_load_store() {
  // prepare the log file
  let log = PathBuf::from("logs/load-store-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/load-store-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_matrix_mul() {
  // prepare the log file
  let log = PathBuf::from("logs/matrix-mul-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/matrix-mul-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_max() {
  // prepare the log file
  let log = PathBuf::from("logs/max-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/max-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_min3() {
  // prepare the log file
  let log = PathBuf::from("logs/min3-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/min3-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_mov_c() {
  // prepare the log file
  let log = PathBuf::from("logs/mov-c-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/mov-c-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_movsx() {
  // prepare the log file
  let log = PathBuf::from("logs/movsx-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/movsx-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_mul_longlong() {
  // prepare the log file
  let log = PathBuf::from("logs/mul-longlong-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/mul-longlong-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_pascal() {
  // prepare the log file
  let log = PathBuf::from("logs/pascal-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/pascal-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_prime() {
  // prepare the log file
  let log = PathBuf::from("logs/prime-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/prime-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_quick_sort() {
  // prepare the log file
  let log = PathBuf::from("logs/quick-sort-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/quick-sort-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_recursion() {
  // prepare the log file
  let log = PathBuf::from("logs/recursion-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/recursion-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_select_sort() {
  // prepare the log file
  let log = PathBuf::from("logs/select-sort-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/select-sort-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_shift() {
  // prepare the log file
  let log = PathBuf::from("logs/shift-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/shift-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_shuixianhua() {
  // prepare the log file
  let log = PathBuf::from("logs/shuixianhua-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/shuixianhua-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_string() {
  // prepare the log file
  let log = PathBuf::from("logs/string-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/string-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sub_longlong() {
  // prepare the log file
  let log = PathBuf::from("logs/sub-longlong-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/sub-longlong-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sum() {
  // prepare the log file
  let log = PathBuf::from("logs/sum-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/sum-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_switch() {
  // prepare the log file
  let log = PathBuf::from("logs/switch-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/switch-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_to_lower_case() {
  // prepare the log file
  let log = PathBuf::from("logs/to-lower-case-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/to-lower-case-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_unalign() {
  // prepare the log file
  let log = PathBuf::from("logs/unalign-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/unalign-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_wanshu() {
  // prepare the log file
  let log = PathBuf::from("logs/wanshu-riscv64-hemu.diff");
  let _ = std::fs::remove_file(&log);

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/wanshu-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(log);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}
