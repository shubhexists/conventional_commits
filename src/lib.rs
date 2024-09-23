//! # Conventional Commits Parser
//!
//! This module provides a parser for [Conventional Commits](https://www.conventionalcommits.org/),
//! a specification for adding human and machine-readable meaning to commit messages.
//!
//! The parser consists of three main components:
//! - `Token`: An enum representing different parts of a conventional commit message.
//! - `ConventionalCommit`: A struct representing a parsed conventional commit.
//! - `Lexer`: A struct that tokenizes the input commit message.
//!
//! ## Usage
//!
//! To parse a conventional commit message:
//!
//! ```rust
//! use conventional_commit::{parse_commit, Lexer};
//!
//! fn parse_example() -> Result<(), String> {
//!     let input = "feat(parser): add ability to parse conventional commits".to_string();
//!     let mut lexer = Lexer::new(input);
//!     let tokens = lexer.lex()?;
//!     let commit = parse_commit(tokens)?;
//!     println!("{:?}", commit);
//!     Ok(())
//! }
//!
//! // Call the function in the doctest
//! parse_example().unwrap();
//! ```
//!
//! ## Error Handling
//!
//! Both the lexing and parsing stages return `Result` types, allowing for error handling.
//! Common errors include invalid commit type, missing description, and unclosed scope parentheses.

/// Represents the different components of a conventional commit message.

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum Token {
    /// The type of the commit, e.g., "feat", "fix", "docs", etc.
    CommitType(String),
    /// The scope of the commit, if provided.
    Scope(String),
    /// Indicates a breaking change in the commit.
    BreakingChangeMarker,
    /// The short description of the commit.
    Description(String),
    /// The body of the commit message, providing additional contextual information.
    Body(String),
    /// The footer of the commit message, often used for referencing issues.
    Footer(String),
}

/// Represents a parsed conventional commit.
#[derive(Debug)]
pub struct ConventionalCommit {
    /// The type of the commit, e.g., "feat", "fix", "docs", etc.
    pub commit_type: String,
    /// The scope of the commit, if provided.
    pub scope: Option<String>,
    /// Indicates whether this commit introduces a breaking change.
    pub breaking_change: bool,
    /// The short description of the commit.
    pub description: String,
    /// The body of the commit message, providing additional contextual information.
    pub body: Option<String>,
    /// The footer of the commit message, often used for referencing issues.
    pub footer: Option<String>,
}

impl ConventionalCommit {
    /// Creates a new `ConventionalCommit` instance.
    ///
    /// # Arguments
    ///
    /// * `commit_type` - The type of the commit.
    /// * `scope` - An optional scope for the commit.
    /// * `breaking_change` - Whether the commit introduces a breaking change.
    /// * `description` - A short description of the commit.
    /// * `body` - An optional body providing more context.
    /// * `footer` - An optional footer, often used for issue references.
    fn new(
        commit_type: String,
        scope: Option<String>,
        breaking_change: bool,
        description: String,
        body: Option<String>,
        footer: Option<String>,
    ) -> Self {
        ConventionalCommit {
            commit_type,
            scope,
            breaking_change,
            description,
            body,
            footer,
        }
    }
}

