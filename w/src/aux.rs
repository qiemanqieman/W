/*
 * @Author: qiemanqieman 1324137924@qq.com
 * @Date: 2024-03-28 21:23:34
 * @LastEditors: qiemanqieman 1324137924@qq.com
 * @LastEditTime: 2024-03-30 19:32:47
 * @FilePath: /W/w/src/aux.rs
 * @Description:
 *
 * Copyright (c) 2024 by ${git_name_email}, All Rights Reserved.
 */
use crate::ast::AST;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::env;
use std::fs;
use std::process::Command;

pub fn get_input() -> (String, String) {
  let args: Vec<String> = env::args().collect();
  if args.len() == 2 {
    // args[1]指定了w语言编译器测试用的源文件（args[1] is of form `name`.w）
    let input_filename = &args[1];
    let mut output_filename = input_filename.clone();
    let dot_index = output_filename.rfind('.'); // Find the last occurrence of '.'
    if let Some(index) = dot_index {
      output_filename.truncate(index); // Remove the extension
    }
    output_filename.push_str(".s"); // Add the new extension
    return (
      fs::read_to_string(input_filename).expect("Failed to read file"),
      output_filename,
    );
  } else if args.len() == 1 {
    // 没有指定源文件，使用tmp目录下的return_2.w作为源文件
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let current_dir_str = current_dir
      .to_str()
      .expect("Failed to convert current directory to string")
      .to_owned(); // Convert to owned String
    print!("Current directory: {}\n", current_dir_str);
    let filename = current_dir_str.clone() + "/../tmp/return_2.w";
    let output_filename = current_dir_str.clone() + "/../tmp/return_2.s";
    let input = fs::read_to_string(filename).expect("Failed to read file");
    return (input, output_filename);
  } else {
    eprintln!("Usage: {} <filename>", args[0]);
    std::process::exit(1);
  }
}

pub fn src2ast(input: String) -> AST {
  let lexer = Lexer::new(&input);
  let mut parser = Parser::new(lexer);
  let ast = parser.parse();
  println!("\n\n\n");
  let mut path = vec![];
  ast.print(0, &mut path);
  ast
}

pub fn ast2exe(mut ast: AST, output_filename: String) {
  // 解析抽象语法树，生成汇编代码
  let mut interpreter = Interpreter::new();
  let asm = interpreter.generate_asm(&mut ast);
  // let asm = ast.gen_asm();

  fs::write(output_filename.clone(), asm).expect("Failed to write to file");
  println!("Assembly code written to file: {}", output_filename);

  // gcc汇编，生成可执行文件
  let filename = output_filename;
  let output_file = filename.clone().replace(".s", "");
  let mut cmd = Command::new("gcc");
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    cmd.current_dir("/home/wm/0WM/W/tmp");
  }
  cmd.arg(filename).arg("-o").arg(output_file);
  let output = cmd.output().expect("Failed to execute command");
  if output.status.success() {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("Command (GCC compilation of assembly code) executed successfully!");
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
  } else {
    let stderr = String::from_utf8_lossy(&output.stderr);
    eprintln!("Command (GCC compilation of assembly code) failed to execute!");
    eprintln!("stderr: {}", stderr);
  }
}
