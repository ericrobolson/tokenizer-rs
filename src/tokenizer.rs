use crate::{
    error::Error,
    location::Location,
    token::{Token, TokenKind},
};

pub fn tokenize(contents: &str, location: Location) -> Result<Vec<Token>, Error> {
    Tokenizer::tokenize(contents, location)
}

pub struct Tokenizer {
    contents: String,
    index: usize,
    location: Location,
}
impl Tokenizer {
    pub fn tokenize(contents: &str, location: Location) -> Result<Vec<Token>, Error> {
        let contents = contents.replace("\r\n", "\n");
        let mut tokens = Vec::new();
        let mut tokenizer = Tokenizer {
            index: 0,
            contents,
            location,
        };

        while let Some(c) = tokenizer.peek_char() {
            if c == '#' {
                let token = tokenizer.read_comment()?;
                tokens.push(token);
            } else if c == '"' {
                let token = tokenizer.read_string_literal()?;
                tokens.push(token);
            } else if c.is_whitespace() {
                tokenizer.next_char();
            } else {
                // If it's not a number, check to see if it starts with a '-' or '.'
                // and if the next character is a number.
                let mut is_numeric = c.is_numeric();
                if !is_numeric && tokenizer.index + 1 < tokenizer.contents.len() {
                    let next_char = tokenizer.contents.chars().nth(tokenizer.index).unwrap();
                    if next_char == '-' || next_char == '.' {
                        is_numeric = tokenizer
                            .contents
                            .chars()
                            .nth(tokenizer.index + 1)
                            .unwrap()
                            .is_numeric();
                    }
                }

                let token = if is_numeric {
                    tokenizer.read_number()?
                } else if is_symbol(c) {
                    tokenizer.read_symbol()?
                } else {
                    tokenizer.read_identifier()?
                };

                tokens.push(token);
            }
        }

        Ok(tokens)
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        let location = self.location.clone();
        let mut buffer = String::new();

        // Chomp the first character
        let mut has_period = false;

        let c = self.next_char().unwrap().0;
        buffer.push(c);

        if c == '.' {
            has_period = true;
        }

        while let Some(c) = self.peek_char() {
            // float case
            if c == '.' && !has_period {
                has_period = true;
            } else if c == '.' && has_period {
                return Err(Error {
                    message: "Float literal cannot have multiple decimal points".to_string(),
                    location: self.location.clone(),
                });
            } else if !c.is_numeric() {
                break;
            }
            buffer.push(c);
            self.next_char();
        }

        let kind = if has_period {
            TokenKind::FloatLiteral(buffer.parse().unwrap())
        } else {
            TokenKind::IntegerLiteral(buffer.parse().unwrap())
        };

        Ok(Token {
            location,
            contents: buffer,
            kind,
        })
    }

    fn read_symbol(&mut self) -> Result<Token, Error> {
        const TWO_CHAR_SYMBOLS: [&str; 10] =
            ["==", "!=", ">=", "<=", "->", "=>", "*=", "-=", "+=", "/="];

        let location = self.location.clone();
        let mut buffer = String::new();
        let first_char = self.contents.chars().nth(self.index).unwrap();
        buffer.push(first_char);

        // Check if the next char is a valid symbol
        if self.index + 1 < self.contents.len() {
            let second_char = self.contents.chars().nth(self.index + 1).unwrap();
            if TWO_CHAR_SYMBOLS.contains(&format!("{}{}", first_char, second_char).as_str()) {
                buffer.push(second_char);
            }
        }

        for _ in 0..buffer.len() {
            self.next_char();
        }

        Ok(Token {
            location,
            contents: buffer.clone(),
            kind: TokenKind::Symbol(buffer),
        })
    }

    fn read_identifier(&mut self) -> Result<Token, Error> {
        let location = self.location.clone();

        let (token, _) = self.read_token().unwrap();
        Ok(Token {
            location,
            contents: token.clone(),
            kind: TokenKind::Identifier(token),
        })
    }