/// A lexer for tokenizing conventional commit messages.
pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    /// Creates a new `Lexer` instance with the given input string.
    ///
    /// # Arguments
    ///
    /// * `input` - The commit message to be tokenized.
    pub fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    /// Returns the next character in the input without consuming it.
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    /// Consumes and returns the next character in the input.
    fn next(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let c: char = self.input.chars().nth(self.position)?;
            self.position += 1;
            Some(c)
        } else {
            None
        }
    }

    /// Skips any whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    /// Tokenizes the commit type.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a `CommitType` token or an error message.
    pub fn lex_commit_type(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        let mut commit_type: String = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                commit_type.push(c);
                self.next();
            } else if c == '(' {
                return Ok(Token::CommitType(commit_type));
            } else {
                break;
            }
        }

        if !commit_type.is_empty() {
            Ok(Token::CommitType(commit_type))
        } else {
            Err("Invalid commit type".to_string())
        }
    }

    /// Tokenizes the scope of the commit.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a `Scope` token or an error message.
    pub fn lex_scope(&mut self) -> Result<Token, String> {
        if self.next() != Some('(') {
            return Err("Expected '(' for scope".to_string());
        }

        let mut scope: String = String::new();
        while let Some(c) = self.next() {
            if c == ')' {
                return Ok(Token::Scope(scope));
            } else {
                scope.push(c);
            }
        }

        Err("Unclosed scope".to_string())
    }

    /// Tokenizes the breaking change marker.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a `BreakingChangeMarker` token or an error message.
    pub fn lex_breaking_change_marker(&mut self) -> Result<Token, String> {
        if self.peek() == Some('!') {
            self.next();
            Ok(Token::BreakingChangeMarker)
        } else {
            Err("No breaking change marker".to_string())
        }
    }

    /// Tokenizes the commit description.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a `Description` token or an error message.
    pub fn lex_description(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        let mut description: String = String::new();
        while let Some(c) = self.next() {
            if c == '\n' {
                break;
            } else {
                description.push(c);
            }
        }

        if !description.is_empty() {
            Ok(Token::Description(description))
        } else {
            Err("Missing description".to_string())
        }
    }

    /// Tokenizes the body and footer of the commit message.
    ///
    /// # Returns
    ///
    /// A tuple containing optional `Body` and `Footer` tokens.
    pub fn lex_body_and_footer(&mut self) -> (Option<Token>, Option<Token>) {
        self.skip_whitespace();

        let remaining_input: &str = &self.input[self.position..];

        if let Some(index) = remaining_input
            .find("BREAKING CHANGE:")
            .or_else(|| remaining_input.find("Reviewed-by:"))
            .or_else(|| remaining_input.find("Refs:"))
        {
            let (body_part, footer_part) = remaining_input.split_at(index);

            let body: Option<Token> = if !body_part.trim().is_empty() {
                Some(Token::Body(body_part.trim().to_string()))
            } else {
                None
            };

            let footer: Option<Token> = Some(Token::Footer(footer_part.trim().to_string()));

            return (body, footer);
        }

        if !remaining_input.is_empty() {
            let body: Option<Token> = Some(Token::Body(remaining_input.trim().to_string()));
            self.position += remaining_input.len();
            return (body, None);
        }

        (None, None)
    }

    /// Tokenizes the entire input commit message.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a vector of `Token`s or an error message.
    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();

        let commit_type: Token = self.lex_commit_type()?;
        tokens.push(commit_type);

        if self.peek() == Some('(') {
            let scope: Token = self.lex_scope()?;
            tokens.push(scope);
        }

        if self.peek() == Some('!') {
            let breaking_change: Token = self.lex_breaking_change_marker()?;
            tokens.push(breaking_change);
        }

        if self.next() != Some(':') {
            return Err("Expected ':' after commit type or scope".to_string());
        }

        let description: Token = self.lex_description()?;
        tokens.push(description);

        let (body, footer) = self.lex_body_and_footer();
        if let Some(body_token) = body {
            tokens.push(body_token);
        }
        if let Some(footer_token) = footer {
            tokens.push(footer_token);
        }

        Ok(tokens)
    }
}

/// Parses a vector of `Token`s into a `ConventionalCommit` struct.
///
/// # Arguments
///
/// * `tokens` - A vector of `Token`s produced by the lexer.
///
/// # Returns
///
/// A `Result` containing either a `ConventionalCommit` or an error message.
pub fn parse_commit(tokens: Vec<Token>) -> Result<ConventionalCommit, String> {
    let mut commit_type: Option<String> = None;
    let mut scope: Option<String> = None;
    let mut breaking_change: bool = false;
    let mut description: Option<String> = None;
    let mut body: Option<String> = None;
    let mut footer: Option<String> = None;

    for token in tokens {
        match token {
            Token::CommitType(t) => commit_type = Some(t),
            Token::Scope(s) => scope = Some(s),
            Token::BreakingChangeMarker => breaking_change = true,
            Token::Description(d) => description = Some(d),
            Token::Body(b) => body = Some(b),
            Token::Footer(f) => footer = Some(f),
        }
    }

    if let Some(commit_type) = commit_type {
        if let Some(description) = description {
            Ok(ConventionalCommit::new(
                commit_type,
                scope,
                breaking_change,
                description,
                body,
                footer,
            ))
        } else {
            Err("Missing description".to_string())
        }
    } else {
        Err("Missing commit type".to_string())
    }
}
