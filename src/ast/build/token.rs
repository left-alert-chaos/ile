//! # token
//! This private module is for initial parsing of a source file--it takes raw code and converts it into a
//! `Vector` of `Token`s which represent symbols and syntax. These symbols are then used to build an
//! AST. This is the first step of interpretation.
//!
//! This module is private because it shouldn't be necessary if you are implementing the interpreter
//! inside a preexisting application.

use crate::*;

use std::fmt;

/// # Token
/// This is a struct holding both a `TokenType` and a line number for debugging. It is one piece of
/// processed information.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub line: u64,
    pub file: Option<String>,
}

impl Token {
    pub fn from(value: String, line: u64, file: Option<String>) -> Result<Self, Error> {
        let ttype = TokenType::from(value)?;

        Ok(Self { ttype, line, file })
    }
}

/// # TokenType
/// This is an enum representing the type of a processed token of source code.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    /// A token for (
    OpenParen,

    /// A token for )
    CloseParen,

    /// A token for [
    OpenBracket,

    /// A token for ]
    CloseBracket,

    /// A token for {
    OpenBrace,

    /// A token for }
    CloseBrace,

    /// A token for a path separator or "dot"
    PathSeparator,

    /// A token for a comma (,)
    Comma,

    /// A token for the end of a statement, which is a semicolon (;)
    ChainEnd,

    /// A token for the assignment operator or single equals sign (=)
    Assignment,

    /// A token for the less than operator (<)
    LessThan,

    /// A token for the greater than operator (>)
    GreaterThan,

    /// A token for the addition operator (+)
    Addition,

    /// A token for the subtraction operator (-)
    Subtraction,

    /// A token for the multiplication operator, in this case an asterisk (*)
    Multiplication,

    /// A token for the division operator, in this case a forward-slash (/)
    Division,

    /// A token for less-than-or-equal-to comparisons (<=)
    LessThanOrEqualTo,

    /// A token for greater-than-or-equal-to comparisons (>=)
    GreaterThanOrEqualTo,

    /// A token for equality comparisons (==)
    Equality,

    /// A token for inequality comparisons (!=)
    NotEqualTo,

    /// A token for reversing a boolean value (!)
    Not,

    /// A token for a logical and gate (&&)
    And,

    /// A token for a logical or gate (||)
    Or,

    /// A token for string literals
    String(String),

    /// A token for integer literals
    Integer(i64),

    /// A token for float literals
    Float(f64),

    /// A token for booleans
    Boolean(bool),

    /// A token for anything that doesn't fit into any other bucket; it's usually object and type
    /// names.
    Word(String),
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // I've never seen a match statement this _stupid_ in my life, but here we are
        let phrase = match self {
            Self::OpenParen => "OpenParen".to_string(),
            Self::CloseParen => "CloseParen".to_string(),
            Self::OpenBracket => "OpenBracket".to_string(),
            Self::CloseBracket => "CloseBracket".to_string(),
            Self::OpenBrace => "OpenBrace".to_string(),
            Self::CloseBrace => "CloseBrace".to_string(),
            Self::PathSeparator => "PathSeparator (period)".to_string(),
            Self::Comma => "Comma".to_string(),
            Self::ChainEnd => "ChainEnd (semicolon)".to_string(),
            Self::Assignment => "Assignment (single equals sign)".to_string(),
            Self::LessThan => "LessThan".to_string(),
            Self::GreaterThan => "GreaterThan".to_string(),
            Self::Addition => "Addition".to_string(),
            Self::Subtraction => "Subtraction".to_string(),
            Self::Multiplication => "Multiplication".to_string(),
            Self::Division => "Division".to_string(),
            Self::LessThanOrEqualTo => "LessThanOrEqualTo".to_string(),
            Self::GreaterThanOrEqualTo => "GreaterThanOrEqualTo".to_string(),
            Self::Equality => "Equality (double equals sign)".to_string(),
            Self::NotEqualTo => "NotEqualTo".to_string(),
            Self::Not => "Not".to_string(),
            Self::And => "And".to_string(),
            Self::Or => "Or".to_string(),
            Self::String(s) => format!("String '{s}'"),
            Self::Integer(i) => format!("Integer {i}"),
            Self::Float(f) => format!("Float {f}"),
            Self::Boolean(b) => format!("Boolean {b}"),
            Self::Word(w) => format!("Word '{w}'"),
        };

        write!(f, "{phrase}")
    }
}

