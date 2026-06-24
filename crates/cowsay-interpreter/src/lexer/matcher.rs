//! Scanner implementation.

use crate::ast::token::Type;
use crate::lexer::scanner::Scanner;

use crate::lexer::utils;

impl Scanner<'_> {
    /// Consume a series of characters until the termination character is
    /// reached. Add a series of tokens being either n literal string of a
    /// variable.
    fn extract_interpolation_string_tokens<T>(
        &mut self,
        start_idx: usize,
        termination_func: T,
    ) where
        T: Fn(char, usize) -> bool,
    {
        let mut string_start_idx = start_idx + 1;
        while let Some(c) = self.peek() {
            if termination_func(c, self.current_idx) {
                self.add_token(Type::LiteralString, string_start_idx);
                self.advance();
                break;
            } else if c == '$' {
                self.add_token(Type::LiteralString, string_start_idx);
                self.scan_variable(self.current_idx);
                string_start_idx = self.current_idx;
            }
            self.advance();
        }
    }

    /// Consume $, then treat as identifier
    pub(crate) fn scan_variable(&mut self, start_idx: usize) {
        // Consume $
        self.advance();
        self.scan_identifier(start_idx);
    }

    /// Scans a sequence of letters/numbers and checks if it is a keyword.
    /// * `start_idx`: Identifier zero index ($).
    pub(crate) fn scan_identifier(&mut self, start_idx: usize) {
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
            "print" => Type::KeywordPrint,
            text if text.starts_with('$') => Type::Variable,
            _ => Type::Identifier,
        };

        self.add_token(typ, start_idx);
    }

    /// Scans a sequence of digits
    pub(crate) fn scan_number(&mut self, start_idx: usize) {
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

    /// Consume a series of characters until the termination character "'" is reached
    pub(crate) fn scan_string(&mut self, start_idx: usize) {
        // Consume characters.
        while let Some(c) = self.peek() {
            self.advance();
            if c != '\'' {
                continue;
            }
            break;
        }

        self.add_token(Type::LiteralString, start_idx);
    }

    /// Consume a series of characters until the termination character "'" is
    /// reached. Add a series of tokens being either n literal string of a
    /// variable.
    pub(crate) fn scan_interpolation_string(&mut self, start_idx: usize) {
        self.add_token(Type::LiteralStringInterpolationStart, start_idx);
        self.advance();
        self.extract_interpolation_string_tokens(start_idx, |c, _| c == '"');
        self.add_token(
            Type::LiteralStringInterpolationEnd,
            self.current_idx - 1,
        );
    }

    /// Consumes a here document with variables as a series of tokens.
    pub(crate) fn scan_here_doc(&mut self, start_idx: usize) {
        let delim_start_idx = start_idx + 2;
        // Consume delimiter
        while let Some(c) = self.peek() {
            if c == ';' {
                self.add_token(Type::Identifier, delim_start_idx);
                break;
            }
            self.advance();
        }
        self.advance();
        self.add_token(Type::Semicolon, self.current_idx - 1);
        let delimiter_len = self.current_idx - 3 - start_idx;
        self.advance_line();

        let remaining_source = &self.source[self.current_idx..];
        let delimiter =
            &self.source[delim_start_idx..delim_start_idx + delimiter_len];
        let search_target = format!("\n{delimiter}");
        let delim_end_idx = remaining_source.find(&search_target);
        if let Some(idx) = delim_end_idx {
            let interpolation_start_idx = self.current_idx - 1;
            self.extract_interpolation_string_tokens(
                interpolation_start_idx,
                |_, i| i > idx + interpolation_start_idx,
            );
        } else {
            self.add_token(Type::Error, self.current_idx);
            return;
        }
        self.advance_nth(delimiter_len);

        self.add_token(Type::Identifier, self.current_idx - delimiter_len);
        self.add_token(Type::RedirectEnd, self.current_idx);
    }
}
