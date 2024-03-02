mod instruction;
pub mod memory;
mod statistic;
mod utils;

use ansi_term::Colour::{Green, Red};
use instruction::BranchType as B;
use instruction::ImmediateType as I;
use instruction::InstPattern as IP;
use instruction::Instruction as Inst;
use instruction::JumpType as J;
use instruction::RegisterType as R;
use instruction::StoreType as S;
use instruction::UpperType as U;

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
    self.inst = read_inst(self.snpc) as u32;
    self.snpc += 4;
  }

  pub fn decode(&self, inst_type: &mut Inst) {
    #[rustfmt::skip]
    let patterns = [
    // Register 
  IP::new("0000000 ????? ????? 000 ????? 01100 11", Inst::Register(R::ADD)),
  IP::new("0000000 ????? ????? 000 ????? 01110 11", Inst::Register(R::ADDW)),
  IP::new("0100000 ????? ????? 000 ????? 01100 11", Inst::Register(R::SUB)),
  IP::new("0000000 ????? ????? 100 ????? 01100 11", Inst::Register(R::XOR)),
  IP::new("0000000 ????? ????? 110 ????? 01100 11", Inst::Register(R::OR)),
  IP::new("0000000 ????? ????? 111 ????? 01100 11", Inst::Register(R::AND)),
  IP::new("0000000 ????? ????? 001 ????? 01100 11", Inst::Register(R::SLL)),
  IP::new("0000000 ????? ????? 101 ????? 01100 11", Inst::Register(R::SRL)),
  IP::new("0000000 ????? ????? 010 ????? 01100 11", Inst::Register(R::SLT)),
  IP::new("0000000 ????? ????? 011 ????? 01100 11", Inst::Register(R::SLTU)),
    // Immediate
  IP::new("??????? ????? ????? 000 ????? 00100 11", Inst::Immediate(I::ADDI)),
  IP::new("??????? ????? ????? 000 ????? 00110 11", Inst::Immediate(I::ADDIW)),
  IP::new("??????? ????? ????? 100 ????? 00100 11", Inst::Immediate(I::XORI)),
  IP::new("??????? ????? ????? 110 ????? 00100 11", Inst::Immediate(I::ORI)),
  IP::new("??????? ????? ????? 111 ????? 00100 11", Inst::Immediate(I::ANDI)),
  IP::new("000000? ????? ????? 001 ????? 00100 11", Inst::Immediate(I::SLLI)),
  IP::new("000000? ????? ????? 101 ????? 00100 11", Inst::Immediate(I::SRLI)),
  IP::new("010000? ????? ????? 101 ????? 00100 11", Inst::Immediate(I::SRAI)),
  IP::new("??????? ????? ????? 010 ????? 00100 11", Inst::Immediate(I::SLTI)),
  IP::new("??????? ????? ????? 011 ????? 00100 11", Inst::Immediate(I::SLTIU)),
  IP::new("??????? ????? ????? 000 ????? 00000 11", Inst::Immediate(I::LB)),
  IP::new("??????? ????? ????? 100 ????? 00000 11", Inst::Immediate(I::LBU)),
  IP::new("??????? ????? ????? 001 ????? 00000 11", Inst::Immediate(I::LH)),
  IP::new("??????? ????? ????? 101 ????? 00000 11", Inst::Immediate(I::LHU)),
  IP::new("??????? ????? ????? 010 ????? 00000 11", Inst::Immediate(I::LW)),
  IP::new("??????? ????? ????? 110 ????? 00000 11", Inst::Immediate(I::LWU)),
  IP::new("??????? ????? ????? 011 ????? 00000 11", Inst::Immediate(I::LD)),
  IP::new("??????? ????? ????? 111 ????? 00000 11", Inst::Immediate(I::LDU)),
  IP::new("??????? ????? ????? 000 ????? 11001 11", Inst::Immediate(I::JALR)),
    // Store
  IP::new("??????? ????? ????? 000 ????? 01000 11", Inst::Store(S::SB)),
  IP::new("??????? ????? ????? 001 ????? 01000 11", Inst::Store(S::SH)),
  IP::new("??????? ????? ????? 010 ????? 01000 11", Inst::Store(S::SW)),
  IP::new("??????? ????? ????? 011 ????? 01000 11", Inst::Store(S::SD)),
    // Branch
  IP::new("??????? ????? ????? 000 ????? 11000 11", Inst::Branch(B::BEQ)),
  IP::new("??????? ????? ????? 001 ????? 11000 11", Inst::Branch(B::BNE)),
  IP::new("??????? ????? ????? 100 ????? 11000 11", Inst::Branch(B::BLT)),
  IP::new("??????? ????? ????? 101 ????? 11000 11", Inst::Branch(B::BGE)),
  IP::new("??????? ????? ????? 110 ????? 11000 11", Inst::Branch(B::BLTU)),
  IP::new("??????? ????? ????? 111 ????? 11000 11", Inst::Branch(B::BGEU)),
    // Jump
  IP::new("??????? ????? ????? ??? ????? 11011 11", Inst::Jump(J::JAL)),
    // Upper
  IP::new("??????? ????? ????? ??? ????? 01101 11", Inst::Upper(U::LUI)),
  IP::new("??????? ????? ????? ??? ????? 00101 11", Inst::Upper(U::AUIPC)),
    // RV32M
  IP::new("0000001 ????? ????? 000 ????? 01100 11", Inst::Register(R::MUL)),
  IP::new("0000001 ????? ????? 000 ????? 01110 11", Inst::Register(R::MULW)),
  IP::new("0000001 ????? ????? 100 ????? 01100 11", Inst::Register(R::DIV)),
  IP::new("0000001 ????? ????? 101 ????? 01100 11", Inst::Register(R::DIVU)),
  IP::new("0000001 ????? ????? 110 ????? 01100 11", Inst::Register(R::REM)),
  IP::new("0000001 ????? ????? 111 ????? 01100 11", Inst::Register(R::REMU)),
    // Transfer Control
  IP::new("0000000 00001 00000 000 00000 11100 11", Inst::Immediate(I::EBREAK)),
  IP::new("0000000 00000 00000 000 00000 11100 11", Inst::Immediate(I::ECALL)),
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
  pub fn execute(&mut self, inst_type: Inst) {
    let (rd, rs1, rs2, imm) = decode_operand(self.inst, inst_type);
    self.dnpc = self.snpc;
    match inst_type {
      Inst::Register(R::ADD)  => {self.gpr[rd] = (self.gpr[rs1] as i64 + self.gpr[rs2] as i64) as u64;}
      Inst::Register(R::ADDW) => {self.gpr[rd] = sext((self.gpr[rs1] as i64 + self.gpr[rs2] as i64) as usize, 32) as u64;}
      Inst::Register(R::SUB)  => {self.gpr[rd] = (self.gpr[rs1] as i64 - self.gpr[rs2] as i64) as u64;}
      Inst::Register(R::XOR)  => {self.gpr[rd] = self.gpr[rs1] ^ self.gpr[rs2];}
      Inst::Register(R::OR)   => {self.gpr[rd] = self.gpr[rs1] | self.gpr[rs2];}
      Inst::Register(R::AND)  => {self.gpr[rd] = self.gpr[rs1] & self.gpr[rs2];}
      Inst::Register(R::SLL)  => {self.gpr[rd] = self.gpr[rs1] << self.gpr[rs2];}
      Inst::Register(R::SRL)  => {self.gpr[rd] = self.gpr[rs1] >> self.gpr[rs2];}
      Inst::Register(R::SLT)  => {self.gpr[rd] = if (self.gpr[rs1] as i64) < (self.gpr[rs2] as i64) {1} else {0};}
      Inst::Register(R::SLTU) => {self.gpr[rd] = if self.gpr[rs1] < self.gpr[rs2] {1} else {0};}

      Inst::Immediate(I::ADDI)  => {self.gpr[rd] = (self.gpr[rs1] as i64 + imm) as u64;}
      Inst::Immediate(I::ADDIW) => {self.gpr[rd] = sext((self.gpr[rs1] as i64 + imm) as usize, 32) as u64;}
      Inst::Immediate(I::XORI)  => {self.gpr[rd] = self.gpr[rs1] ^ imm as u64;}
      Inst::Immediate(I::ORI)   => {self.gpr[rd] = self.gpr[rs1] | imm as u64;}
      Inst::Immediate(I::ANDI)  => {self.gpr[rd] = self.gpr[rs1] & imm as u64;}
      Inst::Immediate(I::SLLI)  => {self.gpr[rd] = self.gpr[rs1] << imm;}
      Inst::Immediate(I::SRLI)  => {self.gpr[rd] = self.gpr[rs1] >> imm;}
      Inst::Immediate(I::SRAI)  => {self.gpr[rd] = (self.gpr[rs1] as i64 >> (if imm < 64 {imm as i64} else {64})) as u64;}
      Inst::Immediate(I::SLTI)  => {self.gpr[rd] = if (self.gpr[rs1] as i64) < imm {1} else {0};}
      Inst::Immediate(I::SLTIU) => {self.gpr[rd] = if self.gpr[rs1] < imm as u64 {1} else {0};}

      Inst::Immediate(I::LB)  => {sext(read_data((self.gpr[rs1] as i64 + imm) as u64, 1) as usize, 8);}
      Inst::Immediate(I::LBU) => {read_data((self.gpr[rs1] as i64 + imm) as u64, 1);}
      Inst::Immediate(I::LH)  => {sext(read_data((self.gpr[rs1] as i64 + imm) as u64, 2) as usize, 8);}
      Inst::Immediate(I::LHU) => {read_data((self.gpr[rs1] as i64 + imm) as u64, 2);}
      Inst::Immediate(I::LW)  => {sext(read_data((self.gpr[rs1] as i64 + imm) as u64, 4) as usize, 8);}
      Inst::Immediate(I::LWU) => {read_data((self.gpr[rs1] as i64 + imm) as u64, 4);}
      Inst::Immediate(I::LD)  => {sext(read_data((self.gpr[rs1] as i64 + imm) as u64, 8) as usize, 8);}
      Inst::Immediate(I::LDU) => {read_data((self.gpr[rs1] as i64 + imm) as u64, 8);}

      Inst::Store(S::SB) => {write_data((self.gpr[rs1] as i64 + imm) as u64, 1, self.gpr[rs2]);}
      Inst::Store(S::SH) => {write_data((self.gpr[rs1] as i64 + imm) as u64, 2, self.gpr[rs2]);}
      Inst::Store(S::SW) => {write_data((self.gpr[rs1] as i64 + imm) as u64, 4, self.gpr[rs2]);}
      Inst::Store(S::SD) => {write_data((self.gpr[rs1] as i64 + imm) as u64, 8, self.gpr[rs2]);}

      Inst::Branch(B::BEQ)  => {if self.gpr[rs1] == self.gpr[rs2] {self.dnpc = (self.pc as i64 + imm) as u64;}}
      Inst::Branch(B::BNE)  => {if self.gpr[rs1] != self.gpr[rs2] {self.dnpc = (self.pc as i64 + imm) as u64;}}
      Inst::Branch(B::BLT)  => {if (self.gpr[rs1] as i64) < (self.gpr[rs2] as i64) {self.dnpc = (self.pc as i64 + imm) as u64;}}
      Inst::Branch(B::BGE)  => {if (self.gpr[rs1] as i64) >= (self.gpr[rs2] as i64) {self.dnpc = (self.pc as i64 + imm) as u64;}}
      Inst::Branch(B::BLTU) => {if self.gpr[rs1] < self.gpr[rs2] {self.dnpc = (self.pc as i64 + imm) as u64;}}
      Inst::Branch(B::BGEU) => {if self.gpr[rs1] >= self.gpr[rs2] {self.dnpc = (self.pc as i64 + imm) as u64;}}

      Inst::Jump(J::JAL)            => {self.gpr[rd] = self.pc + 4; self.dnpc = (self.pc as i64 + imm) as u64;}
      Inst::Immediate(I::JALR) => {self.gpr[rd] = self.pc + 4; self.dnpc = (self.gpr[rs1] as i64 + imm) as u64;}

      Inst::Upper(U::LUI)   => {self.gpr[rd] = self.gpr[rs1];}
      Inst::Upper(U::AUIPC) => {self.gpr[rd] = (self.pc as i64 + imm) as u64;}

      Inst::Immediate(I::ECALL)  => {todo!();}
      Inst::Immediate(I::EBREAK) => {self.hemu_trap();}

      _ => {todo!("{:?} not implemented", inst_type);}
    }
    self.gpr[0] = 0;
  }

  fn exec_once(&mut self) {
    // pipeline start
    let mut inst_type = Inst::Immediate(I::EBREAK);
    self.snpc = self.pc;
    // fetch stage
    self.fetch();
    // decode stage
    self.decode(&mut inst_type);
    // execute stage (including memory stage and write back stage)
    self.execute(inst_type);
    // print disassemble
    log::info!("{:08x}: {:08x}\t{}", self.pc, self.inst, utils::disassemble(self.inst, inst_type));
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
