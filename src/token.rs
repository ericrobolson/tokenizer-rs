use std::fmt::Display;

use crate::{error::Error, location::Location};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub location: Location,
    pub contents: String,
    pub kind: TokenKind,
}
impl Token {
    pub fn assert_string(&self, msg: &str) -> Result<String, Error> {
        if let TokenKind::StringLiteral(s) = &self.kind {
            Ok(s.clone())
        } else {
            Err(Error {
                location: self.location.clone(),
                message: format!("Expected {}, got {}", msg, self.kind),
            })
        }
    }

    pub fn assert_comment(&self, msg: &str) -> Result<String, Error> {
        if let TokenKind::Comment(s) = &self.kind {
            Ok(s.clone())
        } else {
            Err(Error {
                location: self.location.clone(),
                message: format!("Expected {}, got {}", msg, self.kind),
            })
        }
    }

    pub fn assert_identifier(&self, msg: &str) -> Result<String, Error> {
        if let TokenKind::Identifier(s) = &self.kind {
            Ok(s.clone())
        } else {
            Err(Error {
                location: self.location.clone(),
                message: format!("Expected {}, got {}", msg, self.kind),
            })
        }
    }

    pub fn assert_symbol(&self, msg: &str) -> Result<String, Error> {
        if let TokenKind::Symbol(s) = &self.kind {
            Ok(s.clone())
        } else {
            Err(Error {
                location: self.location.clone(),
                message: format!("Expected {}, got {}", msg, self.kind),
            })
        }
    }

    pub fn assert_int(&self, msg: &str) -> Result<i128, Error> {
        if let TokenKind::IntegerLiteral(i) = &self.kind {
            Ok(i.clone())
        } else {
            Err(Error {
                location: self.location.clone(),
                message: format!("Expected {}, got {}", msg, self.kind),
            })
        }
    }

    pub fn assert_float(&self, msg: &str) -> Result<f64, Error> {
        if let TokenKind::FloatLiteral(f) = &self.kind {
            Ok(f.clone())
        } else {
            Err(Error {
                location: self.location.clone(),
                message: format!("Expected {}, got {}", msg, self.kind),
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    StringLiteral(String),
    Comment(String),
    Identifier(String),
    Symbol(String),
    IntegerLiteral(i128),
    FloatLiteral(f64),
}
impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::StringLiteral(s) => write!(f, "string \"{}\"", s),
            TokenKind::Comment(s) => write!(f, "comment \"{}\"", s),
            TokenKind::Identifier(s) => write!(f, "identifier '{}'", s),
            TokenKind::Symbol(s) => write!(f, "symbol '{}'", s),
            TokenKind::IntegerLiteral(i) => write!(f, "int '{}'", i),
            TokenKind::FloatLiteral(float) => write!(f, "float '{}'", float),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_string_works() {
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::StringLiteral("jaja".to_string()),
        };

        let expected = Ok("jaja".to_string());
        let actual = token.assert_string("msg");
        assert_eq!(expected, actual);

        // Test with wrong type
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::Identifier("jaja".to_string()),
        };

        let expected = Err(Error {
            location: (0, 0).into(),
            message: "Expected msg, got identifier 'jaja'".to_string(),
        });
        let actual = token.assert_string("msg");
        assert_eq!(expected, actual);
    }

    #[test]
    fn assert_comment_works() {
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::Comment("jaja".to_string()),
        };

        let expected = Ok("jaja".to_string());
        let actual = token.assert_comment("msg");
        assert_eq!(expected, actual);

        // Test with wrong type
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::Identifier("jaja".to_string()),
        };

        let expected = Err(Error {
            location: (0, 0).into(),
            message: "Expected msg, got identifier 'jaja'".to_string(),
        });
        let actual = token.assert_comment("msg");
        assert_eq!(expected, actual);
    }

    #[test]
    fn assert_identifier_works() {
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::Identifier("jaja".to_string()),
        };

        let expected = Ok("jaja".to_string());
        let actual = token.assert_identifier("msg");
        assert_eq!(expected, actual);

        // Test with wrong type
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::StringLiteral("jaja".to_string()),
        };

        let expected = Err(Error {
            location: (0, 0).into(),
            message: "Expected msg, got string \"jaja\"".to_string(),
        });
        let actual = token.assert_identifier("msg");
        assert_eq!(expected, actual);
    }

    #[test]
    fn assert_symbol_works() {
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::Symbol("jaja".to_string()),
        };

        let expected = Ok("jaja".to_string());
        let actual = token.assert_symbol("msg");
        assert_eq!(expected, actual);

        // Test with wrong type
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::StringLiteral("jaja".to_string()),
        };

        let expected = Err(Error {
            location: (0, 0).into(),
            message: "Expected msg, got string \"jaja\"".to_string(),
        });
        let actual = token.assert_symbol("msg");
        assert_eq!(expected, actual);
    }

    #[test]
    fn assert_int_works() {
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::IntegerLiteral(123),
        };

        let expected = Ok(123);
        let actual = token.assert_int("msg");
        assert_eq!(expected, actual);

        // Test with wrong type
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::StringLiteral("jaja".to_string()),
        };

        let expected = Err(Error {
            location: (0, 0).into(),
            message: "Expected msg, got string \"jaja\"".to_string(),
        });
        let actual = token.assert_int("msg");
        assert_eq!(expected, actual);
    }

    #[test]
    fn assert_float_works() {
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::FloatLiteral(123.0),
        };

        let expected = Ok(123.0);
        let actual = token.assert_float("msg");
        assert_eq!(expected, actual);

        // Test with wrong type
        let token = Token {
            location: (0, 0).into(),
            contents: "jaja".to_string(),
            kind: TokenKind::StringLiteral("jaja".to_string()),
        };

        let expected = Err(Error {
            location: (0, 0).into(),
            message: "Expected msg, got string \"jaja\"".to_string(),
        });
        let actual = token.assert_float("msg");
        assert_eq!(expected, actual);
    }
}
