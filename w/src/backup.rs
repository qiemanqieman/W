/*
* This file is used to save the code which's currently unused in the project, but may be used in the future.
*/

use std::collections::{HashMap, VecDeque};

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
      "Param".to_string(),         // 参数
      "ParamList".to_string(),     // 参数列表
      "ParamListTail".to_string(), // 参数列表
      "FnBody".to_string(),        // 函数体
      "StmtList".to_string(),      // 语句列表
      "Stmt".to_string(),          // 语句
      "VarDecl".to_string(),       // 变量声明
      "VarDef".to_string(),        // 变量定义
      "Assign".to_string(),        // 赋值
      "Expr".to_string(),          // 表达式
    ];
    let start_symbol = "Pg".to_string();
    let mut rules = HashMap::new();
    rules.insert(
      "Pg".to_string(),
      vec![vec!["Fn".to_string(), "FnList".to_string()]],
    );
    rules.insert(
      "Fn".to_string(),
      vec![vec![
        "Type".to_string(),
        "FnName".to_string(),
        "Param".to_string(),
        "FnBody".to_string(),
      ]],
    );
    rules.insert(
      "FnList".to_string(),
      vec![
        vec!["Fn".to_string(), "FnList".to_string()],
        vec!["".to_string()],
      ],
    );
    rules.insert("Type".to_string(), vec![vec!["Keyword".to_string()]]);
    rules.insert("FnName".to_string(), vec![vec!["Identifier".to_string()]]);
    rules.insert(
      "Param".to_string(),
      vec![vec![
        "(".to_string(),
        "ParamList".to_string(),
        ")".to_string(),
      ]],
    );
    rules.insert(
      "ParamList".to_string(),
      vec![
        vec![
          "Type".to_string(),
          "Identifier".to_string(),
          "ParamListTail".to_string(),
        ],
        vec!["".to_string()],
      ],
    );
    rules.insert(
      "ParamListTail".to_string(),
      vec![
        vec![
          ",".to_string(),
          "Type".to_string(),
          "Identifier".to_string(),
          "ParamListTail".to_string(),
        ],
        vec!["".to_string()],
      ],
    );
    rules.insert(
      "FnBody".to_string(),
      vec![vec![
        "{".to_string(),
        "StmtList".to_string(),
        "}".to_string(),
      ]],
    );
    rules.insert(
      "StmtList".to_string(),
      vec![
        vec!["Stmt".to_string(), "StmtList".to_string()],
        vec!["".to_string()],
      ],
    );
    rules.insert(
      "Stmt".to_string(),
      vec![
        vec!["VarDecl".to_string()],
        vec!["VarDef".to_string()],
        vec!["Assign".to_string()],
        vec!["pass".to_string()],
      ],
    );
    rules.insert(
      "VarDecl".to_string(),
      vec![vec!["Type".to_string(), "Identifier".to_string()]],
    );
    rules.insert(
      "VarDef".to_string(),
      vec![vec![
        "Type".to_string(),
        "Identifier".to_string(),
        "=".to_string(),
        "Expr".to_string(),
      ]],
    );
    rules.insert(
      "Assign".to_string(),
      vec![vec![
        "Identifier".to_string(),
        "=".to_string(),
        "Expr".to_string(),
      ]],
    );
    rules.insert(
      "Expr".to_string(),
      vec![
        vec!["Identifier".to_string()],
        vec!["Integer".to_string()],
        vec!["Float".to_string()],
        vec!["StringLiteral".to_string()],
        vec!["(".to_string(), "Expr".to_string(), ")".to_string()],
        vec!["Expr".to_string(), "+".to_string(), "Expr".to_string()],
        vec!["Expr".to_string(), "-".to_string(), "Expr".to_string()],
        vec!["Expr".to_string(), "*".to_string(), "Expr".to_string()],
        vec!["Expr".to_string(), "/".to_string(), "Expr".to_string()],
      ],
    );

    Grammar {
      terminators,
      non_terminators,
      start_symbol,
      rules,
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

#[derive(Debug, PartialEq)]
enum Token {
  Identifier(String),
  Integer(String),
  Float(String),
  StringLiteral(String),
  Operator(String),
  Keyword(String),
  Delimiter(String),
}

pub fn print(&self, depth: usize, is_last: bool) {
  if depth == 0 {
    println!("{}", self.value);
  } else {
    print!("{}", "|  ".repeat(depth - 1));
    if is_last {
      print!("└─ ");
    } else {
      print!("├─ ");
    }
    println!("{}", self.value);
  }

  let num_children = self.children.len();
  for (i, child) in self.children.iter().enumerate() {
    let is_last_child = i == num_children - 1;
    child.print(depth + 1, is_last_child);
  }
}

// fn generate_asm_expr(&mut self, ast: &mut AST) -> String {
//   let mut asm = String::new();
//   if ast.children.len() == 3 {
//     asm.push_str(self.generate_asm_helper(&mut ast.children[0]).as_str());
//     asm.push_str(self.generate_asm_helper(&mut ast.children[2]).as_str());
//     let mut op = "addq";
//     if ast.children[1].value.as_str() == "-" {
//       op = "subq";
//     }
//     asm.push_str(&format!(
//       "	{} %{}, %{}\n",
//       op, ast.children[0].register, ast.children[2].register
//     ));
//     ast.register = self.available_registers();
//     self.used_registers.push(ast.register.clone());
//     asm.push_str(&format!(
//       "	movq %{}, %{}\n",
//       ast.children[2].register, ast.register
//     ));
//   } else {
//     asm.push_str(self.generate_asm_helper(&mut ast.children[0]).as_str());
//     ast.register = ast.children[0].register.clone();
//   }
//   asm
// }

// fn generate_asm_term(&mut self, ast: &mut AST) -> String {
//   let mut asm = String::new();
//   if ast.children.len() == 3 {
//     asm.push_str(self.generate_asm_helper(&mut ast.children[0]).as_str());
//     asm.push_str(self.generate_asm_helper(&mut ast.children[2]).as_str());
//     let mut op = "imulq";
//     if ast.children[1].value.as_str() == "/" {
//       op = "idivq";
//     }
//     asm.push_str(&format!(
//       "	{} %{}, %{}\n",
//       op, ast.children[0].register, ast.children[2].register
//     ));
//     ast.register = self.available_registers();
//     self.used_registers.push(ast.register.clone());
//     asm.push_str(&format!(
//       "	movq %{}, %{}\n",
//       ast.children[2].register, ast.register
//     ));
//   } else {
//     asm.push_str(self.generate_asm_helper(&mut ast.children[0]).as_str());
//     ast.register = ast.children[0].register.clone();
//   }
//   asm
// }

// fn generate_asm_factor(&mut self, ast: &mut AST) -> String {
//   let mut asm = String::new();
//   if ast.children[0].value != "Expr" {
//     ast.register = self.available_registers();
//     self.used_registers.push(ast.register.clone());
//     asm.push_str(&format!(
//       "	movq ${}, %{}\n",
//       ast.children[0].value, ast.register
//     ));
//   } else {
//     asm.push_str(self.generate_asm_helper(&mut ast.children[0]).as_str());
//     ast.register = ast.children[0].register.clone();
//   }
//   asm
// }
