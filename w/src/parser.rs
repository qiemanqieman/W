use crate::ast::AST;
use crate::lexer::Lexer;
use core::panic;
use std::collections::{HashSet, VecDeque};
use std::vec;
pub struct Parser<'a> {
  lexer: Lexer<'a>,
  current_tokens: VecDeque<String>,
  keywords: HashSet<String>,
}

impl<'a> Parser<'a> {
  pub fn new(mut lexer: Lexer<'a>) -> Self {
    let mut current_tokens = VecDeque::new();
    current_tokens.push_back(lexer.next_token());
    let mut keywords = HashSet::new();
    keywords.insert("if".to_string());
    keywords.insert("else".to_string());
    keywords.insert("return".to_string());
    keywords.insert("pass".to_string());
    Parser {
      lexer,
      current_tokens: current_tokens,
      keywords,
    }
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
      return AST::new(
        "FnList".to_string(),
        vec![AST::new("ε".to_string(), vec![])],
      );
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
      return AST::new(token, vec![]);
    } else {
      panic!(
        "parse_type error, expected Type, but got {}",
        self.current_tokens[0]
      );
    }
  }

  fn parse_identifier(&mut self) -> AST {
    if self.current_tokens[0] != "".to_string() {
      println!("Identifier->{}", self.current_tokens[0]);
      let token = self.current_tokens[0].clone();
      self.consume_token();
      return AST::new(token, vec![]);
    } else {
      panic!(
        "parse_identifier error, expected Identifier, but got {}",
        self.current_tokens[0]
      );
    }
  }

  fn parse_param(&mut self) -> AST {
    println!("Param->(ParamList)");
    let pl: AST;
    if self.current_tokens[0] == "(" {
      self.consume_token();
      pl = self.parse_param_list();
      if self.current_tokens[0] == ")" {
        self.consume_token();
      } else {
        panic!(
          "parse_param error, expected ), but got {}",
          self.current_tokens[0]
        );
      }
    } else {
      panic!(
        "parse_param error, expected (, but got {}",
        self.current_tokens[0]
      );
    }
    return AST::new("Param".to_string(), vec![pl]);
  }

  fn parse_param_list(&mut self) -> AST {
    if self.current_tokens[0] == ")".to_string() {
      println!("ParamList->ε");
      return AST::new(
        "ParamList".to_string(),
        vec![AST::new("ε".to_string(), vec![])],
      );
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
      return AST::new(
        "ParamListTail".to_string(),
        vec![AST::new("ε".to_string(), vec![])],
      );
    }
    println!("ParamListTail->, Type Identifier ParamListTail");
    let ty: AST;
    let id: AST;
    let plt: AST;
    if self.current_tokens[0] == "," {
      self.consume_token();
      ty = self.parse_type();
      id = self.parse_identifier();
      plt = self.parse_param_list_tail();
    } else {
      panic!(
        "parse_param_list_tail error, expected , but got {}",
        self.current_tokens[0]
      );
    }
    return AST::new("ParamListTail".to_string(), vec![ty, id, plt]);
  }

  fn parse_fn_body(&mut self) -> AST {
    println!("FnBody->{{StmtList}}");
    let sl: AST;
    if self.current_tokens[0] == "{" {
      self.consume_token();
      sl = self.parse_stmt_list();
      if self.current_tokens[0] == "}" {
        self.consume_token();
      } else {
        panic!(
          "parse_fn_body error, expected }} but got {}",
          self.current_tokens[0]
        );
      }
    } else {
      panic!(
        "parse_fn_body error, expected {{ but got {}",
        self.current_tokens[0]
      );
    }
    AST::new("FnBody".to_string(), vec![sl])
  }

  fn parse_stmt_list(&mut self) -> AST {
    if self.current_tokens[0] != "}" {
      println!("StmtList->Stmt StmtList");
      let s = self.parse_stmt();
      let sl = self.parse_stmt_list();
      return AST::new("StmtList".to_string(), vec![s, sl]);
    } else {
      println!("StmtList->ε");
      return AST::new(
        "StmtList".to_string(),
        vec![AST::new("ε".to_string(), vec![])],
      );
    }
  }

  fn parse_branch_stmt(&mut self) -> AST {
    println!("BranchStmt->if Expr {{ StmtList }} else {{ StmtList }}");
    self.consume_token(); // if token
    let ex = self.parse_expr();
    self.consume_token(); // { token
    let sl1 = self.parse_stmt_list();
    self.consume_token(); // } token
    self.consume_token(); // else token
    self.consume_token(); // { token
    let sl2 = self.parse_stmt_list();
    self.consume_token(); // } token
    return AST::new("BranchStmt".to_string(), vec![ex, sl1, sl2]);
  }

  fn parse_stmt(&mut self) -> AST {
    if self.current_tokens[0] == "if" {
      println!("Stmt->BranchStmt");
      let bs = self.parse_branch_stmt();
      return AST::new("Stmt".to_string(), vec![bs]);
    }
    if self.current_tokens[0] == "return" {
      println!("Stmt->Return");
      let rtn = self.parse_return();
      return AST::new("Stmt".to_string(), vec![rtn]);
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
    } else if self.current_tokens[0] == "pass".to_string() {
      println!("Stmt->pass");
      self.consume_token();
      return AST::new(
        "Stmt".to_string(),
        vec![AST::new("pass".to_string(), vec![])],
      );
    } else if self.keywords.contains(self.current_tokens[0].as_str()) {
      println!("Stmt->VarDecl");
      let vd = self.parse_var_decl();
      return AST::new("Stmt".to_string(), vec![vd]);
    } else {
      println!("Stmt->Expr");
      let e = self.parse_expr();
      return AST::new("Stmt".to_string(), vec![e]);
    }
  }

  fn parse_fn_call(&mut self) -> AST {
    println!("FnCall->Identifier(ExprList)");
    let fn_name = self.current_tokens[0].clone();
    self.consume_token(); // 跳过函数名
    self.consume_token(); // 跳过左括号
    let mut expr_list = vec![];
    expr_list.push(AST::new(fn_name, vec![]));
    while self.current_tokens[0] != ")".to_string() {
      let expr = self.parse_expr();
      expr_list.push(expr);
      if self.current_tokens[0] == "," {
        self.consume_token();
      }
    }
    self.consume_token();
    return AST::new("FnCall".to_string(), expr_list);
  }

  fn parse_return(&mut self) -> AST {
    println!("Return->Expr");
    self.consume_token();
    let ex = self.parse_expr();
    return AST::new("Return".to_string(), vec![ex]);
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
    } else {
      panic!(
        "parse_var_def error, expected = but got {}",
        self.current_tokens[0]
      );
    }
  }

  fn parse_assign(&mut self) -> AST {
    println!("Assign->Identifier = Expr");
    let id = self.parse_identifier();
    if self.current_tokens[0] == "=" {
      self.consume_token();
      let ex = self.parse_expr();
      return AST::new("Assign".to_string(), vec![id, ex]);
    } else {
      panic!(
        "parse_assign error, expected = but got {}",
        self.current_tokens[0]
      );
    }
  }

  fn parse_expr(&mut self) -> AST {
    let left = self.parse_term();
    let op = self.current_tokens[0].clone();
    if op == "+"
      || op == "-"
      || op == "=="
      || op == "!="
      || op == "<"
      || op == ">"
      || op == "<="
      || op == ">="
    {
      self.consume_token();
      let right = self.parse_expr();
      println!("Expr->Term {} Expr", op);
      return AST::new("Expr".to_string(), vec![left, AST::new(op, vec![]), right]);
    } else {
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
      return AST::new("Term".to_string(), vec![left, AST::new(op, vec![]), right]);
    } else {
      println!("Term->Factor");
      return AST::new("Term".to_string(), vec![left]);
    }
  }

  fn parse_factor(&mut self) -> AST {
    while self.current_tokens.len() < 2 {
      self.prefetch_token();
    }
    if self.current_tokens[0] == "(" {
      println!("Factor->(Expr)");
      self.consume_token();
      let ex = self.parse_expr();
      if self.current_tokens[0] == ")" {
        self.consume_token();
      } else {
        panic!(
          "parse_factor error, expected ) but got {}",
          self.current_tokens[0]
        );
      }
      return AST::new(
        "Factor".to_string(),
        vec![
          AST::new("(".to_string(), vec![]),
          ex,
          AST::new(")".to_string(), vec![]),
        ],
      );
    } else if self.current_tokens[1] == "(" {
      println!("Factor->FnCall");
      let fc = self.parse_fn_call();
      return AST::new("Factor".to_string(), vec![fc]);
    } else {
      println!("Factor->Basic");
      let bs = self.parse_basic();
      return AST::new("Factor".to_string(), vec![bs]);
    }
  }

  fn parse_basic(&mut self) -> AST {
    if self.current_tokens[0] != "".to_string() {
      println!("Basic->{}", self.current_tokens[0]);
      let token = self.current_tokens[0].clone();
      self.consume_token();
      return AST::new("".to_string() + token.as_str(), vec![]);
    } else {
      panic!(
        "parse_basic error, expected Basic, but got {}",
        self.current_tokens[0]
      );
    }
  }
}
