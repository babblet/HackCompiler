#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LexicalElement {
  Keyword,
  Symbol,
  IntegerConstant,
  StringConstant,
  Identifier,

}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Statements {
  Statements,
  Statement,
  LetStatement,
  IfStatement,
  WhileStatement,
  DoStatement,
  ReturnStatement,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Expression {
  Expression,
  Term,
  SubroutineCall,
  ExpressionList,
  Op,
  UnaryOp,
  KeywordConstant,
}

pub mod keyword {
  pub const CLASS: &'static str = "class";
  pub const METHOD: &'static str = "method";
  pub const FUNCTION: &'static str = "function";
  pub const CONSTRUCTOR: &'static str = "constructor";
  pub const INT: &'static str = "int";
  pub const BOOLEAN: &'static str = "boolean";
  pub const CHAR: &'static str = "char";
  pub const VOID: &'static str = "void";
  pub const VAR: &'static str = "var";
  pub const STATIC: &'static str = "static";
  pub const FIELD: &'static str = "field";
  pub const LET: &'static str = "let";
  pub const DO: &'static str = "do";
  pub const IF: &'static str = "if";
  pub const ELSE: &'static str = "else";
  pub const WHILE: &'static str = "while";
  pub const RETURN: &'static str = "return";
  pub const TRUE: &'static str = "true";
  pub const FALSE: &'static str = "false";
  pub const NULL: &'static str = "null";
  pub const THIS: &'static str = "this";
}

pub static KEYWORDKEYS: &'static [&'static str] = &[
  keyword::CLASS,
  keyword::METHOD,
  keyword::FUNCTION,
  keyword::CONSTRUCTOR,
  keyword::INT,
  keyword::BOOLEAN,
  keyword::CHAR,
  keyword::VOID,
  keyword::VAR,
  keyword::STATIC,
  keyword::FIELD,
  keyword::LET,
  keyword::DO,
  keyword::IF,
  keyword::ELSE,
  keyword::WHILE,
  keyword::RETURN,
  keyword::TRUE,
  keyword::FALSE,
  keyword::NULL,
  keyword::THIS
];

pub static SYMBOLS: &'static [char] = &['{','}','(',')','[',']','.',',',';','+','-','*','/','&','|','<','>','=','~'];