impl<'a> TokenType {
    /// Determine if the token is logical operator
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Addition
                | Self::Subtraction
                | Self::Multiplication
                | Self::Division
                | Self::Equality
                | Self::NotEqualTo
                | Self::LessThan
                | Self::GreaterThan
                | Self::LessThanOrEqualTo
                | Self::GreaterThanOrEqualTo
                | Self::And
                | Self::Or
        )
    }

    pub fn operator_priority(&self) -> u8 {
        match self {
            Self::GreaterThan
            | Self::LessThan
            | Self::GreaterThanOrEqualTo
            | Self::LessThanOrEqualTo => 1,
            Self::Multiplication | Self::Division => 2,
            Self::Addition | Self::Subtraction => 3,
            Self::And | Self::Or => 4,
            Self::Equality | Self::NotEqualTo => 5,
            _ => 0,
        }
    }

    /// Determine if two tokens have the same TokenType (ignoring inner values)
    pub fn same_as(&self, other: &TokenType) -> bool {
        // remove inner values
        let self_ref = match self {
            Self::String(_) => &Self::String(String::new()),
            Self::Integer(_) => &Self::Integer(0),
            Self::Float(_) => &Self::Float(0.0),
            Self::Boolean(_) => &Self::Boolean(false),
            Self::Word(_) => &Self::Word(String::new()),
            _ => self,
        };

        let other_ref = match other {
            Self::String(_) => &Self::String(String::new()),
            Self::Integer(_) => &Self::Integer(0),
            Self::Float(_) => &Self::Float(0.0),
            Self::Boolean(_) => &Self::Boolean(false),
            Self::Word(_) => &Self::Word(String::new()),
            _ => other,
        };

        self_ref == other_ref
    }

    // This took waaaay too long to write because of stupid lifetime errors.
    /// Check if two operator arms are the correct classifications to use this operator
    pub fn arms_are_correct(&self, arm1: &Object<'a>, arm2: &Object<'a>) -> bool {
        if self.operator_priority() == 4 {
            return arm1.boolean().is_some() && arm2.boolean().is_some();
        }

        match self {
            Self::Addition => {
                arm1.boolean().is_none()
                    && ((arm1.is_array() && arm2.is_array())
                        || (arm1.is_data() && arm2.is_data())
                        || (arm1.float().is_some() && arm2.float().is_some())
                        || (arm1.is_integer() && arm2.is_integer()))
                    || (arm1.is_string() && arm2.is_string())
            }
            _ => {
                (arm1.integer().is_some() && arm2.integer().is_some())
                    || (arm1.float().is_some() && arm2.float().is_some())
                    || (arm1.is_string() && arm2.is_string())
            }
        }
    }

    fn from(mut value: String) -> Result<Self, Error> {
        let len = value.len();

        // Determine if it's a single-character token
        if len == 1 {
            match value.chars().nth(0).unwrap() {
                '(' => return Ok(Self::OpenParen),
                ')' => return Ok(Self::CloseParen),
                '[' => return Ok(Self::OpenBracket),
                ']' => return Ok(Self::CloseBracket),
                '{' => return Ok(Self::OpenBrace),
                '}' => return Ok(Self::CloseBrace),
                '.' => return Ok(Self::PathSeparator),
                ',' => return Ok(Self::Comma),
                ';' => return Ok(Self::ChainEnd),
                '=' => return Ok(Self::Assignment),
                '<' => return Ok(Self::LessThan),
                '>' => return Ok(Self::GreaterThan),
                '+' => return Ok(Self::Addition),
                '-' => return Ok(Self::Subtraction),
                '*' => return Ok(Self::Multiplication),
                '/' => return Ok(Self::Division),
                '!' => return Ok(Self::Not),
                _ => {}
            }
        }

        // various keywords
        match value.as_str() {
            ">=" => return Ok(Self::GreaterThanOrEqualTo),
            "<=" => return Ok(Self::LessThanOrEqualTo),
            "==" => return Ok(Self::Equality),
            "!=" => return Ok(Self::NotEqualTo),
            "&&" => return Ok(Self::And),
            "||" => return Ok(Self::Or),
            "false" => return Ok(Self::Boolean(false)),
            "true" => return Ok(Self::Boolean(true)),
            _ => {}
        }

        if value.starts_with('"') && value.ends_with('"') {
            // check validity
            if len < 2 {
                return Err(Error {
                    message: String::from(
                        "invalid string literal due to odd number of quotation marks",
                    ),
                    location: PipelineLocation::Tokenization,
                    token: None,
                });
            }

            // Remove quotes
            value.remove(0);
            value.remove(len - 2);
            Ok(Self::String(value))
        } else if let Ok(int) = value.parse::<i64>() {
            Ok(Self::Integer(int))
        } else if let Ok(float) = value.parse::<f64>() {
            Ok(Self::Float(float))
        } else {
            Ok(Self::Word(value))
        }
    }
}

