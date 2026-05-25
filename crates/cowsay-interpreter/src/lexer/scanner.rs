//! Scanner implementation.

use crate::ast::token::Span;
use crate::ast::token::Token;
use crate::ast::token::Type;
use std::{iter::Peekable, str::CharIndices};

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
    /// Create a new scanner for the given source.
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
    fn scan_identifier(&mut self, start_idx: usize) {
        // Consume characters.
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        // Match keywords, otherwise identifier.
        let text = &self.source[start_idx..self.current_idx];
        let typ = match text {
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
                        // Just a regular LessThan token
                        self.add_token(Type::LessThan, start_idx); // Assuming you add this
                    }
                }

                '"' => {
                    self.scan_string(start_idx, '"');
                }

                // Ignore whitespace
                ' ' | '\r' | '\t' | '\n' => {}

                c if c.is_ascii_alphabetic() || c == '_' => {
                    self.scan_identifier(start_idx);
                }

                c if c.is_ascii_digit() => {
                    self.scan_number(start_idx);
                }

                _ => {
                    // LSP-friendly error token instead of panicking
                    self.add_token(Type::Error, start_idx);
                }
            }
        }

        // Always append EOF at the very end
        self.add_token(Type::EndOfFile, self.current_idx);
    }
}
