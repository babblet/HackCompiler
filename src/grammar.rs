pub enum GrammarType {
  LexicalElement,
  ProgramStructure,
  Statement,
  Expression,
}

pub enum LexicalElement {
  Keyword,
  Symbol,
  IntegerConstant,
  StringConstant,
  Identifier,
}

pub enum LexicalElementKeyword {
  Class,
  Method,
  Function,
  Constructor,
  Int,
  Boolean,
  Char,
  Void,
  Var,
  Static,
  Field,
  Let,
  Do,
  If,
  Else,
  While,
  Return,
  True,
  False,
  Null,
  This,
}

pub enum ProgramStructure {
  Class,
  ClassVarDec,
  Type,
  SubroutineDec,
  ParameterList,
  SubroutineBody,
  VarDec,
  ClassName,
  SubroutineName,
  VarName,
}

pub enum Statements {
  Statements,
  Statement,
  LetStatement,
  IfStatement,
  WhileStatement,
  DoStatement,
  ReturnStatement,
}

pub enum Expression {
  Expression,
  Term,
  SubroutineCall,
  ExpressionList,
  Op,
  UnaryOp,
  KeywordConstant,   
}