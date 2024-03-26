use crate::lexer::Lexer;
use crate::tree::Tree;
use core::panic;
use std::collections::{HashMap};
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
  current_token: String,
  grammar: Grammar,
}

impl<'a> Parser<'a> {
  pub fn new(mut lexer: Lexer<'a>) -> Self {
    let current_token = lexer.next_token();
    let grammar = Grammar::new();
    Parser { lexer, current_token, grammar }
  }

  fn consume_token(&mut self) {
    self.current_token = self.lexer.next_token();
  }

  /// 自顶向下语法分析； 表格驱动语法分析
  pub fn parse(&mut self) -> Tree {
    let ast = self.parse_pg();
    ast
  }

  fn parse_pg(&mut self) -> Tree {
    println!("pg->Fn FnList");
    let f = self.parse_fn();
    let fn_list = self.parse_fn_list();
    Tree::new("Pg".to_string(), vec![f, fn_list])
  }

  fn parse_fn(&mut self) -> Tree {
    println!("Fn->Type Identifier Param FnBody FnList");
    let ty = self.parse_type();
    let id = self.parse_identifier();
    let pa = self.parse_param();
    let fb = self.parse_fn_body();
    Tree::new("Fn".to_string(), vec![ty, id, pa, fb])
  }

  fn parse_fn_list(&mut self) -> Tree {
    if self.current_token == "".to_string() {
      println!("FnList->ε");
      return Tree::new("FnList->ε".to_string(), vec![]);
    }
    println!("FnList->Fn FnList");
    let f = self.parse_fn();
    let fl = self.parse_fn_list();
    Tree::new("FnList".to_string(), vec![f, fl])
  }

  fn parse_type(&mut self) -> Tree {
    if self.current_token != "".to_string() {
      println!("Type->{}", self.current_token);
      let token = self.current_token.clone();
      self.consume_token();
      return Tree::new("Type->".to_string()+token.as_str(), vec![]);
    }
    else {
      panic!("parse_type error, expected Type, but got {}", self.current_token);
    }
  }

  fn parse_identifier(&mut self) -> Tree {
    if self.current_token != "".to_string() {
      println!("Identifier->{}", self.current_token);
      let token = self.current_token.clone();
      self.consume_token();
      return Tree::new("Identifier->".to_string()+token.as_str(), vec![]);
    }
    else {
      panic!("parse_identifier error, expected Identifier, but got {}", self.current_token);
    }
  }

  fn parse_param(&mut self) -> Tree {
    println!("Param->(ParamList)");
    let pl : Tree;
    if self.current_token == "(" {
      self.consume_token();
      pl = self.parse_param_list();
      if self.current_token == ")" {
        self.consume_token();
      } else{
        panic!("parse_param error, expected ), but got {}", self.current_token);
      }
    } else {
      panic!("parse_param error, expected (, but got {}", self.current_token);
    }
    return Tree::new("Param".to_string(), vec![pl]);
  }

  fn parse_param_list(&mut self) -> Tree {
    if self.current_token == ")".to_string() {
      println!("ParamList->ε");
      return Tree::new("ParamList->ε".to_string(), vec![]);
    }
    println!("ParamList->Type Identifier ParamListTail");
    let ty = self.parse_type();
    let id = self.parse_identifier();
    let plt = self.parse_param_list_tail();
    return Tree::new("ParamList".to_string(), vec![ty, id, plt]);
  }

  fn parse_param_list_tail(&mut self) -> Tree {
    if self.current_token == ")".to_string() {
      println!("ParamListTail->ε");
      return Tree::new("ParamListTail->ε".to_string(), vec![]);
    }
    println!("ParamListTail->, Type Identifier ParamListTail");
    let ty:Tree;
    let id:Tree;
    let plt:Tree;
    if self.current_token == "," {
      self.consume_token();
      ty = self.parse_type();
      id = self.parse_identifier();
      plt = self.parse_param_list_tail();
    }
    else {
      panic!("parse_param_list_tail error, expected , but got {}", self.current_token);
    }
    return Tree::new("ParamListTail".to_string(), vec![ty, id, plt]);
  }

