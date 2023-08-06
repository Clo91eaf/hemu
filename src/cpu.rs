mod difftest;
mod instruction;
pub mod memory;
mod statistic;
mod utils;

use ansi_term::Colour::{Green, Red};
use instruction::*;
use memory::{read_data, read_inst, write_data};
use qemu_difftest::Difftest;
use utils::{decode_operand, match_inst, sext};

#[derive(PartialEq)]
pub enum CpuState {
  Running,
  // Stopped,
  Ended,
  // Aborted,
  Quit,
}

pub struct Halt {
  pc: u32,
  ret: u32,
}

impl Halt {
  pub fn new() -> Halt {
    Halt { pc: 0, ret: 0 }
  }
}

pub struct Cpu {
  pub gpr: [u64; 32],
  pub fpr: [u64; 32],
  pub pc: u64,
  snpc: u64,
  dnpc: u64,
  inst: u32,
  pub state: CpuState,
  halt: Halt,
  statistic: statistic::Statistic,
  difftest: Difftest,
  pub img_size: usize,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      gpr: [0; 32],
      fpr: [0; 32],
      pc: 0x80000000,
      snpc: 0x80000000,
      dnpc: 0x80000000,
      inst: 0,
      state: CpuState::Running,
      halt: Halt::new(),
      statistic: statistic::Statistic::new(),
      difftest: Difftest::new(),
      img_size: 0,
    }
  }

  fn hemu_trap(&mut self) {
    self.state = CpuState::Ended;
    self.halt.pc = self.pc as u32;
    self.halt.ret = self.gpr[10] as u32;

    log::info!("hemu trap, pc = {:x}, ret = {}", self.pc, self.gpr[10]);
  }

  pub fn fetch(&mut self) {
    self.inst = read_inst(self.pc) as u32;
    self.snpc += 4;

    log::debug!("fetch: pc = 0x{:08x}, inst = 0x{:08x}", self.pc, self.inst);
  }

  pub fn decode(&self, inst_type: &mut Instruction) {
    #[rustfmt::skip]
    let patterns = [
    // Register 
    InstPattern::new("000000 ????? ????? ????? 00000 100000", Instruction::Computational(ComputationalType::ADD)),
    InstPattern::new("001000 ????? ????? ????? ????? ??????", Instruction::Computational(ComputationalType::ADDI)),
    InstPattern::new("000000 ????? ????? ????? 00000 100001", Instruction::Computational(ComputationalType::ADDU)),
    InstPattern::new("001001 ????? ????? ????? ????? ??????", Instruction::Computational(ComputationalType::ADDIU)),
    InstPattern::new("000000 ????? ????? ????? 00000 100010", Instruction::Computational(ComputationalType::SUB)),
    InstPattern::new("000000 ????? ????? ????? 00000 100011", Instruction::Computational(ComputationalType::SUBU)),
    InstPattern::new("001010 ????? ????? ????? 00000 101010", Instruction::Computational(ComputationalType::SLTI)),
    InstPattern::new("000000 ????? ????? ????? 00000 101011", Instruction::Computational(ComputationalType::SLTU)),
    InstPattern::new("001011 ????? ????? ????? ????? ??????", Instruction::Computational(ComputationalType::SLTIU)),
    InstPattern::new("000000 ????? ????? 00000 00000 011010", Instruction::Computational(ComputationalType::DIV)),
    InstPattern::new("000000 ????? ????? 00000 00000 011011", Instruction::Computational(ComputationalType::DIVU)),
    InstPattern::new("000000 ????? ????? 00000 00000 011000", Instruction::Computational(ComputationalType::MULT)),
    InstPattern::new("000000 ????? ????? 00000 00000 011001", Instruction::Computational(ComputationalType::MULTU)),

    InstPattern::new("000000 ????? ????? ????? 00000 100100", Instruction::Computational(ComputationalType::AND)),
    InstPattern::new("001100 ????? ????? ????? ????? ??????", Instruction::Computational(ComputationalType::ANDI)),
    InstPattern::new("001111 00000 ????? ????? ????? ??????", Instruction::Computational(ComputationalType::LUI)),
    InstPattern::new("000000 ????? ????? ????? 00000 100111", Instruction::Computational(ComputationalType::NOR)),
    InstPattern::new("000000 ????? ????? ????? 00000 100101", Instruction::Computational(ComputationalType::OR)),
    InstPattern::new("001101 ????? ????? ????? ????? ??????", Instruction::Computational(ComputationalType::ORI)),
    InstPattern::new("000000 ????? ????? ????? 00000 100110", Instruction::Computational(ComputationalType::XOR)),
    InstPattern::new("001110 ????? ????? ????? ????? ??????", Instruction::Computational(ComputationalType::XORI)),
    InstPattern::new("001110 ????? ????? ????? ????? ??????", Instruction::Computational(ComputationalType::XORI)),

    InstPattern::new("000000 ????? ????? ????? 00000 000100", Instruction::Computational(ComputationalType::SLLV)),
    InstPattern::new("000000 00000 ????? ????? ????? 000000", Instruction::Computational(ComputationalType::SLL)),
    InstPattern::new("000000 ????? ????? ????? 00000 000111", Instruction::Computational(ComputationalType::SRAV)),
    InstPattern::new("000000 00000 ????? ????? ????? 000011", Instruction::Computational(ComputationalType::SRA)),
    InstPattern::new("000000 ????? ????? ????? 00000 000110", Instruction::Computational(ComputationalType::SRLV)),
    InstPattern::new("000000 00000 ????? ????? ????? 000010", Instruction::Computational(ComputationalType::SRL)),

    InstPattern::new("000000 00000 ????? ????? ????? 000010", Instruction::Computational(ComputationalType::SRL)),
    ];
    for pattern in patterns.iter() {
      if match_inst(self.inst, pattern.pattern) {
        *inst_type = pattern.itype;
        return;
      }
    }
  }

  #[rustfmt::skip]
  pub fn execute(&mut self, inst_type: Instruction) {
    let (rd, rs1, rs2, imm) = decode_operand(self.inst, inst_type);
    self.dnpc = self.snpc;
    match inst_type {
      Instruction::Register(RegisterType::ADD)  => {self.gpr[rd] = (self.gpr[rs1] as i64 + self.gpr[rs2] as i64) as u64;}
      _ => {todo!("{:?} not implemented", inst_type);}
    }
    self.gpr[0] = 0;
  }

  fn exec_once(&mut self) {
    // pipeline start
    let mut inst_type = Instruction::Immediate(ImmediateType::EBREAK);
    // fetch stage
    self.fetch();
    // decode stage
    self.decode(&mut inst_type);
    // execute stage (including memory stage and write back stage)
    self.execute(inst_type);
    // update pc
    self.pc = self.dnpc;
  }

  fn exec_ntimes(&mut self, n: usize) {
    for _ in 0..n {
      self.exec_once();
      self.statistic.inc_count();
      if self.state != CpuState::Running {
        break;
      }
    }
  }

  fn statistic(&self) {
    log::info!("host time spend = {:?}", self.statistic.time);
    log::info!("total guest instructions = {:?}", self.statistic.count);
    log::info!(
      "simulation frequency = {:?}",
      (self.statistic.count) as f64 / (self.statistic.time.as_secs_f64())
    );
  }

  pub fn exec(&mut self, n: usize) -> i32 {
    let start_time = self.statistic.start_timer();

    self.exec_ntimes(n);

    self.statistic.stop_timer(start_time);

    match self.state {
      CpuState::Ended => {
        if self.halt.ret == 0 {
          println!("{}", Green.bold().paint("HIT GOOD TRAP"));
          self.statistic();
          return 0;
        } else {
          log::error!("{}", Red.bold().paint("HIT BAD TRAP"));
          return -1;
        }
      }
      CpuState::Running => {}
      CpuState::Quit => {
        self.statistic();
      }
    }
    0
  }

  pub fn dump_registers(&self) {
    for i in 0..32 {
      print!("x{:02} = 0x{:016x} ", i, self.gpr[i]);
      if i % 4 == 3 {
        println!();
      }
    }
    println!();
  }

  pub fn dump_watches(&self) {
    todo!("watch points not implemented!");
  }
}
