/*
 * @Author: qiemanqieman 1324137924@qq.com
 * @Date: 2024-03-25 23:53:18
 * @LastEditors: qiemanqieman 1324137924@qq.com
 * @LastEditTime: 2024-03-31 13:12:38
 * @FilePath: /W/w/src/ast.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

/// 抽象语法树
pub struct AST {
  pub value: String,
  pub children: Vec<AST>,
  pub register: String,
}

impl AST {
  pub fn new(value: String, children: Vec<AST>) -> Self {
    Self {
      value,
      children,
      register: String::new(),
    }
  }

  /// Returns a vector of expressions from the AST.
  ///
  /// # Examples
  ///
  /// ```
  /// use crate::ast::AST;
  ///
  /// let tree = AST::new(
  ///     "Expr".to_string(),
  ///     vec![
  ///         AST::new("Value".to_string(), vec![]),
  ///         AST::new("Operator".to_string(), vec![]),
  ///         AST::new("Value".to_string(), vec![]),
  ///     ],
  /// );
  ///
  /// let expressions = tree.get_expression();
  /// assert_eq!(expressions, vec!["Value", "Operator", "Value"]);
  /// ```
  pub fn get_expression(&self) -> Vec<String> {
    if self.children.is_empty() {
      return vec![self.value.clone()];
    }
    let mut expression = vec![];
    for child in &self.children {
      expression.extend(child.get_expression());
    }
    expression
  }

  pub fn print(&self, depth: usize, path: &mut Vec<bool>) {
    for d in 0..depth {
      if d == depth - 1 {
        if path[d] {
          print!("├─ ");
        } else {
          print!("└─ ");
        }
      } else {
        if path[d] {
          print!("|  ");
        } else {
          print!("   ");
        }
      }
    }
    println!("{}", self.value);

    if !self.children.is_empty() {
      for i in 0..self.children.len() {
        path.push(i != self.children.len() - 1);
        self.children[i].print(depth + 1, path);
        path.pop();
      }
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
      vec![AST::new(
        "FnList".to_string(),
        vec![
          AST::new(
            "Type".to_string(),
            vec![AST::new("int".to_string(), vec![])],
          ),
          AST::new(
            "Identifier".to_string(),
            vec![AST::new("main".to_string(), vec![])],
          ),
          AST::new("Param".to_string(), vec![]),
          AST::new("FnBody".to_string(), vec![]),
          AST::new("FnList".to_string(), vec![]),
        ],
      )],
    );
    let mut path = vec![];
    tree.print(0, &mut path);
  }

  #[test]
  fn test_get_expression() {
    let tree = AST::new(
      "Expr".to_string(),
      vec![
        AST::new("1".to_string(), vec![]),
        AST::new("+".to_string(), vec![]),
        AST::new("2".to_string(), vec![]),
      ],
    );
    let expression = tree.get_expression();
    assert_eq!(expression, vec!["1", "+", "2"]);
  }
}
