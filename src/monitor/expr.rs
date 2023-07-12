use eval::eval;
use regex::Regex;

trait HexConversion {
  fn hex(&self) -> String;
}

impl HexConversion for &str {
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

pub fn expr(expression: &str) -> f64 {
  let expression = expression.hex();
  log::info!("expr: {}", expression);
  eval(&expression).unwrap().as_f64().unwrap_or(114514.1919810)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  // add sub mul div
  fn test_expr_asmd() {
    assert_eq!(expr("1 + 2"), 3.0);
    assert_eq!(expr("1 + 2 + 3"), 6.0);
    assert_eq!(expr("1 + 2 * 3"), 7.0);
    assert_eq!(expr("1 / 2 * 3"), 1.5);
    assert_eq!(expr("1 / (2 * 3)"), 1.0 / 6.0);
    assert_eq!(expr("0"), 0.0);
    // if the expression is invalid, return 114514.0
    assert_eq!(expr("1 / 0"), 114514.1919810);
  }

  #[test]
  fn test_expr_0x() {
    assert_eq!(expr("0x1"), 1.0);
    assert_eq!(expr("0x10"), 16.0);
  }
}
