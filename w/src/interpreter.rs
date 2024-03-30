/*
 * @Author: qiemanqieman 1324137924@qq.com
 * @Date: 2024-03-29 21:38:43
 * @LastEditors: qiemanqieman 1324137924@qq.com
 * @LastEditTime: 2024-03-30 20:42:46
 * @FilePath: /W/w/src/interpreter.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

use crate::ast::AST;
use std::{collections::VecDeque, mem::swap};

pub struct Interpreter {
  used_registers: Vec<String>,
  // all_registers: Vec<String>,
  operators: Vec<String>,
}

impl Interpreter {
  pub fn new() -> Interpreter {
    Interpreter {
      used_registers: Vec::new(),
      operators: vec![
        "+".to_string(),
        "-".to_string(),
        "*".to_string(),
        "/".to_string(),
        "^".to_string(),
      ],
    }
  }

  fn available_registers(&self) -> String {
    let registers = vec![
      "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "rcx", "rbx", "rsi",
      "rdi", // "rax", "rdx", 特殊用途，不要用于寄存器分配
    ];
    for reg in registers {
      if !(self.used_registers.contains(&reg.to_string())) {
        return reg.to_string();
      }
    }
    panic!("No available registers");
  }

  fn op_priority(&self, op: &str) -> i8 {
    match op {
      "+" | "-" => 1,
      "*" | "/" => 2,
      "^" => 3,
      _ => 0,
    }
  }

  fn get_op_code(&self, op: &str, operands: Vec<String>) -> &str {
    if operands.is_empty() {}
    match op {
      "+" => "addq",
      "-" => "subq",
      "*" => "imulq",
      "/" => "idivq",
      _ => "",
    }
  }

  // 汇编代码生成器
  // Pg
  // ├─ Fn
  // |  ├─ int
  // |  ├─ main
  // |  ├─ Param
  // |  |  └─ ParamList
  // |  |     └─ ε
  // |  └─ FnBody
  // |     └─ StmtList
  // |        ├─ Stmt
  // |        |  └─ Return
  // |        |     └─ Expr
  // |        |        └─ Term
  // |        |           └─ Factor
  // |        |              └─ 2
  // |        └─ StmtList
  // |           └─ ε
  // └─ FnList
  //    └─ ε
  pub fn generate_asm(&mut self, ast: &mut AST) -> String {
    let mut asm = String::new();
    self.generate_asm_helper(ast, &mut asm);
    asm
  }

  // 辅助函数，递归生成汇编代码
  fn generate_asm_helper(&mut self, ast: &mut AST, asm: &mut String) {
    match ast.value.as_str() {
      "Pg" => self.generate_asm_pg(ast, asm),
      "Fn" => self.generate_asm_fn(ast, asm),
      "FnBody" => self.generate_asm_fn_body(ast, asm),
      "StmtList" => self.generate_asm_stmt_list(ast, asm),
      "Stmt" => self.generate_asm_stmt(ast, asm),
      "Return" => self.generate_asm_ret(ast, asm),
      "Expr" => self.generate_asm_expr(ast, asm),
      "ε" => return,
      _ => return,
    }
  }

  fn generate_asm_pg(&mut self, ast: &mut AST, asm: &mut String) {
    asm.push_str("	.text\n");
    asm.push_str("	.globl	main\n");
    for mut child in &mut ast.children {
      self.generate_asm_helper(&mut child, asm);
    }
  }

  fn generate_asm_fn(&mut self, ast: &mut AST, asm: &mut String) {
    asm.push_str(&format!("{}:\n", ast.children[1].value));
    self.generate_asm_helper(&mut ast.children[3], asm);
  }

  fn generate_asm_fn_body(&mut self, ast: &mut AST, asm: &mut String) {
    self.generate_asm_helper(&mut ast.children[0], asm);
  }

  fn generate_asm_stmt_list(&mut self, ast: &mut AST, asm: &mut String) {
    for mut child in &mut ast.children {
      self.generate_asm_helper(&mut child, asm);
    }
  }

  fn generate_asm_stmt(&mut self, ast: &mut AST, asm: &mut String) {
    self.generate_asm_helper(&mut ast.children[0], asm);
  }

  fn generate_asm_ret(&mut self, ast: &mut AST, asm: &mut String) {
    self.generate_asm_helper(&mut ast.children[0], asm);
    asm.push_str(&format!("  movq %{}, %rax\n", ast.children[0].register));
    asm.push_str("	ret\n");
  }

  fn generate_asm_expr(&mut self, ast: &mut AST, asm: &mut String) {
    let expression = ast.get_expression();
    println!("{:?}", expression);
    let mut operators = VecDeque::<String>::new();
    let mut registers = VecDeque::<String>::new();
    for token in expression.iter() {
      if self.operators.contains(token) {
        if operators.is_empty() {
          operators.push_back(token.clone());
        } else {
          while !operators.is_empty()
            && self.op_priority(operators.back().unwrap()) >= self.op_priority(token)
          {
            self.step(asm, &mut operators, &mut registers);
          }
          operators.push_back(token.clone());
        }
      } else {
        let reg = self.available_registers();
        registers.push_back(reg.clone());
        self.used_registers.push(reg.clone());
        asm.push_str(&format!("	movq ${}, %{}\n", token, reg));
      }
    }
    while !operators.is_empty() {
      self.step(asm, &mut operators, &mut registers);
    }
    ast.register = registers.pop_back().unwrap();
  }

  fn step(
    &mut self,
    asm: &mut String,
    operators: &mut VecDeque<String>,
    registers: &mut VecDeque<String>,
  ) {
    let operator = operators.pop_back().unwrap();
    let mut reg2 = registers.pop_back().unwrap();
    let mut reg1 = registers.pop_back().unwrap();
    let op_code = self.get_op_code(&operator, vec![reg1.clone(), reg2.clone()]);
    if op_code == "idivq" {
      asm.push_str(&format!(
        "  movq %{0}, %rax\n  xor %rdx, %rdx\n	cqto\n  idivq %{1}\n  movq %rax, %{1}\n",
        reg1, reg2
      ));
    } else {
      if op_code == "subq" {
        swap(&mut reg1, &mut reg2);
      }
      asm.push_str(&format!("	{} %{}, %{}\n", op_code, reg1, reg2));
    }
    self.used_registers.retain(|x| x != &reg1);
    registers.push_back(reg2.clone());
  }
}