  fn parse_fn_body(&mut self) -> Tree {
    println!("FnBody->{{StmtList}}");
    let sl : Tree;
    if self.current_token == "{" {
      self.consume_token();
      sl = self.parse_stmt_list();
      if self.current_token == "}" {
        self.consume_token();
      }
      else {
        panic!("parse_fn_body error, expected }} but got {}", self.current_token);
      }
    }
    else {
      panic!("parse_fn_body error, expected {{ but got {}", self.current_token);
    }
    Tree::new("FnBody".to_string(), vec![sl])
  }

  fn parse_stmt_list(&mut self) -> Tree {
    if self.current_token != "}" {
      println!("StmtList->Stmt StmtList");
      let s = self.parse_stmt();
      let sl = self.parse_stmt_list();
      return Tree::new("StmtList".to_string(), vec![s, sl]);
    }
    else {
      println!("StmtList->ε");
      return Tree::new("StmtList->ε".to_string(), vec![]);
    }

  }

  fn parse_stmt(&mut self) -> Tree {
    println!("Stmt->VarDecl");
    let vd = self.parse_var_decl();
    return Tree::new("Stmt".to_string(), vec![vd]);
    // match self.current_token.as_str() {
    //   "VarDecl" => self.parse_var_decl(),
    //   "VarDef" => self.parse_var_def(),
    //   "Assign" => self.parse_assign(),
    //   "pass" => self.consume_token(),
    //   _ => (),
    // }
  }

  fn parse_var_decl(&mut self) -> Tree {
    println!("VarDecl->Type Identifier");
    let ty = self.parse_type();
    let id = self.parse_identifier();
    return Tree::new("VarDecl".to_string(), vec![ty, id]);
  }

  fn parse_var_def(&mut self) -> Tree {
    println!("VarDef->Type Identifier = Expr");
    let ty = self.parse_type();
    let id = self.parse_identifier();
    if self.current_token == "=" {
      self.consume_token();
      let ex = self.parse_expr();
      return Tree::new("VarDef".to_string(), vec![ty, id, ex]);
    }
    else {
      panic!("parse_var_def error, expected = but got {}", self.current_token);
    }
  }

  fn parse_assign(&mut self) -> Tree {
    println!("Assign->Identifier = Expr");
    let id = self.parse_identifier();
    if self.current_token == "=" {
      self.consume_token();
      let ex = self.parse_expr();
      return Tree::new("Assign".to_string(), vec![id, ex]);
    }
    else {
      panic!("parse_assign error, expected = but got {}", self.current_token);
    }
  }

  fn parse_expr(&mut self) -> Tree {
    println!("Expr->Identifier");
    let id = self.parse_identifier();
    return Tree::new("Expr".to_string(), vec![id]);
    // match self.current_token.as_str() {
    //   "Identifier" => self.parse_identifier(),
    //   "Integer" => self.consume_token(),
    //   "Float" => self.consume_token(),
    //   "StringLiteral" => self.consume_token(),
    //   "(" => {
    //     self.consume_token();
    //     self.parse_expr();
    //     if self.current_token == ")" {
    //       self.consume_token();
    //     }
    //   }
    //   _ => {
    //     self.parse_expr();
    //     self.parse_op();
    //     self.parse_expr();
    //   }
    // }
  }

  fn parse_op(&mut self) -> Tree {
    match self.current_token.as_str() {
      "+" | "-" | "*" | "/" => {
        println!("Op->{}", self.current_token);
        let token = self.current_token.clone();
        self.consume_token();
        return Tree::new("Op->".to_string()+token.as_str(), vec![]);
      }
      _ => panic!("parse_op error, expected Op, but got {}", self.current_token),
    }
  }
}


enum Expr {
  Identifier(String),
  Integer(i32),
  Float(f64),
  StringLiteral(String),
  BinaryOp(Box<Expr>, Op, Box<Expr>),
  Paren(Box<Expr>),
}

enum Op {
  Add,
  Sub,
  Mul,
  Div,
}
