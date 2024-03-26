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

VarDecl -> Type Identifier
VarDef -> Type Identifier = Expr
Assign -> Identifier = Expr

Expr -> Expr + Term
      | Expr - Term
      | Term

Term -> Term * Factor
      | Term / Factor
      | Factor

Factor -> ( Expr )
        | Identifier
        | Literal

