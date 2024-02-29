mod instruction;
pub mod memory;
mod statistic;
mod utils;

use ansi_term::Colour::{Green, Red};
use instruction::InstPattern;
use instruction::{
  BranchType, ImmediateType, Instruction, JumpType, RegisterType, StoreType,
  UpperType,
};
use memory::{read_data, read_inst, write_data};
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
  pub pc: u64,
  snpc: u64,
  dnpc: u64,
  inst: u32,
  pub state: CpuState,
  halt: Halt,
  statistic: statistic::Statistic,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      gpr: [0; 32],
      pc: 0x80000000,
      snpc: 0x80000000,
      dnpc: 0x80000000,
      inst: 0,
      state: CpuState::Running,
      halt: Halt::new(),
      statistic: statistic::Statistic::new(),
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
  InstPattern::new("0000000 ????? ????? 000 ????? 01100 11", Instruction::Register(RegisterType::ADD)),
  InstPattern::new("0000000 ????? ????? 000 ????? 01110 11", Instruction::Register(RegisterType::ADDW)),
  InstPattern::new("0100000 ????? ????? 000 ????? 01100 11", Instruction::Register(RegisterType::SUB)),
  InstPattern::new("0000000 ????? ????? 100 ????? 01100 11", Instruction::Register(RegisterType::XOR)),
  InstPattern::new("0000000 ????? ????? 110 ????? 01100 11", Instruction::Register(RegisterType::OR)),
  InstPattern::new("0000000 ????? ????? 111 ????? 01100 11", Instruction::Register(RegisterType::AND)),
  InstPattern::new("0000000 ????? ????? 001 ????? 01100 11", Instruction::Register(RegisterType::SLL)),
  InstPattern::new("0000000 ????? ????? 101 ????? 01100 11", Instruction::Register(RegisterType::SRL)),
  InstPattern::new("0000000 ????? ????? 010 ????? 01100 11", Instruction::Register(RegisterType::SLT)),
  InstPattern::new("0000000 ????? ????? 011 ????? 01100 11", Instruction::Register(RegisterType::SLTU)),
    // Immediate
  InstPattern::new("??????? ????? ????? 000 ????? 00100 11", Instruction::Immediate(ImmediateType::ADDI)),
  InstPattern::new("??????? ????? ????? 000 ????? 00110 11", Instruction::Immediate(ImmediateType::ADDIW)),
  InstPattern::new("??????? ????? ????? 100 ????? 00100 11", Instruction::Immediate(ImmediateType::XORI)),
  InstPattern::new("??????? ????? ????? 110 ????? 00100 11", Instruction::Immediate(ImmediateType::ORI)),
  InstPattern::new("??????? ????? ????? 111 ????? 00100 11", Instruction::Immediate(ImmediateType::ANDI)),
  InstPattern::new("000000? ????? ????? 001 ????? 00100 11", Instruction::Immediate(ImmediateType::SLLI)),
  InstPattern::new("000000? ????? ????? 101 ????? 00100 11", Instruction::Immediate(ImmediateType::SRLI)),
  InstPattern::new("010000? ????? ????? 101 ????? 00100 11", Instruction::Immediate(ImmediateType::SRAI)),
  InstPattern::new("??????? ????? ????? 010 ????? 00100 11", Instruction::Immediate(ImmediateType::SLTI)),
  InstPattern::new("??????? ????? ????? 011 ????? 00100 11", Instruction::Immediate(ImmediateType::SLTIU)),
  InstPattern::new("??????? ????? ????? 000 ????? 00000 11", Instruction::Immediate(ImmediateType::LB)),
  InstPattern::new("??????? ????? ????? 100 ????? 00000 11", Instruction::Immediate(ImmediateType::LBU)),
  InstPattern::new("??????? ????? ????? 001 ????? 00000 11", Instruction::Immediate(ImmediateType::LH)),
  InstPattern::new("??????? ????? ????? 101 ????? 00000 11", Instruction::Immediate(ImmediateType::LHU)),
  InstPattern::new("??????? ????? ????? 010 ????? 00000 11", Instruction::Immediate(ImmediateType::LW)),
  InstPattern::new("??????? ????? ????? 110 ????? 00000 11", Instruction::Immediate(ImmediateType::LWU)),
  InstPattern::new("??????? ????? ????? 011 ????? 00000 11", Instruction::Immediate(ImmediateType::LD)),
  InstPattern::new("??????? ????? ????? 111 ????? 00000 11", Instruction::Immediate(ImmediateType::LDU)),
  InstPattern::new("??????? ????? ????? 000 ????? 11001 11", Instruction::Immediate(ImmediateType::JALR)),
    // Store
  InstPattern::new("??????? ????? ????? 000 ????? 01000 11", Instruction::Store(StoreType::SB)),
  InstPattern::new("??????? ????? ????? 001 ????? 01000 11", Instruction::Store(StoreType::SH)),
  InstPattern::new("??????? ????? ????? 010 ????? 01000 11", Instruction::Store(StoreType::SW)),
  InstPattern::new("??????? ????? ????? 011 ????? 01000 11", Instruction::Store(StoreType::SD)),
    // Branch
  InstPattern::new("??????? ????? ????? 000 ????? 11000 11", Instruction::Branch(BranchType::BEQ)),
  InstPattern::new("??????? ????? ????? 001 ????? 11000 11", Instruction::Branch(BranchType::BNE)),
  InstPattern::new("??????? ????? ????? 100 ????? 11000 11", Instruction::Branch(BranchType::BLT)),
  InstPattern::new("??????? ????? ????? 101 ????? 11000 11", Instruction::Branch(BranchType::BGE)),
  InstPattern::new("??????? ????? ????? 110 ????? 11000 11", Instruction::Branch(BranchType::BLTU)),
  InstPattern::new("??????? ????? ????? 111 ????? 11000 11", Instruction::Branch(BranchType::BGEU)),
    // Jump
  InstPattern::new("??????? ????? ????? ??? ????? 11011 11", Instruction::Jump(JumpType::JAL)),
    // Upper
  InstPattern::new("??????? ????? ????? ??? ????? 01101 11", Instruction::Upper(UpperType::LUI)),
  InstPattern::new("??????? ????? ????? ??? ????? 00101 11", Instruction::Upper(UpperType::AUIPC)),
    // RV32M
  InstPattern::new("0000001 ????? ????? 000 ????? 01100 11", Instruction::Register(RegisterType::MUL)),
  InstPattern::new("0000001 ????? ????? 000 ????? 01110 11", Instruction::Register(RegisterType::MULW)),
  InstPattern::new("0000001 ????? ????? 100 ????? 01100 11", Instruction::Register(RegisterType::DIV)),
  InstPattern::new("0000001 ????? ????? 101 ????? 01100 11", Instruction::Register(RegisterType::DIVU)),
  InstPattern::new("0000001 ????? ????? 110 ????? 01100 11", Instruction::Register(RegisterType::REM)),
  InstPattern::new("0000001 ????? ????? 111 ????? 01100 11", Instruction::Register(RegisterType::REMU)),
    // Transfer Control
  InstPattern::new("0000000 00001 00000 000 00000 11100 11", Instruction::Immediate(ImmediateType::EBREAK)),
  InstPattern::new("0000000 00000 00000 000 00000 11100 11", Instruction::Immediate(ImmediateType::ECALL)),
    // TODO: CSR
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
      Instruction::Register(RegisterType::ADDW) => {self.gpr[rd] = sext((self.gpr[rs1] as i64 + self.gpr[rs2] as i64) as usize, 32) as u64;}
      Instruction::Register(RegisterType::SUB)  => {self.gpr[rd] = (self.gpr[rs1] as i64 - self.gpr[rs2] as i64) as u64;}
      Instruction::Register(RegisterType::XOR)  => {self.gpr[rd] = self.gpr[rs1] ^ self.gpr[rs2];}
      Instruction::Register(RegisterType::OR)   => {self.gpr[rd] = self.gpr[rs1] | self.gpr[rs2];}
      Instruction::Register(RegisterType::AND)  => {self.gpr[rd] = self.gpr[rs1] & self.gpr[rs2];}
      Instruction::Register(RegisterType::SLL)  => {self.gpr[rd] = self.gpr[rs1] << self.gpr[rs2];}
      Instruction::Register(RegisterType::SRL)  => {self.gpr[rd] = self.gpr[rs1] >> self.gpr[rs2];}
      Instruction::Register(RegisterType::SLT)  => {self.gpr[rd] = if (self.gpr[rs1] as i64) < (self.gpr[rs2] as i64) {1} else {0};}
      Instruction::Register(RegisterType::SLTU) => {self.gpr[rd] = if self.gpr[rs1] < self.gpr[rs2] {1} else {0};}

      Instruction::Immediate(ImmediateType::ADDI)  => {self.gpr[rd] = (self.gpr[rs1] as i64 + imm) as u64;}
      Instruction::Immediate(ImmediateType::ADDIW)  => {self.gpr[rd] = sext((self.gpr[rs1] as i64 + imm) as usize, 32) as u64;}
      Instruction::Immediate(ImmediateType::XORI)  => {self.gpr[rd] = self.gpr[rs1] ^ imm as u64;}
      Instruction::Immediate(ImmediateType::ORI)   => {self.gpr[rd] = self.gpr[rs1] | imm as u64;}
      Instruction::Immediate(ImmediateType::ANDI)  => {self.gpr[rd] = self.gpr[rs1] & imm as u64;}
      Instruction::Immediate(ImmediateType::SLLI)  => {self.gpr[rd] = self.gpr[rs1] << imm;}
      Instruction::Immediate(ImmediateType::SRLI)  => {self.gpr[rd] = self.gpr[rs1] >> imm;}
      Instruction::Immediate(ImmediateType::SRAI)  => {self.gpr[rd] = (self.gpr[rs1] as i64 >> imm) as u64;}
      Instruction::Immediate(ImmediateType::SLTI)  => {self.gpr[rd] = if (self.gpr[rs1] as i64) < imm {1} else {0};}
      Instruction::Immediate(ImmediateType::SLTIU) => {self.gpr[rd] = if self.gpr[rs1] < imm as u64 {1} else {0};}

      Instruction::Immediate(ImmediateType::LB)  => {sext(read_data((self.gpr[rs1] as i64 + imm) as u64, 1) as usize, 8);}
      Instruction::Immediate(ImmediateType::LBU) => {read_data((self.gpr[rs1] as i64 + imm) as u64, 1);}
      Instruction::Immediate(ImmediateType::LH)  => {sext(read_data((self.gpr[rs1] as i64 + imm) as u64, 2) as usize, 8);}
      Instruction::Immediate(ImmediateType::LHU) => {read_data((self.gpr[rs1] as i64 + imm) as u64, 2);}
      Instruction::Immediate(ImmediateType::LW)  => {sext(read_data((self.gpr[rs1] as i64 + imm) as u64, 4) as usize, 8);}
      Instruction::Immediate(ImmediateType::LWU) => {read_data((self.gpr[rs1] as i64 + imm) as u64, 4);}
      Instruction::Immediate(ImmediateType::LD)  => {sext(read_data((self.gpr[rs1] as i64 + imm) as u64, 8) as usize, 8);}
      Instruction::Immediate(ImmediateType::LDU) => {read_data((self.gpr[rs1] as i64 + imm) as u64, 8);}

      Instruction::Store(StoreType::SB) => {write_data((self.gpr[rs1] as i64 + imm) as u64, 1, self.gpr[rs2]);}
      Instruction::Store(StoreType::SH) => {write_data((self.gpr[rs1] as i64 + imm) as u64, 2, self.gpr[rs2]);}
      Instruction::Store(StoreType::SW) => {write_data((self.gpr[rs1] as i64 + imm) as u64, 4, self.gpr[rs2]);}
      Instruction::Store(StoreType::SD) => {write_data((self.gpr[rs1] as i64 + imm) as u64, 8, self.gpr[rs2]);}

      Instruction::Branch(BranchType::BEQ)  => {if self.gpr[rs1] == self.gpr[rs2] {self.pc = (self.pc as i64 + imm) as u64;}}
      Instruction::Branch(BranchType::BNE)  => {if self.gpr[rs1] != self.gpr[rs2] {self.pc = (self.pc as i64 + imm) as u64;}}
      Instruction::Branch(BranchType::BLT)  => {if (self.gpr[rs1] as i64) < (self.gpr[rs2] as i64) {self.pc = (self.pc as i64 + imm) as u64;}}
      Instruction::Branch(BranchType::BGE)  => {if (self.gpr[rs1] as i64) >= (self.gpr[rs2] as i64) {self.pc = (self.pc as i64 + imm) as u64;}}
      Instruction::Branch(BranchType::BLTU) => {if self.gpr[rs1] < self.gpr[rs2] {self.pc = (self.pc as i64 + imm) as u64;}}
      Instruction::Branch(BranchType::BGEU) => {if self.gpr[rs1] >= self.gpr[rs2] {self.pc = (self.pc as i64 + imm) as u64;}}

      Instruction::Jump(JumpType::JAL)            => {self.gpr[rd] = self.pc + 4; self.dnpc = (self.pc as i64 + imm) as u64;}
      Instruction::Immediate(ImmediateType::JALR) => {self.gpr[rd] = self.pc + 4; self.dnpc = (self.gpr[rs1] as i64 + imm) as u64;}

      Instruction::Upper(UpperType::LUI)   => {self.gpr[rd] = self.gpr[rs1];}
      Instruction::Upper(UpperType::AUIPC) => {self.gpr[rd] = (self.pc as i64 + imm) as u64;}

      Instruction::Immediate(ImmediateType::ECALL)  => {todo!();}
      Instruction::Immediate(ImmediateType::EBREAK) => {self.hemu_trap();}

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
