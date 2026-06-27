//! Scanner implementation.

use crate::ast::token::Span;
use crate::ast::token::Token;
use crate::ast::token::Type;
use crate::lexer::utils::is_valid_identifier;
use std::{iter::Peekable, str::CharIndices};

#[derive(Debug)]
/// Lexical scanner.
///
/// * `source`: Raw source code.
/// * `chars`: Char index peekable iterator of the source code.
/// * `tokens`: Token vector.
/// * `current_idx`: Current scanning index
pub struct Scanner<'a> {
    pub(crate) source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    tokens: Vec<Token>,
    pub(crate) current_idx: usize,
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
    pub(crate) fn advance(&mut self) -> Option<char> {
        if let Some((idx, c)) = self.chars.next() {
            // Update the index to point to the byte after this character
            self.current_idx = idx + c.len_utf8();
            Some(c)
        } else {
            None
        }
    }
    /// Consumes and returns the nth character, advancing the byte index.
    pub(crate) fn advance_nth(&mut self, n: usize) -> Option<char> {
        if let Some((idx, c)) = self.chars.nth(n) {
            // Update the index to point to the byte after this character
            self.current_idx = idx + c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    /// Looks at the next character without consuming it.
    pub(crate) fn peek(&mut self) -> Option<char> {
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

    /// Helper to create and store a token
    pub(crate) fn add_token(&mut self, typ: Type, start_idx: usize) {
        let span = Span::new(start_idx, self.current_idx);
        self.tokens.push(Token::new(typ, span));
    }

    /// Advance until a line break is found.
    /// Consumes the new-line character.
    pub(crate) fn advance_line(&mut self) {
        loop {
            if let Some(c) = self.peek() {
                if c == '\n' {
                    self.advance();
                    break;
                }
            } else {
                self.add_token(Type::Error, self.current_idx);
                break;
            }
            self.advance();
        }
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
                        self.scan_here_doc(start_idx);
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
                    self.scan_interpolation_string(start_idx);
                }
                '\'' => {
                    self.scan_string(start_idx);
                }
                '$' => {
                    self.scan_variable(start_idx);
                }
                ';' => {
                    self.add_token(Type::Semicolon, start_idx);
                }
                '#' => {
                    self.scan_comment(start_idx);
                }

                // Ignore whitespace
                ' ' | '\r' | '\t' | '\n' => {}

                c if is_valid_identifier(c) => {
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
    use pretty_assertions::assert_eq;

    #[test]
    fn can_read_direct_identifier() {
        let mut scanner = Scanner::new("$hello");
        scanner.scan_tokens();
        let expected = vec![
            Token::new(Type::Variable, Span::new(0, 6)),
            Token::new(Type::EndOfFile, Span::new(6, 6)),
        ];

        assert_eq!(scanner.tokens(), expected);
    }

    #[test]
    fn can_read_comments() {
        let mut scanner = Scanner::new("# hi\n$hello");
        scanner.scan_tokens();
        let expected = vec![
            Token::new(Type::Comment, Span::new(0, 5)),
            Token::new(Type::Variable, Span::new(5, 11)),
            Token::new(Type::EndOfFile, Span::new(11, 11)),
        ];

        assert_eq!(scanner.tokens(), expected);
    }

    #[test]
    fn can_read_simple_assignment() {
        let mut scanner = Scanner::new("$var = 'hello'");
        scanner.scan_tokens();
        let expected = vec![
            Token::new(Type::Variable, Span::new(0, 4)),
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

    #[test]
    fn can_read_print_declaration() {
        let source = "print 'hello';";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        let expected = vec![
            Token::new(Type::KeywordPrint, Span::new(0, 5)),
            Token::new(Type::LiteralString, Span::new(6, 13)),
            Token::new(Type::Semicolon, Span::new(13, 14)),
            Token::new(Type::EndOfFile, Span::new(14, 14)),
        ];

        assert_eq!(scanner.tokens(), expected);
    }

    #[test]
    fn can_read_variable_declaration() {
        let source = r"$var='hello';";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        let expected = vec![
            Token::new(Type::Variable, Span::new(0, 4)),
            Token::new(Type::Equal, Span::new(4, 5)),
            Token::new(Type::LiteralString, Span::new(5, 12)),
            Token::new(Type::Semicolon, Span::new(12, 13)),
            Token::new(Type::EndOfFile, Span::new(13, 13)),
        ];

        assert_eq!(scanner.tokens(), expected);
    }

    #[test]
    fn can_read_variable_declaration_with_spaces() {
        let source = r#"$var = "hello";"#;
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        let expected = vec![
            Token::new(Type::Variable, Span::new(0, 4)),
            Token::new(Type::Equal, Span::new(5, 6)),
            Token::new(Type::LiteralStringInterpolationStart, Span::new(7, 8)),
            Token::new(Type::LiteralString, Span::new(8, 13)),
            Token::new(Type::LiteralStringInterpolationEnd, Span::new(13, 14)),
            Token::new(Type::Semicolon, Span::new(14, 15)),
            Token::new(Type::EndOfFile, Span::new(15, 15)),
        ];

        assert_eq!(scanner.tokens(), expected);
    }

    #[test]
    fn can_read_here_doc() {
        let source = "$var = <<EOF;\nhello, this is a here document\nEOF\n";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        let expected = vec![
            Token::new(Type::Variable, Span::new(0, 4)),
            Token::new(Type::Equal, Span::new(5, 6)),
            Token::new(Type::Redirect, Span::new(7, 9)),
            Token::new(Type::Identifier, Span::new(9, 12)),
            Token::new(Type::Semicolon, Span::new(12, 13)),
            Token::new(Type::LiteralString, Span::new(14, 44)),
            Token::new(Type::Identifier, Span::new(46, 49)),
            Token::new(Type::RedirectEnd, Span::new(49, 49)),
            Token::new(Type::EndOfFile, Span::new(49, 49)),
        ];

        assert_eq!(scanner.tokens(), expected);
    }
}
