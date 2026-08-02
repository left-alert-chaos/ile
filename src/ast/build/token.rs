//! # token
//! This private module is for initial parsing of a source file--it takes raw code and converts it into a
//! `Vector` of `Token`s which represent symbols and syntax. These symbols are then used to build an
//! AST. This is the first step of interpretation.
//!
//! This module is private because it shouldn't be necessary if you are implementing the interpreter
//! inside a preexisting application.

/// # Token
/// This is a struct holding both a `TokenType` and a line number for debugging. It is one piece of
/// processed information.
#[derive(Debug, PartialEq)]
pub struct Token {
    ttype: TokenType,
    line: u64,
}

impl Token {
    pub fn from(value: String, line: u64) -> Result<Self, String> {
        let ttype = TokenType::from(value)?;

        Ok(Self {
            ttype,
            line
        })
    }
}

/// # TokenType
/// This is an enum representing the type of a processed token of source code.
#[derive(Debug, PartialEq)]
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

    /// A token for string literals
    String(String),

    /// A token for integer literals
    Integer(i64),

    /// A token for float literals
    Float(f64),

    /// A token for booleans
    Boolean(bool),

    /// A token for the `datatype` keyword
    Datatype,

    /// A token for anything that doesn't fit into any other bucket; it's usually object and type
    /// names.
    Word(String),
}

impl TokenType {
    fn from(mut value: String) -> Result<Self, String> {
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
            "false" => return Ok(Self::Boolean(false)),
            "true" => return Ok(Self::Boolean(true)),
            "datatype" => return Ok(Self::Datatype),
            _ => {}
        }

        if value.starts_with('"') && value.ends_with('"') {
            // check validity
            if len < 2 {
                return Err(String::from("invalid string literal due to odd number of quotation marks"));
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
pub fn tokenize(code: impl ToString) -> Result<Vec<Token>, String> {
    let code = code.to_string();
    let mut tokens = Vec::new();

    let mut buffer = String::new();
    let mut string = false;
    let mut previous = ' ';
    let mut line = 1;

    // Closure to run when all characters of a token have been read
    let mut finish_token = |b: &mut String, s: bool, line: u64| {
        // Skip if no buffer to tokenize
        if b.is_empty() || s {
            return Ok(());
        }

        // Attempt to tokenize buffer
        match Token::from(b.clone(), line) {
            Ok(token) => {println!("Created token {token:?} from buffer {b}"); tokens.push(token)},
            Err(reason) => return Err(reason)
        }
        b.clear();
        Ok(())
    };
    
    for (index, character) in code.chars().enumerate() {
        // This match statement is ugly and gross
        match character {
            // single-character tokens are processed by sending complete previous token and then
            // sending them by themselves
            ';' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '+' | '-' | '*' | '/'  => {
                finish_token(&mut buffer, string, line)?;
                buffer.push(character);
                finish_token(&mut buffer, string, line)?;
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
            '!' => {
                // process as its own token
                if Some('=') != code.chars().nth(index + 1) {
                    finish_token(&mut buffer, string, line)?;
                    buffer.push('!');
                    finish_token(&mut buffer, string, line)?;
                } else {
                    // if it's not its own thing, add to buffer
                    buffer.push('!');
                }
            }
            // process string literals
            '"' => {
                string = !string;
                if !string {
                    finish_token(&mut buffer, string, line)?;
                }
            }
            ' ' | '\t'
                if !string => {
                    finish_token(&mut buffer, string, line)?;
                }
            '\n' => {
                line += 1;
                if !string {
                    finish_token(&mut buffer, string, line)?;
                } else {
                    buffer.push('\n');
                }
            }
            _ => {
                buffer.push(character);
            }
        }

        previous = character;
    }

    // flush buffer in case there's a trailing token
    finish_token(&mut buffer, string, line)?;

    Ok(tokens)
}
