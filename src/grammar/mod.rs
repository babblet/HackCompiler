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
pub enum KeywordKind {
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
  Retrun,
  True,
  False,
  Null,
  This
}

pub struct Keyword {
  pub kind: KeywordKind,
  pub as_string: String
}

impl Keyword {
  pub fn new(kind: KeywordKind) -> Keyword {
    let string = match kind {
      KeywordKind::Class => "class".to_string(),
      KeywordKind::Method => "method".to_string(),
      KeywordKind::Function => "function".to_string(),
      KeywordKind::Constructor => "constructor".to_string(),
      KeywordKind::Inta => "int".to_string(),
      KeywordKind::Boolean => "boolean".to_string(),
      KeywordKind::Char => "char".to_string(),
      KeywordKind::Void => "void".to_string(),
      KeywordKind::Var => "var".to_string(),
      KeywordKind::Static => "static".to_string(),
      KeywordKind::Field => "field".to_string(),
      KeywordKind::Let => "let".to_string(),
      KeywordKind::Do => "do".to_string(),
      KeywordKind::If => "if".to_string(),
      KeywordKind::Else => "else".to_string(),
      KeywordKind::While => "while".to_string(),
      KeywordKind::Retrun => "return".to_string(),
      KeywordKind::True => "true".to_string(),
      KeywordKind::False => "false".to_string(),
      KeywordKind::Null => "null".to_string(),
      KeywordKind::This => "this".to_string()
    };

    Keyword {
      kind: kind,
      as_string: string,
    }
  }

  pub fn getKindArray() -> [Keyword; 21] {
    [
      Keyword::new(KeywordKind::Class),
      Keyword::new(KeywordKind::Method),
      Keyword::new(KeywordKind::Function),
      Keyword::new(KeywordKind::Constructor),
      Keyword::new(KeywordKind::Int),
      Keyword::new(KeywordKind::Boolean),
      Keyword::new(KeywordKind::Char),
      Keyword::new(KeywordKind::Void),
      Keyword::new(KeywordKind::Var),
      Keyword::new(KeywordKind::Static),
      Keyword::new(KeywordKind::Field),
      Keyword::new(KeywordKind::Let),
      Keyword::new(KeywordKind::Do),
      Keyword::new(KeywordKind::If),
      Keyword::new(KeywordKind::Else),
      Keyword::new(KeywordKind::While),
      Keyword::new(KeywordKind::Retrun),
      Keyword::new(KeywordKind::True),
      Keyword::new(KeywordKind::False),
      Keyword::new(KeywordKind::Null),
      Keyword::new(KeywordKind::This)
    ]
  }
}

pub static SYMBOLS: &'static [char] = &['{','}','(',')','[',']','.',',',';','+','-','*','/','&','|','<','>','=','~'];