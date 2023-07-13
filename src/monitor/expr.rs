use crate::cpu::Cpu;
use atoi::atoi;
use eval::eval;
use regex::Regex;

trait HexConversion {
  fn hex(&self) -> String;
}

impl HexConversion for String {
  fn hex(&self) -> String {
    let re = Regex::new(r"0x([0-9a-fA-F]+)").unwrap();

    let result = re.replace_all(self, |caps: &regex::Captures| {
      let hex_number = &caps[1];
      let dec_number = u64::from_str_radix(hex_number, 16).unwrap();
      dec_number.to_string()
    });

    result.to_string()
  }
}

trait RegConversion {
  fn reg(&self, cpu: &Cpu) -> String;
}

impl RegConversion for String {
  fn reg(&self, cpu: &Cpu) -> String {
    let re = Regex::new(r"\$(\w+)").unwrap();

    let result = re.replace_all(self, |caps: &regex::Captures| {
      let reg_name = &caps[1];
      let reg_value = match reg_name {
        "pc" => cpu.pc,
        _ => {
          let re_gpr = Regex::new(r"x(\d+)").unwrap();
          if let Some(caps) = re_gpr.captures(reg_name) {
            let reg_index = atoi::<usize>(caps[1].as_bytes()).unwrap();
            cpu.gpr[reg_index]
          } else {
            panic!("Unknown register name: {}", reg_name);
          }
        }
      };
      reg_value.to_string()
    });

    result.to_string()
  }
}

pub fn expr(expression: String, cpu: &Cpu) -> u64 {
  let expression = expression.hex().reg(cpu);
  log::debug!("expr: {}", expression);
  let result = eval(&expression)
  .unwrap()
  .as_f64()
  .unwrap_or(114514.1919810);
  unsafe{ 
      result.to_int_unchecked::<u64>()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  // add sub mul div
  fn test_expr_asmd() {
    assert_eq!(expr("1 + 2".to_string(), cpu), 3);
    assert_eq!(expr("1 + 2 + 3".to_string(), cpu), 6);
    assert_eq!(expr("1 + 2 * 3".to_string(), cpu), 7);
    assert_eq!(expr("1 / 2 * 3".to_string(), cpu), 1);
    assert_eq!(expr("1 / (2 * 3)".to_string(), cpu), 0);
    assert_eq!(expr("0".to_string(), cpu), 0);
    // if the expression is invalid, return 114514.0
    assert_eq!(expr("1 / 0".to_string(), cpu), 114514);
  }

  #[test]
  fn test_expr_hex() {
    assert_eq!(expr("0x1".to_string(), cpu), 1);
    assert_eq!(expr("0x10".to_string(), cpu), 16);
  }

  #[test]
  fn test_expr_reg() {
    assert_eq!(expr("$pc".to_string(), cpu), cpu.pc);
    assert_eq!(expr("$x1".to_string(), cpu), cpu.gpr[1]);
  }
}