    fn read_comment(&mut self) -> Result<Token, Error> {
        let location = self.location.clone();
        // Chomp the '#'
        self.next_char();

        let (comment, _) = match self.read_until_endline() {
            Some((comment, location)) => (comment, location),
            None => (String::new(), location.clone()),
        };
        Ok(Token {
            location,
            contents: comment.trim().to_string(),
            kind: TokenKind::Comment(comment.trim().to_string()),
        })
    }

    fn read_string_literal(&mut self) -> Result<Token, Error> {
        let location = self.location.clone();
        let mut buffer = String::new();

        // Chomp the '"'
        self.next_char();
        let mut closed = false;
        let mut prev_char = None;
        while let Some(c) = self.peek_char() {
            if c == '"' {
                if prev_char != Some('\\') {
                    closed = true;
                    self.next_char();
                    break;
                } else {
                    buffer.pop();
                }
            }
            buffer.push(c);
            prev_char = Some(c);
            self.next_char();
        }

        if !closed {
            return Err(Error {
                message: "Unclosed string".to_string(),
                location,
            });
        }

        Ok(Token {
            location,
            contents: buffer.clone(),
            kind: TokenKind::StringLiteral(buffer),
        })
    }

    /// Reads a token from the contents, stopping before the next token.
    fn read_token(&mut self) -> Option<(String, Location)> {
        if self.peek_char().is_none() {
            return None;
        }

        let mut buffer = String::new();
        let location = self.location.clone();
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() || is_symbol(c) {
                break;
            }
            buffer.push(c);
            self.next_char();
        }

        if buffer.is_empty() {
            return None;
        }

        Some((buffer, location))
    }

    fn read_until_endline(&mut self) -> Option<(String, Location)> {
        let mut buffer = String::new();
        let location = self.location.clone();

        while let Some(c) = self.peek_char() {
            if c == '\n' {
                break;
            }
            buffer.push(c);
            self.next_char();
        }

        if buffer.is_empty() {
            return None;
        }

        Some((buffer, location))
    }

    fn peek_char(&self) -> Option<char> {
        self.contents.chars().nth(self.index)
    }

    /// Returns the next character and updates the location
    fn next_char(&mut self) -> Option<(char, Location)> {
        let c = self.contents.chars().nth(self.index)?;
        let location = self.location.clone();

        self.index += 1;
        self.location.column += 1;
        if c == '\n' {
            self.location.row += 1;
            self.location.column = 0;
        }

        Some((c, location))
    }
}

