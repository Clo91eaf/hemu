use crate::monitor::{init_monitor, sdb};
use crate::cpu::Cpu;

pub fn engine_start() {
  let _ = init_monitor();

  let cpu = &mut Cpu::new();

  sdb::sdb_mainloop(cpu);
}
