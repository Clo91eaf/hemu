use eval::eval;

pub fn expr(expression: &str) -> f64 {
  log::info!("expr: {}", expression);
  eval(expression).unwrap().as_f64().unwrap_or(114514.1919810)
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
}
