# Conventional Commits Parser

A robust Rust library for parsing [Conventional Commits](https://www.conventionalcommits.org/), adding human and machine-readable meaning to commit messages.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Valid Commit Structure](#valid-commit-structure)
- [Invalid Commits](#invalid-commits)
- [API Reference](#api-reference)
- [Contributing](#contributing)
- [License](#license)

## Features

- Parse Conventional Commits into structured data
- Tokenize commit messages for detailed analysis
- Identify commit type, scope, description, body, and footer
- Detect breaking changes
- Robust error handling for invalid commits

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
conventional-commit = "0.1.0"
```

## Usage

Here's a quick example of how to use the Conventional Commits Parser:

```rust
use conventional_commit::{parse_commit, Lexer};

fn main() -> Result<(), String> {
    let input = "feat(parser): add ability to parse conventional commits".to_string();
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex()?;
    let commit = parse_commit(tokens)?;
    println!("{:?}", commit);
    Ok(())
}
```

## Valid Commit Structure

A valid Conventional Commit has the following structure:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Components:

1. **Type**: Describes the kind of change (e.g., feat, fix, docs, style, refactor, test, chore).

   - Must be lowercase.
   - Examples: `feat`, `fix`, `docs`

2. **Scope** (optional): Describes what area of the project is changing.

   - Enclosed in parentheses.
   - Can use kebab-case, lowercase, or uppercase.
   - Examples: `(parser)`, `(ui)`, `(docs)`

3. **Breaking Change Indicator** (optional): An exclamation mark (`!`) before the colon.

   - Indicates a breaking change.
   - Example: `feat!:` or `feat(scope)!:`

4. **Description**: A brief explanation of the change.

   - Separated from the type (and scope) by a colon and space.
   - Written in the imperative mood.
   - Example: `add ability to parse conventional commits`

5. **Body** (optional): Provides additional contextual information about the change.

   - Must be separated from the description by a blank line.
   - Can be multiple paragraphs.

6. **Footer** (optional): Used for referencing issues, noting breaking changes, etc.
   - Must start with a word token followed by `:<space>` or `<space>#`.
   - Common footers: `BREAKING CHANGE:`, `Refs:`, `Reviewed-by:`

### Examples of Valid Commits:

```
feat(parser): add ability to parse conventional commits

This commit introduces a new parser for Conventional Commits.
The parser can identify commit types, scopes, and descriptions.

Refs: #123
```

```
fix!: correct critical bug in authentication flow

BREAKING CHANGE: This changes the API for user authentication.
Old auth tokens will no longer be valid.
```

```
docs: update README with new API examples

Updated the README to include examples of how to use
the new parsing functions introduced in v2.0.0.
```

## Invalid Commits

The following are examples of invalid commits and why they're incorrect:

1. Missing type:

   ```
   add new feature
   ```

   Error: Commit type is missing.

2. Unclosed scope:

   ```
   feat(parser: add new parsing logic
   ```

   Error: Scope is not properly closed with a parenthesis.

3. Missing description:

   ```
   feat(ui):
   ```

   Error: Description is missing.


## API Reference

For detailed API documentation, please refer to the rustdoc comments in the source code.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
