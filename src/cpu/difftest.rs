use crate::{constants::RESET_VECTOR, cpu::Cpu, memory::paddr::guest_to_host};

trait HasDifftest {
  fn difftest_init(self, port: u16);
  fn difftest_step();
  fn difftest_skip_ref();
  fn difftest_skip_dut();
  fn difftest_checkregs();
}

impl HasDifftest for Cpu {
  fn difftest_init(mut self, port: u16) {
    let mut diff_ref = self.difftest;
    diff_ref.init(port);

    let src = unsafe {
      std::slice::from_raw_parts(
        guest_to_host((RESET_VECTOR as u64).into()),
        self.img_size,
      )
    };
    diff_ref.memcpy(RESET_VECTOR as u32, src);
    // diff_ref.regcpy(self.gpr, direction);
    self.difftest = diff_ref;
  }

  fn difftest_step() {
    todo!("difftest_step");
  }

  fn difftest_skip_ref() {
    todo!("difftest_skip_ref");
  }

  fn difftest_skip_dut() {
    todo!("difftest_skip_dut")
  }

  fn difftest_checkregs() {
    todo!("difftest_checkregs")
  }
}
