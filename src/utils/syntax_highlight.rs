use leptos::prelude::*;
use regex::Regex;

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
struct Token {
    text: String,
    token_type: TokenType,
}

/// Highlights Rust/Leptos code by returning a list of styled View elements
pub fn highlight_code(code: String) -> Vec<impl IntoView> {
    let mut tokens = Vec::new();
    let remaining_str = code.clone();
    let mut remaining = remaining_str.as_str();

    // Regex definitions
    // Note: Order matters. More specific patterns should come first.
    let patterns = [
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
        // Function calls (identifier followed by paren)
        (Regex::new(r"^[a-z_][a-zA-Z0-9_]*(?=\()").unwrap(), TokenType::Function),
        // Numbers
        (Regex::new(r"^\d+").unwrap(), TokenType::Number),
        // Operators
        (Regex::new(r"^(\->|=>|::|[+\-*/%=<>&|!])").unwrap(), TokenType::Operator),
        // Whitespace (keep as plain)
        (Regex::new(r"^\s+").unwrap(), TokenType::Plain),
    ];

    while !remaining.is_empty() {
        let mut matched = false;

        for (regex, token_type) in &patterns {
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
