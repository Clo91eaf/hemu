use crate::monitor::{init_monitor, sdb};
use crate::cpu::Cpu;

pub fn engine_start() {
  let size = init_monitor();

  let cpu = &mut Cpu::new();

  cpu.img_size = size.unwrap();

  sdb::sdb_mainloop(cpu);
}
