use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum TOKEN {
  NOTYPE,
  EQ,
  PLUS,
}

struct Rule {
  token_type: TOKEN,
  regex: Regex,
}

impl Rule {
  fn new(token_type: TOKEN, regex: Regex) -> Rule {
    Rule { token_type, regex }
  }
}

struct RuleTable {
  rules: [Rule; 3],
}

impl RuleTable {
  fn new() -> RuleTable {
    RuleTable {
      rules: [
        Rule::new(TOKEN::EQ, Regex::new(r"==").unwrap()),
        Rule::new(TOKEN::NOTYPE, Regex::new(r" +").unwrap()),
        Rule::new(TOKEN::PLUS, Regex::new(r"\+").unwrap()),
      ],
    }
  }

  fn get_token_type(&self, expr: &str) -> TOKEN {
    for rule in &self.rules {
      if rule.regex.is_match(expr) {
        return rule.token_type;
      }
    }
    TOKEN::NOTYPE
  }
}

pub fn init_regex() -> Expr {
  Expr::new()
}

struct Token {
  token_type: TOKEN,
  expr: String,
}

pub struct Expr {
  tokens: Vec<Token>,
}

impl Expr {
  fn new() -> Expr {
    // todo!("Expr::new()");
    Expr { tokens: Vec::new() }
  }
  pub fn expr(expr: &str) -> i64 {
    // todo!("Expr::expr()");
    let rule_table = RuleTable::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut expr = expr;
    while expr.len() > 0 {
      let token_type = rule_table.get_token_type(expr);
      let mut token = Token {
        token_type,
        expr: String::new(),
      };
      match token_type {
        TOKEN::NOTYPE => {
          expr = expr.trim_start();
          continue;
        }
        TOKEN::EQ => {
          token.expr.push_str("==");
          expr = &expr[2..];
        }
        TOKEN::PLUS => {
          token.expr.push_str("+");
          expr = &expr[1..];
        }
      }
      tokens.push(token);
    }
    0
  }
}
