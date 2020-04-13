#[cfg(test)]
mod tokenizer {
  use std::ffi::OsString;
  use hack_compiler::classes::tokenizer::*;
  use hack_compiler::grammar::LexicalElement;
  use hack_compiler::grammar::keyword;

  #[test]
  fn test_create_token() {
    let _token = match create_token(&"class".to_string()) {
      Some(token) => {
        assert!(token.element == LexicalElement::Keyword);
        assert_eq!(token.data, "class".to_string());
        assert_eq!(token.keyword_key, Some(keyword::CLASS));
      },
      _=> assert!(false, "Could not create token"),
    };
  }

  #[test]
  fn test_create_identifier_token() {
    let token = create_identifier_token(&"some_identifier".to_string());
    assert!(token.element == LexicalElement::Identifier);
    assert_eq!(token.data, "some_identifier".to_string());
    assert_eq!(token.keyword_key, None);
  }

  #[test]
  fn test_find_token_type() {
    assert_eq!(find_token_type(&"}".to_string()), Some(LexicalElement::Symbol));
    assert_eq!(find_token_type(&"{".to_string()), Some(LexicalElement::Symbol));
    assert_eq!(find_token_type(&"]".to_string()), Some(LexicalElement::Symbol));
    assert_eq!(find_token_type(&"20".to_string()), Some(LexicalElement::IntegerConstant));
    assert_eq!(find_token_type(&"2000000000".to_string()), None);
    assert_eq!(find_token_type(&"-49".to_string()), None);
    assert_eq!(find_token_type(&"\"Testing String\"".to_string()), Some(LexicalElement::StringConstant));
  }

  #[test]
  fn test_is_symbol() {
    assert_eq!(is_symbol(&"[".to_string()), true);
    assert_eq!(is_symbol(&"D".to_string()), false);
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
    assert_eq!(find_keyword(&"class".to_string()), Some(keyword::CLASS));
    assert_eq!(find_keyword(&"classes".to_string()), None);
  }

  #[test]
  fn test_is_whitespace() {
    assert_eq!(is_whitespace(' '), true);
    assert_eq!(is_whitespace('\t'), true);
    assert_eq!(is_whitespace(';'), false);
  }
}