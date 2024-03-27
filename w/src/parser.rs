use crate::lexer::Lexer;
use crate::ast::AST;
use core::panic;
use std::collections::{HashMap, VecDeque};
use std::vec;

struct Grammar {
  terminators: Vec<String>,
  non_terminators: Vec<String>,
  start_symbol: String,
  rules: HashMap<String, Vec<Vec<String>>>,
}

impl Grammar {
  pub fn new() -> Self {
    let terminators = vec!["Identifier".to_string(), "Keyword".to_string()];
    let non_terminators = vec![
        "Pg".to_string(),
        "Fn".to_string(),
        "FnList".to_string(),
        "Type".to_string(),
        "Param".to_string(), // 参数 
        "ParamList".to_string(), // 参数列表
        "ParamListTail".to_string(), // 参数列表
        "FnBody".to_string(), // 函数体
        "StmtList".to_string(), // 语句列表
        "Stmt".to_string(), // 语句
        "VarDecl".to_string(), // 变量声明
        "VarDef".to_string(), // 变量定义
        "Assign".to_string(), // 赋值
        "Expr".to_string(), // 表达式
      ];
    let start_symbol = "Pg".to_string();
    let mut rules = HashMap::new();
    rules.insert("Pg".to_string(), vec![
      vec!["Fn".to_string(), "FnList".to_string()],
    ]);
    rules.insert("Fn".to_string(), vec![
      vec!["Type".to_string(), "FnName".to_string(), "Param".to_string(), "FnBody".to_string()],
    ]);
    rules.insert("FnList".to_string(), vec![
      vec!["Fn".to_string(), "FnList".to_string()],
      vec!["".to_string()],
    ]);
    rules.insert("Type".to_string(), vec![
      vec!["Keyword".to_string()],
    ]);
    rules.insert("FnName".to_string(), vec![
      vec!["Identifier".to_string()],
    ]);
    rules.insert("Param".to_string(), vec![
      vec!["(".to_string(), "ParamList".to_string(), ")".to_string()],
    ]);
    rules.insert("ParamList".to_string(), vec![
      vec!["Type".to_string(), "Identifier".to_string(), "ParamListTail".to_string()],
      vec!["".to_string()],
    ]);
    rules.insert("ParamListTail".to_string(), vec![
      vec![",".to_string(), "Type".to_string(), "Identifier".to_string(), "ParamListTail".to_string()],
      vec!["".to_string()],
    ]);
    rules.insert("FnBody".to_string(), vec![
      vec!["{".to_string(), "StmtList".to_string(), "}".to_string()],
    ]);
    rules.insert("StmtList".to_string(), vec![
      vec!["Stmt".to_string(), "StmtList".to_string()],
      vec!["".to_string()],
    ]);
    rules.insert("Stmt".to_string(), vec![
      vec!["VarDecl".to_string()],
      vec!["VarDef".to_string()],
      vec!["Assign".to_string()],
      vec!["pass".to_string()],
    ]);
    rules.insert("VarDecl".to_string(), vec![
      vec!["Type".to_string(), "Identifier".to_string()],
    ]);
    rules.insert("VarDef".to_string(), vec![
      vec!["Type".to_string(), "Identifier".to_string(), "=".to_string(), "Expr".to_string()],
    ]);
    rules.insert("Assign".to_string(), vec![
      vec!["Identifier".to_string(), "=".to_string(), "Expr".to_string()],
    ]);
    rules.insert("Expr".to_string(), vec![
      vec!["Identifier".to_string()],
      vec!["Integer".to_string()],
      vec!["Float".to_string()],
      vec!["StringLiteral".to_string()],
      vec!["(".to_string(), "Expr".to_string(), ")".to_string()],
      vec!["Expr".to_string(), "+".to_string(), "Expr".to_string()],
      vec!["Expr".to_string(), "-".to_string(), "Expr".to_string()],
      vec!["Expr".to_string(), "*".to_string(), "Expr".to_string()],
      vec!["Expr".to_string(), "/".to_string(), "Expr".to_string()],
    ]);

    Grammar {
      terminators,
      non_terminators,
      start_symbol,
      rules,
    }
  }   
}

pub struct Parser<'a> {
  lexer: Lexer<'a>,
  current_tokens: VecDeque<String>,
  grammar: Grammar,
}

impl<'a> Parser<'a> {
  pub fn new(mut lexer: Lexer<'a>) -> Self {
    let mut current_tokens = VecDeque::new();
    current_tokens.push_back(lexer.next_token());
    let grammar = Grammar::new();
    Parser { lexer, current_tokens: current_tokens, grammar }
  }

