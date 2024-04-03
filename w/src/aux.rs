/*
 * @Author: qiemanqieman 1324137924@qq.com
 * @Date: 2024-03-31 19:57:55
 * @LastEditors: qiemanqieman 1324137924@qq.com
 * @LastEditTime: 2024-04-03 23:09:16
 * @FilePath: /W/w/src/aux.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
pub fn is_identifier(token: &str) -> bool {
  token
    .chars()
    .all(|c| (c <= 'z' && c >= 'a') || c == '_' || (c <= 'Z' && c >= 'A'))
}
