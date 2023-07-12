use regex::Regex;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Token {
  Number(u64),
  HexadecimalNumber(u64),
  RegName(String),
  LParen,
  RParen,
  Plus,
  Minus,
  Multiply,
  Divide,
  Equals,
  NotEquals,
  And,
  Pointer,
}
fn tokenize(input: String) -> Vec<Token> {
  let decimal_number_re = Regex::new(r"^-?\d+(\.\d+)?").unwrap(); // Regex for decimal numbers
  let hexadecimal_number_re = Regex::new(r"^0x[0-9a-fA-F]+").unwrap(); // Regex for hexadecimal numbers
  let reg_name_re = Regex::new(r"^\$[a-zA-Z_][a-zA-Z0-9_]*").unwrap(); // Regex for register names
  let token_re = Regex::new(r"^\s*(==|!=|&&|\(|\)|\+|-|\*|/|->|[0-9a-zA-Z_][0-9a-zA-Z_]*|0x[0-9a-fA-F]+)").unwrap(); // Regex for all tokens

  let mut tokens: Vec<Token> = Vec::new();
  let mut remaining_input = input.replace(" ", "");

  while !remaining_input.is_empty() {
    let caps = token_re.captures(remaining_input.as_str()).unwrap();
    let token = caps.get(1).unwrap().as_str();

    if decimal_number_re.is_match(token) {
      let num: u64 = token.parse().unwrap();
      tokens.push(Token::Number(num));
    } else if hexadecimal_number_re.is_match(token) {
      let num = u64::from_str_radix(&token[2..], 16).unwrap();
      tokens.push(Token::HexadecimalNumber(num));
    } else if reg_name_re.is_match(token) {
      let name = token[1..].to_string();
      tokens.push(Token::RegName(name));
    } else {
      match token {
        "+" => tokens.push(Token::Plus),
        "-" => tokens.push(Token::Minus),
        "*" => {
          if let Some(Token::Number(_))
          | Some(Token::HexadecimalNumber(_))
          | Some(Token::RegName(_))
          | Some(Token::RParen) = tokens.last()
          {
            tokens.push(Token::Multiply);
          } else {
            tokens.push(Token::Pointer);
          }
        }
        "/" => tokens.push(Token::Divide),
        "==" => tokens.push(Token::Equals),
        "!=" => tokens.push(Token::NotEquals),
        "&&" => tokens.push(Token::And),
        "(" => tokens.push(Token::LParen),
        ")" => tokens.push(Token::RParen),
        _ => panic!("Invalid token: {}", token),
      }
    }

    let token_len = token.len();
    remaining_input.drain(..token_len);
  }

  tokens
}

fn evaluate(tokens: &mut Vec<Token>) -> u64 {
  let mut tokens_iter = tokens.iter().rev();
  parse_expression(&mut tokens_iter)
}

fn parse_expression(
  tokens_iter: &mut std::iter::Rev<std::slice::Iter<Token>>,
) -> u64 {
  let mut result = parse_term(tokens_iter);

  while let Some(token) = tokens_iter.next() {
    match token {
      Token::Plus => result += parse_term(tokens_iter),
      Token::Minus => result -= parse_term(tokens_iter),
      Token::Equals => {
        result = (result == parse_term(tokens_iter)) as u8 as u64
      }
      Token::NotEquals => {
        result = (result != parse_term(tokens_iter)) as u8 as u64
      }
      Token::And => result *= parse_term(tokens_iter),
      Token::RParen => break,
      _ => panic!("Invalid token: {:?}", token),
    }
  }

  result
}

// This function is called when the parser encounters a multiplication or division operator
fn parse_term(
  tokens_iter: &mut std::iter::Rev<std::slice::Iter<Token>>,
) -> u64 {
  let mut result = parse_factor(tokens_iter);

  while let Some(token) = tokens_iter.next() {
    match token {
      Token::Multiply => {
        result *= parse_factor(tokens_iter);
      }
      Token::Divide => {
        result /= parse_factor(tokens_iter);
      }
      Token::Plus
      | Token::Minus
      | Token::RParen
      | Token::Equals
      | Token::NotEquals
      | Token::And => break,
      _ => panic!("Invalid token: {:?}", token),
    }
  }

  result
}

fn parse_factor(
  tokens_iter: &mut std::iter::Rev<std::slice::Iter<Token>>,
) -> u64 {
  match tokens_iter.next().unwrap() {
    Token::Number(num) => *num,
    Token::HexadecimalNumber(num) => *num as u64,
    Token::RegName(_) => {
      // Placeholder logic, you can implement the evaluation of register values here
      0
    }
    Token::LParen => {
      let result = parse_expression(tokens_iter);
      assert_eq!(tokens_iter.next(), Some(&Token::RParen));
      result
    }
    Token::Pointer => {
      let result = parse_expression(tokens_iter);
      result // Placeholder logic, you can implement pointer dereference here
    }
    _ => unreachable!(),
  }
}

pub fn expr(expression: &str) -> u64 {
  let mut tokens = tokenize(expression.to_owned());
  evaluate(&mut tokens) as u64;
  todo!("expr not implemented!")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_expr() {
    assert_eq!(expr("1 + 2"), 3);
    // assert_eq!(expr("1 + 2 + 3"), 6);
    // assert_eq!(expr("1 + 2 * 3"), 7);
    // assert_eq!(expr("1 / 2 * 3"), 0);
    // assert_eq!(expr("1 / (2 * 3)"), 0);
    // assert_eq!(expr("0"), 0);
  }
}
