use super::*;

#[test]
fn test_basic_commit() {
    let input: String = "feat: add a new feature".to_string();
    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.lex().unwrap();
    let commit: ConventionalCommit = parse_commit(tokens).unwrap();

    assert_eq!(commit.commit_type, "feat");
    assert_eq!(commit.scope, None);
    assert!(!commit.breaking_change);
    assert_eq!(commit.description, "add a new feature");
    assert_eq!(commit.body, None);
    assert_eq!(commit.footer, None);
}

#[test]
fn test_commit_with_scope() {
    let input: String = "fix(parser): fix a bug in the parser".to_string();
    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.lex().unwrap();
    let commit: ConventionalCommit = parse_commit(tokens).unwrap();

    assert_eq!(commit.commit_type, "fix");
    assert_eq!(commit.scope, Some("parser".to_string()));
    assert!(!commit.breaking_change);
    assert_eq!(commit.description, "fix a bug in the parser");
    assert_eq!(commit.body, None);
    assert_eq!(commit.footer, None);
}

#[test]
fn test_commit_with_breaking_change() {
    let input: String = "feat!: add a new feature that breaks API".to_string();
    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.lex().unwrap();
    let commit: ConventionalCommit = parse_commit(tokens).unwrap();

    assert_eq!(commit.commit_type, "feat");
    assert_eq!(commit.scope, None);
    assert!(commit.breaking_change);
    assert_eq!(commit.description, "add a new feature that breaks API");
    assert_eq!(commit.body, None);
    assert_eq!(commit.footer, None);
}

#[test]
fn test_commit_with_scope_and_breaking_change() {
    let input: String = "refactor(core)!: refactor core functionality".to_string();
    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.lex().unwrap();
    let commit: ConventionalCommit = parse_commit(tokens).unwrap();

    assert_eq!(commit.commit_type, "refactor");
    assert_eq!(commit.scope, Some("core".to_string()));
    assert!(commit.breaking_change);
    assert_eq!(commit.description, "refactor core functionality");
    assert_eq!(commit.body, None);
    assert_eq!(commit.footer, None);
}

#[test]
fn test_commit_with_body() {
    let input: String =
        "feat: add a new feature\n\nThis feature allows parsing of commits.".to_string();
    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.lex().unwrap();
    let commit: ConventionalCommit = parse_commit(tokens).unwrap();

    assert_eq!(commit.commit_type, "feat");
    assert_eq!(commit.scope, None);
    assert!(!commit.breaking_change);
    assert_eq!(commit.description, "add a new feature");
    assert_eq!(
        commit.body,
        Some("This feature allows parsing of commits.".to_string())
    );
    assert_eq!(commit.footer, None);
}

#[test]
fn test_commit_with_footer() {
    let input: String = "feat: add a new feature\n\nReviewed-by: Alice".to_string();
    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.lex().unwrap();
    let commit: ConventionalCommit = parse_commit(tokens).unwrap();

    assert_eq!(commit.commit_type, "feat");
    assert_eq!(commit.scope, None);
    assert!(!commit.breaking_change);
    assert_eq!(commit.description, "add a new feature");
    assert_eq!(commit.body, None);
    assert_eq!(commit.footer, Some("Reviewed-by: Alice".to_string()));
}

#[test]
fn test_commit_with_body_and_footer() {
    let input: String =
        "feat: add a new feature\n\nThis feature allows parsing of commits.\n\nReviewed-by: Alice"
            .to_string();
    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.lex().unwrap();
    let commit: ConventionalCommit = parse_commit(tokens).unwrap();

    assert_eq!(commit.commit_type, "feat");
    assert_eq!(commit.scope, None);
    assert!(!commit.breaking_change);
    assert_eq!(commit.description, "add a new feature");
    assert_eq!(
        commit.body,
        Some("This feature allows parsing of commits.".to_string())
    );
    assert_eq!(commit.footer, Some("Reviewed-by: Alice".to_string()));
}

#[test]
fn test_missing_colon_after_type() {
    let input: String = "feat add a new feature".to_string();
    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Result<Vec<Token>, String> = lexer.lex();

    assert!(tokens.is_err());
}

#[test]
fn test_missing_commit_type() {
    let input: String = ": add a new feature".to_string();
    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Result<Vec<Token>, String> = lexer.lex();

    assert!(tokens.is_err());
}

#[test]
fn test_missing_description() {
    let input: String = "feat: ".to_string();
    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Result<Vec<Token>, String> = lexer.lex();

    assert!(tokens.is_err());
}

#[test]
fn test_invalid_commit_type() {
    let input: String = "invalid add something".to_string();
    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Result<Vec<Token>, String> = lexer.lex();

    assert!(tokens.is_err());
}
