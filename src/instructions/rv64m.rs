use super::*;

#[rustfmt::skip]
fn rv32m() -> Vec<InstPattern> {
  vec![
  // rv32M
  InstPattern::new("mul", "0000001 ????? ????? 000 ????? 01100 11", Instruction::Register(RegisterType::MUL)),
  InstPattern::new("mulh", "0000001 ????? ????? 001 ????? 01100 11", Instruction::Register(RegisterType::MULH)),
  InstPattern::new("mulhsu", "0000001 ????? ????? 010 ????? 01100 11", Instruction::Register(RegisterType::MULHSU)),
  InstPattern::new("mulhu", "0000001 ????? ????? 011 ????? 01100 11", Instruction::Register(RegisterType::MULHU)),
  InstPattern::new("div", "0000001 ????? ????? 100 ????? 01100 11", Instruction::Register(RegisterType::DIV)),
  InstPattern::new("divu", "0000001 ????? ????? 101 ????? 01100 11", Instruction::Register(RegisterType::DIVU)),
  InstPattern::new("rem", "0000001 ????? ????? 110 ????? 01100 11", Instruction::Register(RegisterType::REM)),
  InstPattern::new("remu", "0000001 ????? ????? 111 ????? 01100 11", Instruction::Register(RegisterType::REMU)),
  ]
}

#[rustfmt::skip]
pub fn rv64m() -> Vec<InstPattern> {
  let mut m = vec![
  // rv64M
  InstPattern::new("mulw", "0000001 ????? ????? 000 ????? 01110 11", Instruction::Register(RegisterType::MULW)),
  InstPattern::new("divw", "0000001 ????? ????? 100 ????? 01110 11", Instruction::Register(RegisterType::DIVW)),
  InstPattern::new("divuw", "0000001 ????? ????? 100 ????? 01110 11", Instruction::Register(RegisterType::DIVUW)),
  InstPattern::new("remw", "0000001 ????? ????? 110 ????? 01110 11", Instruction::Register(RegisterType::REMW)),
  InstPattern::new("remuw", "0000001 ????? ????? 110 ????? 01110 11", Instruction::Register(RegisterType::REMUW)),
  ];
  m.extend(rv32m());
  m
}
