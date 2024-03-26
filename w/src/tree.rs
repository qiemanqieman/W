/// 多叉树

pub struct Tree {
  value: String,
  n_children: usize,
  children: Vec<Tree>,
}

impl Tree {
  pub fn new(value: String, children: Vec<Tree>) -> Self {
    let n_children = children.len();
    Self {
      value,
      n_children,
      children,
    }
  }
  pub fn new1(value: String) -> Self {
    Self {
      value,
      n_children: 0,
      children: vec![],
    }
  }

  pub fn print(&self, depth: usize) {
    print!("{}", "  ".repeat(depth));
    print!("{}\n", self.value);
    for child in &self.children {
      child.print(depth + 1);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_Tree() {
    let Tree = Tree::new(
      "pg".to_string(),
      vec![
        Tree::new(
          "FnList".to_string(),
          vec![
            Tree::new(
              "Type".to_string(),
              vec![Tree::new("int".to_string(), vec![])],
            ),
            Tree::new("Identifier".to_string(), vec![Tree::new("main".to_string(), vec![])]),
            Tree::new("Param".to_string(), vec![]),
            Tree::new("FnBody".to_string(), vec![]),
            Tree::new("FnList".to_string(), vec![]),
          ],
        ),
      ],
    );
    Tree.print(0);
  }
}