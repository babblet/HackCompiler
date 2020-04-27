use crate::grammar::Keyword;
use crate::grammar::LexicalElement;
use crate::grammar::IdentifierKind;

pub struct Token {
  pub element: LexicalElement,
  pub data: String,
  pub keyword_data: Option<Keyword>,
  pub identifier_kind: Option<IdentifierKind>
}

