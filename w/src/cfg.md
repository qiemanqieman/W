<!--
 * @Author: qiemanqieman 1324137924@qq.com
 * @Date: 2024-03-23 15:49:34
 * @LastEditors: qiemanqieman 1324137924@qq.com
 * @LastEditTime: 2024-04-04 00:18:23
 * @FilePath: /W/w/src/cfg.md
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->
# Context Free Grammar for W language

Program -> Fn FnList

Fn -> Type FnName Param FnBody

FnList -> Fn FnList
        | ε
        
Param -> ( ParamList )
ParamList -> Type Identifier ParamListTail
           | ε
ParamListTail -> , Type Identifier ParamListTail
               | ε

FnBody -> { StmtList }

StmtList -> Stmt StmtList
          | ε

Stmt -> VarDecl
      | VarDef
      | Assign
      | pass
      | Return
      | Expr
      | BranchStmt
      | LoopStmt 

BranchStmt -> if Expr { StmtList } else { StmtList }

FnCall -> Identifier ( ExprList )

VarDecl -> Type Identifier
VarDef -> Type Identifier = Expr
Assign -> Identifier = Expr
Return -> Expr

Expr -> Term + Expr
      | Term - Expr
      | Term > Expr
      | Term

Term -> Factor * Term
      | Factor / Term
      | Factor

Factor -> ( Expr )
        | Basic
        | FnCall

Basic -> (Integer Float StringLiteral et. al.)

