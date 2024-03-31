/*
 * @Author: qiemanqieman 1324137924@qq.com
 * @Date: 2024-03-28 00:48:23
 * @LastEditors: qiemanqieman 1324137924@qq.com
 * @LastEditTime: 2024-03-31 21:17:34
 * @FilePath: /W/w/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
pub mod ast;
pub mod aux;
pub mod interpreter;
pub mod lexer;
pub mod main_run;
pub mod parser;
use main_run::*;

fn main() {
  let (input, output_filename) = get_input();
  let ast = src2ast(input);

  ast2exe(ast, output_filename);
}