/// # tokenize
/// This function processes a piece of code into a `Vec<TokenType>` object. This is the first step of
/// interpretation.
pub fn tokenize(code: impl ToString, file: Option<String>) -> Result<Vec<Token>, Error> {
    let code = code.to_string();
    let mut tokens = Vec::new();

    let mut buffer = String::new();
    let mut string = false;
    let mut previous = ' ';
    let mut line = 1;
    let mut comment = false;

    // Closure to run when all characters of a token have been read
    let mut finish_token = |b: &mut String, s: bool, line: u64| {
        // Skip if no buffer to tokenize
        if b.is_empty() || s {
            return Ok(());
        }

        // Attempt to tokenize buffer
        match Token::from(b.clone(), line, file.clone()) {
            Ok(token) => tokens.push(token),
            Err(reason) => {
                return Err(reason);
            }
        }
        b.clear();
        Ok(())
    };

    for (index, character) in code.chars().enumerate() {
        if comment && character != '\n' {
            continue;
        }

        // This match statement is ugly and gross
        match character {
            // single-character tokens are processed by sending complete previous token and then
            // sending them by themselves
            ';' | '(' | ')' | '[' | ']' | '{' | '}' | '+' | '*' | '/' | ',' => {
                finish_token(&mut buffer, string, line)?;
                buffer.push(character);
                finish_token(&mut buffer, string, line)?;
            }
            // minus can be a subtraction or a negative number
            '-' => {
                // raise if you can't tell what it does
                if index + 1 >= code.len() {
                    return Err(Error {
                        message: String::from("EOF before meaning of minus sign can be determined"),
                        location: PipelineLocation::Tokenization,
                        token: None,
                    });
                }

                let next = code.chars().nth(index + 1).unwrap();

                if next.is_ascii_digit() {
                    buffer.push('-');
                } else {
                    finish_token(&mut buffer, string, line)?;
                    buffer.push('-');
                    finish_token(&mut buffer, string, line)?;
                }
            }
            // Dot is similar to other single-character tokens, but has to check if it's being used
            // in a float first
            '.' => {
                if let Ok(_num) = buffer.parse::<i64>() {
                    buffer.push('.');
                } else {
                    // if it's not a number, create a new token
                    finish_token(&mut buffer, string, line)?;
                    buffer.push('.');
                    finish_token(&mut buffer, string, line)?;
                }
            }
            // Equality only processed as its own token if the previous character wasn't special.
            '=' => {
                match previous {
                    '>' | '<' | '!' | '=' => {
                        buffer.push('=');
                    }
                    _ => {
                        if let Some(next) = code.chars().nth(index + 1) {
                            // if the next character doesn't make it a double equals, finish the
                            // previous token. The logic of pushing an equals sign and finishing is
                            // the same regardless.
                            // I wrote this at 11 PM, so this might get replaced at some point
                            if next != '=' {
                                finish_token(&mut buffer, string, line)?;
                                buffer.push('=');
                                finish_token(&mut buffer, string, line)?;
                            } else {
                                finish_token(&mut buffer, string, line)?;
                                buffer.push('=');
                            }
                        }
                    }
                }
            }
            // Similar to the =, the ! is only processed by itself if the next character isn't =.
            '!' | '>' | '<' => {
                // process as its own token
                if Some('=') != code.chars().nth(index + 1) {
                    finish_token(&mut buffer, string, line)?;
                    buffer.push(character);
                    finish_token(&mut buffer, string, line)?;
                } else {
                    // if it's not its own thing, add to buffer
                    buffer.push(character);
                }
            }
            // process string literals
            '"' => {
                string = !string;
                buffer.push('"');
                if !string {
                    finish_token(&mut buffer, string, line)?;
                }
            }
            ' ' | '\t' if !string => {
                finish_token(&mut buffer, string, line)?;
            }
            '\n' => {
                comment = false;
                if !string {
                    finish_token(&mut buffer, string, line)?;
                } else {
                    buffer.push('\n');
                }
                line += 1;
            }
            '#' => {
                if !string {
                    comment = true;
                }
            }
            _ => {
                if !comment {
                    buffer.push(character);
                }
            }
        }

        previous = character;
    }

    // flush buffer in case there's a trailing token
    finish_token(&mut buffer, string, line)?;

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to extract TokenType
    fn token_types(code: impl ToString) -> Result<Vec<TokenType>, String> {
        let tokens = tokenize(code, None)?;
        let mut types = Vec::new();
        for token in tokens {
            types.push(token.ttype);
        }
        Ok(types)
    }

    #[test]
    fn proper_line() {
        let token = tokenize(
            "

        15
            ",
            None,
        )
        .unwrap();
        assert_eq!(token[0].line, 3);
    }

    #[test]
    fn float() {
        let token = tokenize("3.1415926535", None).unwrap()[0].clone();
        assert_eq!(token.ttype, TokenType::Float(3.1415926535));
    }

    #[test]
    fn path() {
        let tokens = token_types("obj.attr.attr").unwrap();
        let expected = Vec::from([
            TokenType::Word(String::from("obj")),
            TokenType::PathSeparator,
            TokenType::Word(String::from("attr")),
            TokenType::PathSeparator,
            TokenType::Word(String::from("attr")),
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn comma() {
        let token = token_types(",").unwrap()[0].clone();
        assert_eq!(token, TokenType::Comma);
    }

    #[test]
    fn comment() {
        let tokens = tokenize(
            "#this line is commented out\n15 #the rest of this line is also commented out",
            None,
        )
        .unwrap();
        assert_eq!(tokens.len(), 1);
    }
}