  fn consume_token(&mut self) {
    if self.current_tokens.len() <= 1 {
    self.prefetch_token();
    }
    self.current_tokens.pop_front();
  }

  fn prefetch_token(&mut self) {
    let token = self.lexer.next_token();
    self.current_tokens.push_back(token);
  }

  /// 自顶向下递归下降语法分析； 表格驱动语法分析
  pub fn parse(&mut self) -> AST {
    let ast = self.parse_pg();
    ast
  }

  fn parse_pg(&mut self) -> AST {
    println!("pg->Fn FnList");
    let f = self.parse_fn();
    let fn_list = self.parse_fn_list();
    AST::new("Pg".to_string(), vec![f, fn_list])
  }

  fn parse_fn(&mut self) -> AST {
    println!("Fn->Type Identifier Param FnBody FnList");
    let ty = self.parse_type();
    let id = self.parse_identifier();
    let pa = self.parse_param();
    let fb = self.parse_fn_body();
    AST::new("Fn".to_string(), vec![ty, id, pa, fb])
  }

  fn parse_fn_list(&mut self) -> AST {
    if self.current_tokens[0] == "".to_string() {
      println!("FnList->ε");
      return AST::new("FnList->ε".to_string(), vec![]);
    }
    println!("FnList->Fn FnList");
    let f = self.parse_fn();
    let fl = self.parse_fn_list();
    AST::new("FnList".to_string(), vec![f, fl])
  }

  fn parse_type(&mut self) -> AST {
    if self.current_tokens[0] != "".to_string() {
      println!("Type->{}", self.current_tokens[0]);
      let token = self.current_tokens[0].clone();
      self.consume_token();
      return AST::new("Type->".to_string()+token.as_str(), vec![]);
    }
    else {
      panic!("parse_type error, expected Type, but got {}", self.current_tokens[0]);
    }
  }

  fn parse_identifier(&mut self) -> AST {
    if self.current_tokens[0] != "".to_string() {
      println!("Identifier->{}", self.current_tokens[0]);
      let token = self.current_tokens[0].clone();
      self.consume_token();
      return AST::new("Identifier->".to_string()+token.as_str(), vec![]);
    }
    else {
      panic!("parse_identifier error, expected Identifier, but got {}", self.current_tokens[0]);
    }
  }

  fn parse_param(&mut self) -> AST {
    println!("Param->(ParamList)");
    let pl : AST;
    if self.current_tokens[0] == "(" {
      self.consume_token();
      pl = self.parse_param_list();
      if self.current_tokens[0] == ")" {
        self.consume_token();
      } else{
        panic!("parse_param error, expected ), but got {}", self.current_tokens[0]);
      }
    } else {
      panic!("parse_param error, expected (, but got {}", self.current_tokens[0]);
    }
    return AST::new("Param".to_string(), vec![pl]);
  }

  fn parse_param_list(&mut self) -> AST {
    if self.current_tokens[0] == ")".to_string() {
      println!("ParamList->ε");
      return AST::new("ParamList->ε".to_string(), vec![]);
    }
    println!("ParamList->Type Identifier ParamListTail");
    let ty = self.parse_type();
    let id = self.parse_identifier();
    let plt = self.parse_param_list_tail();
    return AST::new("ParamList".to_string(), vec![ty, id, plt]);
  }

  fn parse_param_list_tail(&mut self) -> AST {
    if self.current_tokens[0] == ")".to_string() {
      println!("ParamListTail->ε");
      return AST::new("ParamListTail->ε".to_string(), vec![]);
    }
    println!("ParamListTail->, Type Identifier ParamListTail");
    let ty:AST;
    let id:AST;
    let plt:AST;
    if self.current_tokens[0] == "," {
      self.consume_token();
      ty = self.parse_type();
      id = self.parse_identifier();
      plt = self.parse_param_list_tail();
    }
    else {
      panic!("parse_param_list_tail error, expected , but got {}", self.current_tokens[0]);
    }
    return AST::new("ParamListTail".to_string(), vec![ty, id, plt]);
  }

  fn parse_fn_body(&mut self) -> AST {
    println!("FnBody->{{StmtList}}");
    let sl : AST;
    if self.current_tokens[0] == "{" {
      self.consume_token();
      sl = self.parse_stmt_list();
      if self.current_tokens[0] == "}" {
        self.consume_token();
      }
      else {
        panic!("parse_fn_body error, expected }} but got {}", self.current_tokens[0]);
      }
    }
    else {
      panic!("parse_fn_body error, expected {{ but got {}", self.current_tokens[0]);
    }
    AST::new("FnBody".to_string(), vec![sl])
  }

