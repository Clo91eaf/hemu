use hemu::cpu;
use hemu::monitor::*;
use std::path::PathBuf;

#[test]
fn test_add_longlong() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/add-longlong-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/add-longlong-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_add() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/add-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/add-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_bit() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/bit-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/bit-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_bubble_sort() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/bubble-sort-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/bubble-sort-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_div() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/div-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/div-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_dummy() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/dummy-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/dummy-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_fact() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/fact-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/fact-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_fib() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/fib-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/fib-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_goldbach() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/goldbach-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/goldbach-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_if_else() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/if-else-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/if-else-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_leap_year() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/leap-year-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/leap-year-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_load_store() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/load-store-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/load-store-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_matrix_mul() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/matrix-mul-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/matrix-mul-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_max() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/max-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/max-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_min3() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/min3-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/min3-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_mov_c() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/mov-c-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/mov-c-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_movsx() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/movsx-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/movsx-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_mul_longlong() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/mul-longlong-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/mul-longlong-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_pascal() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/pascal-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/pascal-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_prime() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/prime-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/prime-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_quick_sort() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/quick-sort-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/quick-sort-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_recursion() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/recursion-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/recursion-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_select_sort() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/select-sort-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/select-sort-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_shift() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/shift-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/shift-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_shuixianhua() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/shuixianhua-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/shuixianhua-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sub_longlong() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/sub-longlong-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/sub-longlong-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_sum() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/sum-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/sum-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_switch() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/switch-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/switch-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_to_lower_case() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/to-lower-case-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/to-lower-case-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_unalign() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/unalign-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/unalign-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}

#[test]
fn test_wanshu() {
  // prepare the diff file
  let diff = PathBuf::from("resources/am-tests/wanshu-riscv64-nemu.diff");

  // prepare the img file
  let img = PathBuf::from("resources/am-tests/wanshu-riscv64-nemu.bin");

  // start the monitor
  let _ = load_img(img).unwrap();
  let cpu = &mut cpu::Cpu::new(diff);
  sdb::sdb_mainloop(cpu, true);

  // check the result
  assert_eq!(cpu.state, cpu::CpuState::Ended);
  assert_eq!(cpu.halt.ret, 0);
}
