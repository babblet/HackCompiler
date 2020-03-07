pub enum GrammarType {
    LEXICAL_ELEMENT,
    PROGRAM_STRUCTURE,
    STATEMENT,
    EXPRESSION,
}

pub enum LexicalElement {
    KEYWORD,
    SYMBOL,
    INTEGER_CONSTANT,
    STRING_CONSTANT,
    IDENTIFIER,
}

pub enum LexicalElementKeyword {
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
    THIS,
}

pub enum ProgramStructure {
    CLASS,
    CLASS_VAR_DEC,
    TYPE,
    SUBROUTINE_DEC,
    PARAMETER_LIST,
    SUBROUTINE_BODY,
    VAR_DEC,
    CLASS_NAME,
    SUBROUTINE_NAME,
    VAR_NAME,
}

pub enum Statements {
    STATEMENTS,
    STATEMENT,
    LET_STATEMENT,
    IF_STATEMENT,
    WHILE_STATEMENT,
    DO_STATEMENT,
    RETURN_STATEMENT,
}

pub enum Expression {
    EXPRESSION,
    TERM,
    SUBROUTINE_CALL,
    EXPRESSION_LIST,
    OP,
    UNARY_OP,
    KEYWORD_CONSTANT,   
}