  fn parse_stmt_list(&mut self) -> AST {
    if self.current_tokens[0] != "}" {
      println!("StmtList->Stmt StmtList");
      let s = self.parse_stmt();
      let sl = self.parse_stmt_list();
      return AST::new("StmtList".to_string(), vec![s, sl]);
    }
    else {
      println!("StmtList->ε");
      return AST::new("StmtList->ε".to_string(), vec![]);
    }

  }

  fn parse_stmt(&mut self) -> AST {
    if self.current_tokens[0] == "return" {
      println!("Stmt->return Expr");
      self.consume_token();
      let ex = self.parse_expr();
      return AST::new("Stmt->return".to_string(), vec![ex]);
    }
    while self.current_tokens.len() < 3 {
      self.prefetch_token();
    }
    if self.current_tokens[2] == "=".to_string() {
      println!("Stmt->VarDef");
      let vd = self.parse_var_def();
      return AST::new("Stmt".to_string(), vec![vd]);
    } else if self.current_tokens[1] == "=".to_string() {
      println!("Stmt->Assign");
      let a = self.parse_assign();
      return AST::new("Stmt".to_string(), vec![a]);
    } else if self.current_tokens[0] == "pass".to_string(){
      println!("Stmt->pass");
      self.consume_token();
      return AST::new("Stmt->pass".to_string(), vec![]);
    } else {
      println!("Stmt->VarDecl");
      let vd = self.parse_var_decl();
      return AST::new("Stmt".to_string(), vec![vd]);
    }
  }

  fn parse_var_decl(&mut self) -> AST {
    println!("VarDecl->Type Identifier");
    let ty = self.parse_type();
    let id = self.parse_identifier();
    return AST::new("VarDecl".to_string(), vec![ty, id]);
  }

  fn parse_var_def(&mut self) -> AST {
    println!("VarDef->Type Identifier = Expr");
    let ty = self.parse_type();
    let id = self.parse_identifier();
    if self.current_tokens[0] == "=" {
      self.consume_token();
      let ex = self.parse_expr();
      return AST::new("VarDef".to_string(), vec![ty, id, ex]);
    }
    else {
      panic!("parse_var_def error, expected = but got {}", self.current_tokens[0]);
    }
  }

  fn parse_assign(&mut self) -> AST {
    println!("Assign->Identifier = Expr");
    let id = self.parse_identifier();
    if self.current_tokens[0] == "=" {
      self.consume_token();
      let ex = self.parse_expr();
      return AST::new("Assign".to_string(), vec![id, ex]);
    }
    else {
      panic!("parse_assign error, expected = but got {}", self.current_tokens[0]);
    }
  }

  fn parse_expr(&mut self) -> AST {
    let left = self.parse_term();
    let op = self.current_tokens[0].clone();
    if op == "+" || op == "-" {
      self.consume_token();
      let right = self.parse_expr();
      println!("Expr->Term {} Expr", op);
      return AST::new("Expr->Op".to_string() + op.as_str(), vec![left, right]);
    } {
      println!("Expr->Term");
      return AST::new("Expr".to_string(), vec![left]);
    }
  }

  fn parse_term(&mut self) -> AST {
    let left = self.parse_factor();
    let op = self.current_tokens[0].clone();
    if op == "*" || op == "/" {
      self.consume_token();
      let right = self.parse_term();
      println!("Term->Factor {} Term", op);
      return AST::new("Term->Op".to_string() + op.as_str(), vec![left, right]);
    } else {
      println!("Term->Factor");
      return AST::new("Term".to_string(), vec![left]);
    }
  }

  fn parse_factor(&mut self) -> AST {
    if self.current_tokens[0] == "(" {
      println!("Factor->(Expr)");
      self.consume_token();
      let ex = self.parse_expr();
      if self.current_tokens[0] == ")" {
        self.consume_token();
      } else {
        panic!("parse_factor error, expected ) but got {}", self.current_tokens[0]);
      }
      return AST::new("Factor".to_string(), vec![ex]);
    } else {
      println!("Factor->Basic");
      let basic = self.current_tokens[0].clone();
      self.consume_token();
      return AST::new("Factor->Basic ".to_string()+basic.as_str(), vec![]);
    }
  }
}


// enum Expr {
//   Identifier(String),
//   Integer(i32),
//   Float(f64),
//   StringLiteral(String),
//   BinaryOp(Box<Expr>, Op, Box<Expr>),
//   Paren(Box<Expr>),
// }

// enum Op {
//   Add,
//   Sub,
//   Mul,
//   Div,
// }
