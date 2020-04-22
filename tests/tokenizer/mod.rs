#[cfg(test)]
mod tokenizer {
  use std::ffi::OsString;
  use hack_compiler::classes::tokenizer::*;
  use hack_compiler::grammar::LexicalElement;
  use hack_compiler::grammar::LexicalElementKind;
  use hack_compiler::grammar::Keyword;
  use hack_compiler::grammar::KeywordKind;

  #[test]
  fn test_create_token() {
    let token = create_token(LexicalElementKind::Keyword, &"class".to_string(), Some(KeywordKind::Class));
    assert_eq!(token.element.kind, LexicalElementKind::Keyword);
    assert_eq!(token.element.as_string, "keyword".to_string());
    assert_eq!(token.data, "class".to_string());

    let key_was_created = match token.keyword_key {
      Some(k) => {
        if k.kind == KeywordKind::Class && k.as_string == "class".to_string() {
          true 
        } else {
          false
        }
      },
      _ => false
    };
    assert!(key_was_created);
  }

  #[test]
  fn test_find_constant() {
    assert_eq!(find_constant(&"}".to_string()), None);
    assert_eq!(find_constant(&"{".to_string()), None);
    assert_eq!(find_constant(&"]".to_string()), None);
    assert_eq!(find_constant(&"20".to_string()), Some(LexicalElementKind::IntegerConstant));
    assert_eq!(find_constant(&"2000000000".to_string()), None);
    assert_eq!(find_constant(&"-49".to_string()), None);
    assert_eq!(find_constant(&"\"Testing String\"".to_string()), Some(LexicalElementKind::StringConstant));
  }

  #[test]
  fn test_is_symbol() {
    assert_eq!(is_symbol('['), true);
    assert_eq!(is_symbol('D'), false);
  }

  #[test]
  fn test_is_integer_constant() {
    assert_eq!(is_integer_constant(&"100".to_string()), true);
    assert_eq!(is_integer_constant(&"0".to_string()), true);
    assert_eq!(is_integer_constant(&"-20".to_string()), false);
    assert_eq!(is_integer_constant(&"2000000000000".to_string()), false);
    assert_eq!(is_integer_constant(&"test".to_string()), false);
  }

  #[test]
  fn test_is_string_constant() {
    assert_eq!(is_string_constant(&"\"\"".to_string()), true);
    assert_eq!(is_string_constant(&"\"asd\"".to_string()), true);
    assert_eq!(is_string_constant(&"asd".to_string()), false);
  }

  #[test]
  fn test_find_keyword() {
    assert_eq!(find_keyword(&"class".to_string()), Some(KeywordKind::Class));
    assert_eq!(find_keyword(&"classes".to_string()), None);
  }

  #[test]
  fn test_is_whitespace() {
    assert_eq!(is_whitespace(' '), true);
    assert_eq!(is_whitespace('\t'), true);
    assert_eq!(is_whitespace(';'), false);
  }
}