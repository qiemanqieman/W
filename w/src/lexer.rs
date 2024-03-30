use std::{char, str::Chars};

pub struct Lexer<'a> {
  input: Chars<'a>,
  current_char: Option<char>,
  current_indent: i8,
  indent_left: i8,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    let mut chars = input.chars();
    let current_char = chars.next();

    Lexer {
      input: chars,
      current_char,
      current_indent: 0,
      indent_left: 0,
    }
  }

  fn advance(&mut self) {
    self.current_char = self.input.next();
  }

  fn skip_whitespace(&mut self) {
    while let Some(ch) = self.current_char {
      if !ch.is_whitespace() {
        break;
      }
      if ch == '\n' {
        break;
      }
      self.advance();
    }
  }

  fn compute_indent(&mut self) {
    let mut indent;
    if let Some(_) = self.current_char {
      indent = self.current_indent * 2;
      while let Some(ch) = self.current_char {
        if ch == ' ' {
          indent += 1;
          self.advance();
        } else {
          if ch == '\n' {
            self.advance();
            indent = 0;
          } else {
            break;
          };
        }
      }
    } else {
      indent = 0;
    }
    if self.current_char.is_none() {
      indent = 0;
    }

    indent /= 2;
    if indent != self.current_indent {
      self.indent_left = indent - self.current_indent;
      self.current_indent = indent;
    }
  }

  fn read_identifier(&mut self) -> String {
    let mut identifier = String::new();
    while let Some(ch) = self.current_char {
      if ch.is_alphanumeric() || ch == '_' {
        identifier.push(ch);
        self.advance();
      } else {
        break;
      }
    }
    identifier
  }

  fn read_keyword_or_identifier(&mut self) -> String {
    let keyword_or_identifier = self.read_identifier();
    keyword_or_identifier
  }

  fn read_number(&mut self) -> String {
    let mut number = String::new();

    while let Some(ch) = self.current_char {
      if ch.is_digit(10) {
        number.push(ch);
        self.advance();
      } else {
        break;
      }
    }

    if let Some('.') = self.current_char {
      number.push('.');
      self.advance();

      while let Some(ch) = self.current_char {
        if ch.is_digit(10) {
          number.push(ch);
          self.advance();
        } else {
          break;
        }
      }
    }
    number
  }

  fn read_string_literal(&mut self) -> String {
    let mut string_literal = String::new();

    self.advance();

    while let Some(ch) = self.current_char {
      if ch == '"' {
        self.advance();
        break;
      } else {
        string_literal.push(ch);
        self.advance();
      }
    }

    string_literal
  }

  /// Reads an operator from the lexer's input stream.
  ///
  /// # Examples
  ///
  /// ```
  /// # use crate::lexer::Lexer;
  /// let mut lexer = Lexer::new("+-*/%=&|".chars().collect());
  ///
  /// assert_eq!(lexer.read_operator(), "+-*/%=&|".to_string());
  /// ```
  fn read_operator(&mut self) -> String {
    let mut operator = String::new();

    while let Some(ch) = self.current_char {
      if ch.is_ascii_punctuation() {
        operator.push(ch);
        self.advance();
      } else {
        break;
      }
    }

    operator
  }

  fn read_delimiter(&mut self) -> String {
    let delimiter = self.current_char.unwrap().to_string();
    self.advance();
    delimiter
  }

  pub fn next_token(&mut self) -> String {
    self.skip_whitespace();
    self.compute_indent();
    if self.indent_left > 0 {
      self.indent_left -= 1;
      return "{".to_string();
    } else if self.indent_left < 0 {
      self.indent_left += 1;
      return "}".to_string();
    }

    if let Some(ch) = self.current_char {
      match ch {
        '0'..='9' => self.read_number(),
        '"' => self.read_string_literal(),
        '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '~' => {
          self.read_operator()
        }
        '(' | ')' | '[' | ']' | ',' | ':' => self.read_delimiter(),
        _ => {
          let identifier = self.read_keyword_or_identifier();
          match identifier.as_str() {
            "if" | "else" | "while" | "for" | "int" | "return" | "true" | "false" | "auto" => {
              identifier
            }
            _ => identifier,
          }
        }
      }
    } else {
      "".to_string()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_number() {
    let mut lexer = Lexer::new("1234");
    assert_eq!(lexer.read_number(), "1234");
  }

  #[test]
  fn test_read_string_literal() {
    let mut lexer = Lexer::new("\"Hello, World!\"");
    assert_eq!(lexer.read_string_literal(), "Hello, World!");
  }

  #[test]
  fn test_read_operator() {
    let mut lexer = Lexer::new("+-*/%=&|<>!");
    assert_eq!(lexer.read_operator(), "+-*/%=&|<>!");
  }

  #[test]
  fn test_read_keyword_or_identifier() {
    let mut lexer = Lexer::new("if else while for int return true false auto");
    assert_eq!(lexer.read_keyword_or_identifier(), "if");
    lexer.skip_whitespace();
    assert_eq!(lexer.read_keyword_or_identifier(), "else");
    lexer.skip_whitespace();
    assert_eq!(lexer.read_keyword_or_identifier(), "while");
    lexer.skip_whitespace();
    assert_eq!(lexer.read_keyword_or_identifier(), "for");
    lexer.skip_whitespace();
    assert_eq!(lexer.read_keyword_or_identifier(), "int");
    lexer.skip_whitespace();
    assert_eq!(lexer.read_keyword_or_identifier(), "return");
    lexer.skip_whitespace();
    assert_eq!(lexer.read_keyword_or_identifier(), "true");
    lexer.skip_whitespace();
    assert_eq!(lexer.read_keyword_or_identifier(), "false");
    lexer.skip_whitespace();
    assert_eq!(lexer.read_keyword_or_identifier(), "auto");
  }

  #[test]
  fn test_read_delimiter() {
    let mut lexer = Lexer::new("()[],:");
    assert_eq!(lexer.read_delimiter(), "(");
    assert_eq!(lexer.read_delimiter(), ")");
    assert_eq!(lexer.read_delimiter(), "[");
    assert_eq!(lexer.read_delimiter(), "]");
    assert_eq!(lexer.read_delimiter(), ",");
    assert_eq!(lexer.read_delimiter(), ":");
  }

  #[test]
  fn test_next_token() {
    let mut lexer = Lexer::new("1234 \"Hello, World!\" +-*/%=&|<>! ()[],:");
    assert_eq!(lexer.next_token(), "1234");
    assert_eq!(lexer.next_token(), "Hello, World!");
    assert_eq!(lexer.next_token(), "+-*/%=&|<>!");
    assert_eq!(lexer.next_token(), "(");
    assert_eq!(lexer.next_token(), ")");
    assert_eq!(lexer.next_token(), "[");
    assert_eq!(lexer.next_token(), "]");
    assert_eq!(lexer.next_token(), ",");
    assert_eq!(lexer.next_token(), ":");
  }
}