fn is_symbol(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '=' | '>' | '<' | '!' | '?' | '.' | ',' | ';' | ':' | '(' | ')'
        | '[' | ']' | '{' | '}' | '&' | '|' | '^' | '%' | '~' | '#' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let contents = "";
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 0);

        let contents = "\n\n \r\n";
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn empty_comment() {
        let contents = r#"#"#;
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Comment("".to_string()));
        assert_eq!(tokens[0].contents, "");
        assert_eq!(tokens[0].location, (0, 0).into());
    }

    #[test]
    fn single_comment() {
        let contents = r#"# This is a comment"#;
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].kind,
            TokenKind::Comment("This is a comment".to_string())
        );
        assert_eq!(tokens[0].contents, "This is a comment");
        assert_eq!(tokens[0].location, (0, 0).into());
    }

    #[test]
    fn single_identifier() {
        let contents = r#"my_variable"#;
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].kind,
            TokenKind::Identifier("my_variable".to_string())
        );
        assert_eq!(tokens[0].contents, "my_variable");
        assert_eq!(tokens[0].location, (0, 0).into());
    }

    #[test]
    fn multiple_identifiers_with_space() {
        let contents = r#"my_variable my_variable2"#;
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0].kind,
            TokenKind::Identifier("my_variable".to_string())
        );
        assert_eq!(tokens[0].contents, "my_variable");
        assert_eq!(tokens[0].location, (0, 0).into());
        assert_eq!(
            tokens[1].kind,
            TokenKind::Identifier("my_variable2".to_string())
        );
        assert_eq!(tokens[1].contents, "my_variable2");
        assert_eq!(tokens[1].location, (0, 12).into());
    }

    #[test]
    fn multiple_identifiers_with_newline() {
        let contents = "my_variable \nmy_variable2";
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0].kind,
            TokenKind::Identifier("my_variable".to_string())
        );
        assert_eq!(tokens[0].contents, "my_variable");
        assert_eq!(tokens[0].location, (0, 0).into());
        assert_eq!(
            tokens[1].kind,
            TokenKind::Identifier("my_variable2".to_string())
        );
        assert_eq!(tokens[1].contents, "my_variable2");
        assert_eq!(tokens[1].location, (1, 0).into());
    }

    #[test]
    fn multiple_identifiers_with_comment() {
        let contents = "my_variable \n# This is a comment\nmy_variable2";
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(
            tokens[0].kind,
            TokenKind::Identifier("my_variable".to_string())
        );
        assert_eq!(tokens[0].contents, "my_variable");
        assert_eq!(tokens[0].location, (0, 0).into());
        assert_eq!(
            tokens[1].kind,
            TokenKind::Comment("This is a comment".to_string())
        );
        assert_eq!(tokens[1].contents, "This is a comment");
        assert_eq!(tokens[1].location, (1, 0).into());
        assert_eq!(
            tokens[2].kind,
            TokenKind::Identifier("my_variable2".to_string())
        );
        assert_eq!(tokens[2].contents, "my_variable2");
        assert_eq!(tokens[2].location, (2, 0).into());
    }

    #[test]
    fn identifier_splits_on_comment() {
        let contents = "my_variable# This is a comment\nmy_variable2";
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(
            tokens[0].kind,
            TokenKind::Identifier("my_variable".to_string())
        );
        assert_eq!(tokens[0].contents, "my_variable");
        assert_eq!(tokens[0].location, (0, 0).into());
        assert_eq!(
            tokens[1].kind,
            TokenKind::Comment("This is a comment".to_string())
        );
        assert_eq!(tokens[1].contents, "This is a comment");
        assert_eq!(tokens[1].location, (0, 11).into());
        assert_eq!(
            tokens[2].kind,
            TokenKind::Identifier("my_variable2".to_string())
        );
        assert_eq!(tokens[2].contents, "my_variable2");
        assert_eq!(tokens[2].location, (1, 0).into());
    }

    #[test]
    fn string_literal_unclosed_returns_error() {
        let contents = r#""This is a string"#;
        let result = Tokenizer::tokenize(contents, (0, 0).into());
        let expected = Err(Error {
            message: "Unclosed string".to_string(),
            location: (0, 0).into(),
        });

        assert_eq!(expected, result);
    }

    #[test]
    fn string_literal() {
        let contents = r#""This is a string""#;
        let result = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].kind,
            TokenKind::StringLiteral("This is a string".to_string())
        );
        assert_eq!(result[0].contents, "This is a string");
        assert_eq!(result[0].location, (0, 0).into());
    }

    #[test]
    fn string_literal_with_escaping() {
        let contents = r#""This is a string with \"escaping\" characters""#;
        let result = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].kind,
            TokenKind::StringLiteral("This is a string with \"escaping\" characters".to_string())
        );
        assert_eq!(
            result[0].contents,
            "This is a string with \"escaping\" characters"
        );
        assert_eq!(result[0].location, (0, 0).into());
    }

    #[test]
    fn string_literal_with_escaped_comment() {
        let contents = r#""This is a string with # escaped comment""#;
        let result = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].kind,
            TokenKind::StringLiteral("This is a string with # escaped comment".to_string())
        );
        assert_eq!(
            result[0].contents,
            "This is a string with # escaped comment"
        );
        assert_eq!(result[0].location, (0, 0).into());
    }

    #[test]
    fn read_single_symbol() {
        let symbols = vec![
            '+', '-', '*', '/', '=', '>', '<', '!', '?', '.', ',', ';', ':', '(', ')', '[', ']',
            '{', '}', '&', '|', '^', '%', '~',
        ];
        for symbol in symbols {
            let contents = format!("{}", symbol);
            let tokens = Tokenizer::tokenize(&contents, (0, 0).into()).unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].kind, TokenKind::Symbol(symbol.to_string()));
            assert_eq!(tokens[0].contents, symbol.to_string());
            assert_eq!(tokens[0].location, (0, 0).into());
        }
    }

    #[test]
    fn read_two_char_symbols() {
        let symbols = vec!["==", "!=", ">=", "<=", "->", "=>", "*=", "-=", "+=", "/="];
        for symbol in symbols {
            let contents = format!("{}", symbol);
            let tokens = Tokenizer::tokenize(&contents, (0, 0).into()).unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].kind, TokenKind::Symbol(symbol.to_string()));
            assert_eq!(tokens[0].contents, symbol);
            assert_eq!(tokens[0].location, (0, 0).into());
        }
    }

    #[test]
    fn read_identifier_with_symbol() {
        let symbols = vec![
            ".", ",", ";", ":", "(", ")", "[", "]", "{", "}", "->", "==", "!=", ">=", "<=", "->",
            "=>", "*=", "-=", "+=", "/=", "=>",
        ];

        for symbol in symbols {
            let contents = format!("my_variable{}my_variable2", symbol);
            let tokens = Tokenizer::tokenize(&contents, (0, 0).into()).unwrap();
            assert_eq!(tokens.len(), 3);
            assert_eq!(
                tokens[0].kind,
                TokenKind::Identifier("my_variable".to_string())
            );
            assert_eq!(tokens[0].contents, "my_variable");
            assert_eq!(tokens[0].location, (0, 0).into());
            assert_eq!(tokens[1].kind, TokenKind::Symbol(symbol.to_string()));
            assert_eq!(tokens[1].contents, symbol.to_string());
            assert_eq!(tokens[1].location, (0, 11).into());
            assert_eq!(
                tokens[2].kind,
                TokenKind::Identifier("my_variable2".to_string())
            );
            assert_eq!(tokens[2].contents, "my_variable2");
            assert_eq!(tokens[2].location, (0, 11 + symbol.len()).into());
        }
    }

    #[test]
    fn read_integer() {
        let contents = "12345";
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::IntegerLiteral(12345));
        assert_eq!(tokens[0].contents, "12345");
        assert_eq!(tokens[0].location, (0, 0).into());
    }

    #[test]
    fn read_float() {
        let contents = "12345.6789";
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::FloatLiteral(12345.6789));
        assert_eq!(tokens[0].contents, "12345.6789");
        assert_eq!(tokens[0].location, (0, 0).into());
    }

    #[test]
    fn read_float_with_negative() {
        let contents = "-12345.6789";
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::FloatLiteral(-12345.6789));
        assert_eq!(tokens[0].contents, "-12345.6789");
        assert_eq!(tokens[0].location, (0, 0).into());
    }

    #[test]
    fn read_float_with_multiple_decimals() {
        let contents = "-12345.6789.12345";
        let tokens = Tokenizer::tokenize(contents, (0, 0).into());
        assert!(tokens.is_err());
        assert_eq!(
            tokens.err().unwrap(),
            Error {
                message: "Float literal cannot have multiple decimal points".to_string(),
                location: (0, 11).into(),
            }
        );
    }

    #[test]
    fn read_integer_with_negative() {
        let contents = "-12345";
        let tokens = Tokenizer::tokenize(contents, (0, 0).into()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::IntegerLiteral(-12345));
        assert_eq!(tokens[0].contents, "-12345");
        assert_eq!(tokens[0].location, (0, 0).into());
    }

    #[test]
    fn is_numeric() {
        assert_eq!('0'.is_numeric(), true);
        assert_eq!('1'.is_numeric(), true);
        assert_eq!('2'.is_numeric(), true);
        assert_eq!('3'.is_numeric(), true);
        assert_eq!('4'.is_numeric(), true);
        assert_eq!('5'.is_numeric(), true);
        assert_eq!('6'.is_numeric(), true);
        assert_eq!('7'.is_numeric(), true);
        assert_eq!('8'.is_numeric(), true);
        assert_eq!('9'.is_numeric(), true);
        assert_eq!('0'.is_numeric(), true);
    }
}
