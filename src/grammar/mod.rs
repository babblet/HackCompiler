#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LexicalElementKind {
  Keyword,
  Symbol,
  IntegerConstant,
  StringConstant,
  Identifier
}

pub struct LexicalElement {
  pub kind: LexicalElementKind,
  pub as_string: String,
}

impl LexicalElement {
  pub fn new(kind: LexicalElementKind) -> LexicalElement {
    let string = match kind {
      LexicalElementKind::Keyword => "keyword".to_string(),
      LexicalElementKind::Symbol => "symbol".to_string(),
      LexicalElementKind::IntegerConstant => "integerConstant".to_string(),
      LexicalElementKind::StringConstant => "stringConstant".to_string(),
      LexicalElementKind::Identifier => "identifier".to_string()
    };

    LexicalElement {
      kind: kind,
      as_string: string,
    }
  }
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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum KeywordKind {
  CLASS,
  METHOD,
  FUNCTION,
  CONSTRUCTOR,
  INT,
  BOOLEAN,
  CHAR,
  VOID,
  VAR,
  STATIC,
  FIELD,
  LET,
  DO,
  IF,
  ELSE,
  WHILE,
  RETURN,
  TRUE,
  FALSE,
  NULL,
  THIS
}

pub struct Keyword {
  pub kind: KeywordKind,
  pub as_string: String
}

impl Keyword {
  pub fn new(kind: KeywordKind) -> Keyword {
    let string = match kind {
      KeywordKind::CLASS => "class".to_string(),
      KeywordKind::METHOD => "method".to_string(),
      KeywordKind::FUNCTION => "function".to_string(),
      KeywordKind::CONSTRUCTOR => "constructor".to_string(),
      KeywordKind::INT => "int".to_string(),
      KeywordKind::BOOLEAN => "boolean".to_string(),
      KeywordKind::CHAR => "char".to_string(),
      KeywordKind::VOID => "void".to_string(),
      KeywordKind::VAR => "var".to_string(),
      KeywordKind::STATIC => "static".to_string(),
      KeywordKind::FIELD => "field".to_string(),
      KeywordKind::LET => "let".to_string(),
      KeywordKind::DO => "do".to_string(),
      KeywordKind::IF => "if".to_string(),
      KeywordKind::ELSE => "else".to_string(),
      KeywordKind::WHILE => "while".to_string(),
      KeywordKind::RETURN => "return".to_string(),
      KeywordKind::TRUE => "true".to_string(),
      KeywordKind::FALSE => "false".to_string(),
      KeywordKind::NULL => "null".to_string(),
      KeywordKind::THIS => "this".to_string()
    };

    Keyword {
      kind: kind,
      as_string: string,
    }
  }

  pub fn getKindArray() -> [Keyword; 21] {
    [
      Keyword::new(KeywordKind::CLASS),
      Keyword::new(KeywordKind::METHOD),
      Keyword::new(KeywordKind::FUNCTION),
      Keyword::new(KeywordKind::CONSTRUCTOR),
      Keyword::new(KeywordKind::INT),
      Keyword::new(KeywordKind::BOOLEAN),
      Keyword::new(KeywordKind::CHAR),
      Keyword::new(KeywordKind::VOID),
      Keyword::new(KeywordKind::VAR),
      Keyword::new(KeywordKind::STATIC),
      Keyword::new(KeywordKind::FIELD),
      Keyword::new(KeywordKind::LET),
      Keyword::new(KeywordKind::DO),
      Keyword::new(KeywordKind::IF),
      Keyword::new(KeywordKind::ELSE),
      Keyword::new(KeywordKind::WHILE),
      Keyword::new(KeywordKind::RETURN),
      Keyword::new(KeywordKind::TRUE),
      Keyword::new(KeywordKind::FALSE),
      Keyword::new(KeywordKind::NULL),
      Keyword::new(KeywordKind::THIS)
    ]
  }
}

pub static SYMBOLS: &'static [char] = &['{','}','(',')','[',']','.',',',';','+','-','*','/','&','|','<','>','=','~'];