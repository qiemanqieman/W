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
      | return Expr

VarDecl -> Type Identifier
VarDef -> Type Identifier = Expr
Assign -> Identifier = Expr

Expr -> Term + Expr
      | Term - Expr
      | Term

Term -> Factor * Term
      | Factor / Term
      | Factor

Factor -> ( Expr )
        | Basic (Integer Float StringLiteral et. al.)

