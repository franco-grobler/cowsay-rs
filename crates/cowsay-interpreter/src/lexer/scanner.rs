//! Scanner implementation.

use crate::ast::token::Span;
use crate::ast::token::Token;
use crate::ast::token::Type;
use std::{iter::Peekable, str::CharIndices};

use crate::lexer::utils;

// (Assuming your Token, Type, and Span structs are imported here)

#[derive(Debug)]
/// Lexical scanner.
///
/// * `source`: Raw source code.
/// * `chars`: Char index peekable iterator of the source code.
/// * `tokens`: Token vector.
/// * `current_idx`: Current scanning index
pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    tokens: Vec<Token>,
    current_idx: usize,
}

impl<'a> Scanner<'a> {
    /// Create a new [`Scanner`].
    ///
    /// * `source`: Raw source code.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            tokens: Vec::new(),
            current_idx: 0,
        }
    }

    /// Consumes and returns the next character, advancing the byte index.
    fn advance(&mut self) -> Option<char> {
        if let Some((idx, c)) = self.chars.next() {
            // Update the index to point to the byte after this character
            self.current_idx = idx + c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    /// Looks at the next character without consuming it.
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    /// Consumes the next character if it matches the expected one.
    fn match_char(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(c) if c == expected => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    /// Scans a sequence of letters/numbers and checks if it is a keyword.
    /// * `start_idx`: Identifier zero index ($).
    fn scan_identifier(&mut self, start_idx: usize) {
        // Consume $
        self.advance();
        // Consume characters.
        while let Some(c) = self.peek() {
            if utils::is_valid_identifier(c) {
                self.advance();
            } else {
                break;
            }
        }

        // Match keywords, otherwise identifier.
        let text = &self.source[start_idx..self.current_idx];
        let typ = match text {
            "true" | "false" => Type::LiteralBoolean,
            "unless" => Type::KeywordUnless,
            "ne" => Type::KeywordNotEqual,
            _ => Type::Identifier,
        };

        self.add_token(typ, start_idx);
    }

    /// Scans a sequence of digits
    fn scan_number(&mut self, start_idx: usize) {
        // Consume digits
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Read floating point numbers
        if self.peek() == Some('.') {
            // Consume the '.'
            self.advance();
            // Consume the fractional digits
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.add_token(Type::LiteralNumber, start_idx);
    }

    /// Scans a sequence of letters/numbers and checks if it is a keyword.
    fn scan_string(&mut self, start_idx: usize, termination_char: char) {
        // Consume characters.
        while let Some(c) = self.peek() {
            self.advance();
            if c != termination_char {
                continue;
            }
            break;
        }

        self.add_token(Type::LiteralString, start_idx);
    }

    /// Helper to create and store a token
    fn add_token(&mut self, typ: Type, start_idx: usize) {
        let span = Span::new(start_idx, self.current_idx);
        self.tokens.push(Token::new(typ, span));
    }

    /// Main loop to scan all tokens
    pub fn scan_tokens(&mut self) {
        // Keep looping until advance() returns None
        while let Some(c) = self.peek() {
            // Record where this lexeme starts
            let start_idx = self.current_idx;

            // Consume the character
            self.advance();

            match c {
                '(' => self.add_token(Type::ParenthesisLeft, start_idx),
                ')' => self.add_token(Type::ParenthesisRight, start_idx),
                '+' => self.add_token(Type::Plus, start_idx),
                '-' => self.add_token(Type::Minus, start_idx),
                '=' => {
                    let typ = if self.match_char('=') {
                        Type::EqualEqual // Assuming you have this
                    } else {
                        Type::Equal
                    };
                    self.add_token(typ, start_idx);
                }
                '<' => {
                    if self.match_char('<') {
                        if self.match_char('-') {
                            self.add_token(Type::RedirectStrip, start_idx);
                        } else {
                            self.add_token(Type::Redirect, start_idx);
                        }
                    } else {
                        self.add_token(Type::LessThan, start_idx);
                    }
                }
                '.' => {
                    if self.match_char('=') {
                        self.add_token(Type::DotEqual, start_idx);
                    } else {
                        self.add_token(Type::Error, start_idx);
                    }
                }

                '"' => {
                    self.scan_string(start_idx, '"');
                }

                '\n' => {
                    self.add_token(Type::NewLine, start_idx);
                }

                // Ignore whitespace
                ' ' | '\r' | '\t' => {}

                '$' => {
                    self.scan_identifier(start_idx);
                }

                c if c.is_ascii_digit() => {
                    self.scan_number(start_idx);
                }

                _ => {
                    self.add_token(Type::Error, start_idx);
                }
            }
        }

        // Always append EOF at the very end
        self.add_token(Type::EndOfFile, self.current_idx);
    }

    /// Get tokens.
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::token::{Span, Token, Type},
        lexer::scanner::Scanner,
    };

    #[test]
    fn can_read_direct_identifier() {
        let mut scanner = Scanner::new("$hello");
        scanner.scan_tokens();
        let expected = vec![
            Token::new(Type::Identifier, Span::new(0, 6)),
            Token::new(Type::EndOfFile, Span::new(6, 6)),
        ];

        assert_eq!(scanner.tokens(), expected);
    }

    #[test]
    fn can_read_simple_assignment() {
        let mut scanner = Scanner::new("$var = \"hello\"");
        scanner.scan_tokens();
        let expected = vec![
            Token::new(Type::Identifier, Span::new(0, 4)),
            Token::new(Type::Equal, Span::new(5, 6)),
            Token::new(Type::LiteralString, Span::new(7, 14)),
            Token::new(Type::EndOfFile, Span::new(14, 14)),
        ];

        assert_eq!(scanner.tokens(), expected);
    }

    #[test]
    fn can_read_comparison() {
        let mut scanner = Scanner::new("1 < 2");
        scanner.scan_tokens();
        let expected = vec![
            Token::new(Type::LiteralNumber, Span::new(0, 1)),
            Token::new(Type::LessThan, Span::new(2, 3)),
            Token::new(Type::LiteralNumber, Span::new(4, 5)),
            Token::new(Type::EndOfFile, Span::new(5, 5)),
        ];

        assert_eq!(scanner.tokens(), expected);
    }
}
