use crate::monitor::{init_monitor, sdb};

pub fn engine_start() {
  let _ = init_monitor();

  sdb::sdb_mainloop();
}
