use leptos::prelude::*;
use regex::Regex;
use std::sync::OnceLock;

/// Enum representing different token types for syntax highlighting
#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Keyword,
    String,
    Comment,
    Macro,
    Type,
    Function,
    Number,
    Operator,
    Attribute,
    Plain,
}

impl TokenType {
    fn to_class(&self) -> &'static str {
        match self {
            TokenType::Keyword => "token-keyword",
            TokenType::String => "token-string",
            TokenType::Comment => "token-comment",
            TokenType::Macro => "token-macro",
            TokenType::Type => "token-type",
            TokenType::Function => "token-function",
            TokenType::Number => "token-number",
            TokenType::Operator => "token-operator",
            TokenType::Attribute => "token-attribute",
            TokenType::Plain => "token-plain",
        }
    }
}

/// Helper struct to hold a token
#[derive(Debug, PartialEq)]
struct Token {
    text: String,
    token_type: TokenType,
}

/// Compiled regex patterns, initialized once
fn get_patterns() -> &'static Vec<(Regex, TokenType)> {
    static PATTERNS: OnceLock<Vec<(Regex, TokenType)>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        vec![
            // Comments (single line)
            (Regex::new(r"^//.*").unwrap(), TokenType::Comment),
            // Strings
            (Regex::new(r#"^"[^"]*""#).unwrap(), TokenType::String),
            // Attributes (e.g., on:click, prop:value, class=...)
            (Regex::new(r"^[a-z]+:[a-z_]+").unwrap(), TokenType::Attribute),
            (Regex::new(r"^[a-z_]+=").unwrap(), TokenType::Attribute),
            // Macros
            (Regex::new(r"^[a-zA-Z_]+\!").unwrap(), TokenType::Macro),
            // Keywords
            (
                Regex::new(r"^(pub|fn|struct|enum|impl|use|mod|let|mut|if|else|match|move|return|const|static|trait|where|for|in|loop|while|break|continue|type)\b").unwrap(),
                TokenType::Keyword,
            ),
            // Types (starts with Uppercase)
            (Regex::new(r"^[A-Z][a-zA-Z0-9_]*").unwrap(), TokenType::Type),
            // Function calls - Simplified: identifiers followed by `(` (matched together, split later if needed, or just color the name)
            // Since Rust Regex doesn't support lookahead, we'll just match the identifier if possible,
            // or we might skip specific function highlighting requiring lookahead for simplicity and performance.
            // Let's rely on standard identifier matching. If it's not a keyword, it might be a function or var.
            // For now, let's remove the specific "function call" lookahead regex to fix the panic.
            // We can match `ident` generally as Plain (default) or handle known functions if we had a list.

            // Numbers
            (Regex::new(r"^\d+").unwrap(), TokenType::Number),
            // Operators
            (Regex::new(r"^(\->|=>|::|[+\-*/%=<>&|!])").unwrap(), TokenType::Operator),
            // Whitespace (keep as plain)
            (Regex::new(r"^\s+").unwrap(), TokenType::Plain),
        ]
    })
}

/// Tokenizer logic separated for testability
fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut remaining = code;
    let patterns = get_patterns();

    while !remaining.is_empty() {
        let mut matched = false;

        for (regex, token_type) in patterns {
            if let Some(mat) = regex.find(remaining) {
                let len = mat.end();
                // Ensure match is at the start (regex has ^ anchor but we double check logic)
                if len > 0 {
                    tokens.push(Token {
                        text: remaining[..len].to_string(),
                        token_type: token_type.clone(),
                    });
                    remaining = &remaining[len..];
                    matched = true;
                    break;
                }
            }
        }

        if !matched {
            // If no pattern matches, consume one character as plain text
            // This handles punctuation like { } ( ) ; , which we might want to color specifically later
            let char_len = remaining.chars().next().unwrap().len_utf8();
            tokens.push(Token {
                text: remaining[..char_len].to_string(),
                token_type: TokenType::Plain,
            });
            remaining = &remaining[char_len..];
        }
    }
    tokens
}

/// Highlights Rust/Leptos code by returning a list of styled View elements
pub fn highlight_code(code: String) -> Vec<impl IntoView> {
    let tokens = tokenize(&code);

    // Convert tokens to Views.
    tokens
        .into_iter()
        .map(|token| {
            // Move text into the closure
            let text = token.text;
            let class = token.token_type.to_class();
            view! { <span class=class>{text}</span> }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let code = "let x = 42;";
        let tokens = tokenize(code);

        // Expected: "let" (Keyword), " " (Plain), "x" (Plain), " " (Plain), "=" (Operator), " " (Plain), "42" (Number), ";" (Plain)
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[0].text, "let");

        assert_eq!(tokens[4].token_type, TokenType::Operator);
        assert_eq!(tokens[4].text, "=");

        assert_eq!(tokens[6].token_type, TokenType::Number);
        assert_eq!(tokens[6].text, "42");
    }

    #[test]
    fn test_tokenize_macro() {
        let code = "view! {";
        let tokens = tokenize(code);

        assert_eq!(tokens[0].token_type, TokenType::Macro);
        assert_eq!(tokens[0].text, "view!");
    }

    #[test]
    fn test_tokenize_attributes() {
        let code = "on:click=move";
        let tokens = tokenize(code);

        // "on:click" matched as Attribute
        assert_eq!(tokens[0].token_type, TokenType::Attribute);
        assert_eq!(tokens[0].text, "on:click");

        // "=" matched as Operator
        assert_eq!(tokens[1].token_type, TokenType::Operator);

        // "move" matched as Keyword
        assert_eq!(tokens[2].token_type, TokenType::Keyword);
    }
}
