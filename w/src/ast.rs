
/// 抽象语法树
pub struct AST {
  value: String,
  children: Vec<AST>,
}

impl AST {
  pub fn new(value: String, children: Vec<AST>) -> Self {
    Self {
      value,
      children,
    }
  }

  /// 代码生成器
  /// Pg
  /// ├─ Fn
  /// |  ├─ Type->int
  /// |  ├─ Identifier->main
  /// |  ├─ Param
  /// |  |  └─ ParamList->ε
  /// |  └─ FnBody
  /// |  |  └─ StmtList
  /// |  |  |  ├─ Stmt->return
  /// |  |  |  |  └─ Expr
  /// |  |  |  |  |  └─ Term
  /// |  |  |  |  |  |  └─ Factor->Basic 2
  /// |  |  |  └─ StmtList->ε
  /// └─ FnList->ε
  pub fn gen_asm(&self) -> String {
    let mut asm = String::new();
    asm  = self.generate_asm_helper();
    asm
  }

  // 辅助函数，递归生成汇编代码
  fn generate_asm_helper(&self) -> String {
    // 在这里根据节点的类型生成相应的汇编代码
    match self.value.as_str() {
      "Pg" => {
        // 在这里生成程序入口的汇编代码，这里只是一个示例
        self.generate_asm_pg()
      }
      "Fn" => {
        // 在这里生成函数的汇编代码，这里只是一个示例
        self.generate_asm_fn()
      }
      "FnBody" => {
        // 在这里生成函数体的汇编代码，这里只是一个示例
        self.generate_asm_fn_body()
      }
      "StmtList" => {
        // 在这里生成语句列表的汇编代码，这里只是一个示例
        self.generate_asm_stmt_list()
      }
      "StmtList->ε" => {
        // 空语句列表，不需要生成汇编代码
        String::new()
      }
      "Stmt->return" => {
        // 在这里生成返回语句的汇编代码，这里只是一个示例
        self.generate_asm_ret_stmt()
      }
      "Expr" => {
        // 在这里生成返回语句的汇编代码，这里只是一个示例
        self.generate_asm_expr()
      }
      "Term" => {
        // 在这里生成返回语句的汇编代码，这里只是一个示例
        self.generate_asm_term()
      }
      "Factor" => {
        // 在这里生成返回语句的汇编代码，这里只是一个示例
        self.generate_asm_factor()
      }
      "Factor->Basic 2" => {
        // 在这里生成返回语句的汇编代码，这里只是一个示例
        self.generate_asm_factor_basic()
      }
      // "Stmt->return" => {
      //   // 在这里生成返回语句的汇编代码，这里只是一个示例
      //   self.generate_asm_fn()
      // },
      _ => {
        // 对于其他节点类型，可以根据需要生成相应的汇编代码
        // 这里省略了其他节点类型的代码生成逻辑
        String::new()
      }
    }
  }

  fn generate_asm_pg(&self) -> String {
    let mut asm = String::new();
    asm.push_str("	.text\n");
    asm.push_str("	.globl	main\n");
    for child in &self.children {
      asm.push_str(&child.generate_asm_helper());
    }
    asm
  }

  fn generate_asm_fn(&self) -> String {
    let mut asm = String::new();
    asm.push_str(&format!("{}:\n", self.children[1].value.split("->").collect::<Vec<&str>>()[1]));
    asm.push_str(self.children[3].generate_asm_helper().as_str());
    asm
  }

  fn generate_asm_fn_body(&self) -> String {
    let mut asm = String::new();
    asm.push_str(self.children[0].generate_asm_helper().as_str());
    asm
  }

  fn generate_asm_stmt_list(&self) -> String {
    let mut asm = String::new();
    for child in &self.children {
      asm.push_str(&child.generate_asm_helper());
    }
    asm
  }
  fn generate_asm_ret_stmt(&self) -> String {
    let mut asm = String::new();
    for child in &self.children {
      asm.push_str(&child.generate_asm_helper());
    }
    asm.push_str("	ret\n");
    asm
  }

  fn generate_asm_expr(&self) -> String {
    let mut asm = String::new();
    asm.push_str(self.children[0].generate_asm_helper().as_str());
    asm
  }

  fn generate_asm_term(&self) -> String {
    let mut asm = String::new();
    asm.push_str(self.children[0].generate_asm_helper().as_str());
    asm
  }

  fn generate_asm_factor(&self) -> String {
    let mut asm = String::new();
    asm.push_str(&format!("	 movl	${}, %eax\n", self.children[0].value));
    asm
  }

  fn generate_asm_factor_basic(&self) -> String {
    let mut asm = String::new();
    asm.push_str(&format!("  movl	${}, %eax\n", 2));
    asm
  }

  pub fn print(&self, depth: usize, is_last: bool) {
    if depth == 0 {
      println!("{}", self.value);
    } 
    else {
      print!("{}", "|  ".repeat(depth-1));
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
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tree() {
    let tree = AST::new(
      "pg".to_string(),
      vec![
        AST::new(
          "FnList".to_string(),
          vec![
            AST::new(
              "Type".to_string(),
              vec![AST::new("int".to_string(), vec![])],
            ),
            AST::new("Identifier".to_string(), vec![AST::new("main".to_string(), vec![])]),
            AST::new("Param".to_string(), vec![]),
            AST::new("FnBody".to_string(), vec![]),
            AST::new("FnList".to_string(), vec![]),
          ],
        ),
      ],
    );
    tree.print(0, true);
  }
}