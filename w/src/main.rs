pub mod lexer;
pub mod parser;
pub mod ast;
use lexer::Lexer;
use std::fs;
use std::env;
use std::process::Command;

fn main() {
  let current_dir = env::current_dir()
    .expect("Failed to get current directory");
  let current_dir_str = current_dir.to_str()
    .expect("Failed to convert current directory to string")
    .to_owned(); // Convert to owned String
  print!("Current directory: {}\n", current_dir_str);
  let filename = current_dir_str.clone() + "/../tmp/return_2.w";
  let output_filename = current_dir_str.clone() + "/build/return_2.s";
  let input = fs::read_to_string(filename).expect("Failed to read file");

  let lexer = Lexer::new(&input);
  let mut parser = parser::Parser::new(lexer);
  let ast = parser.parse();
  println!("\n\n\n");
  ast.print(0, true);

  // 解析抽象语法树，生成汇编代码
  let asm = ast.gen_asm();
  fs::write(output_filename.clone(), asm).expect("Failed to write to file");
  println!("Assembly code written to file: {}", output_filename);

  // gcc汇编，生成可执行文件
  let file_path = "return_2.s";
  let output_file = "return_2";
  let mut cmd = Command::new("gcc");
  cmd.current_dir("/home/wm/0WM/W/w/build");
  cmd.arg(file_path).arg("-o").arg(output_file);
  let output = cmd.output()
    .expect("Failed to execute command");
  if output.status.success() {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("Command executed successfully!");
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
  } else {
    let stderr = String::from_utf8_lossy(&output.stderr);
    eprintln!("Command failed to execute!");
    eprintln!("stderr: {}", stderr);
  }
}