pub mod lexer;
pub mod parser;
pub mod tree;
use lexer::Lexer;
use std::fs;
use std::env;

fn main() {
  let current_dir = env::current_dir()
    .expect("Failed to get current directory");
  let current_dir_str = current_dir.to_str()
    .expect("Failed to convert current directory to string")
    .to_owned(); // Convert to owned String
  print!("Current directory: {}\n", current_dir_str);
  let filename = current_dir_str + "/../tmp/return_2.w";
  let input = fs::read_to_string(filename).expect("Failed to read file");

  let lexer = Lexer::new(&input);
  let mut parser = parser::Parser::new(lexer);
  let ast = parser.parse();
  println!("");
  println!("");
  println!("");
  ast.print(0)
}