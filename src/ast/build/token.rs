//! # token
//! This private module is for initial parsing of a source file--it takes raw code and converts it into a
//! `Vector` of `Token`s which represent symbols and syntax. These symbols are then used to build an
//! AST. This is the first step of interpretation.
//!
//! This module is private because it shouldn't be necessary if you are implementing the interpreter
//! inside a preexisting application.

/// # Token
/// This is an enum representing a processed token of source code.
#[derive(Debug, PartialEq)]
pub enum Token {
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

    /// A token for string literals
    String(String),

    /// A token for integer literals
    Integer(i64),

    /// A token for float literals
    Float(f64),

    /// A token for anything that doesn't fit into any other bucket; it's usually object and type
    /// names.
    Word(String),
}

impl Token {
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
                _ => {}
            }
        } else if len == 2 {
            match value.chars().as_str() {
                ">=" => return Ok(Self::GreaterThanOrEqualTo),
                "<=" => return Ok(Self::LessThanOrEqualTo),
                "==" => return Ok(Self::Equality),
                "!=" => return Ok(Self::NotEqualTo),
                _ => {}
            }
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
/// This function processes a piece of code into a `Vec<Token>` object. This is the first step of
/// interpretation.
pub fn tokenize(code: impl ToString) -> Result<Vec<Token>, String> {
    let code = code.to_string();
    let mut tokens = Vec::new();

    let mut buffer = String::new();
    let mut string = false;
    let mut previous = ' ';

    // Closure to run when all characters of a token have been read
    let mut finish_token = |b: &mut String, s: bool| {
        // Skip if no buffer to tokenize
        if b.len() == 0 || s {
            return Ok(());
        }

        // Attempt to tokenize buffer
        match Token::from(b.clone()) {
            Ok(token) => {println!("Created token {token:?} from buffer {b}"); tokens.push(token)},
            Err(reason) => return Err(reason)
        }
        b.clear();
        Ok(())
    };
    
    for (index, character) in code.chars().enumerate() {
        match character {
            // single-character tokens are processed by sending complete previous token and then
            // sending them by themselves
            ';' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '+' | '-' | '*' | '/'  => {
                finish_token(&mut buffer, string)?;
                buffer.push(character);
                finish_token(&mut buffer, string)?;
            }
            // Dot is similar to other single-character tokens, but has to check if it's being used
            // in a float first
            '.' => {
                if let Ok(_num) = buffer.parse::<i64>() {
                    buffer.push('.');
                } else {
                    // if it's not a number, create a new token
                    finish_token(&mut buffer, string)?;
                    buffer.push('.');
                    finish_token(&mut buffer, string)?;
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
                                finish_token(&mut buffer, string)?;
                                buffer.push('=');
                                finish_token(&mut buffer, string)?;
                            } else {
                                finish_token(&mut buffer, string)?;
                                buffer.push('=');
                            }
                        }
                    }
                }
            }
            // process string literals
            '"' => {
                string = !string;
                if !string {
                    finish_token(&mut buffer, string)?;
                }
            }
            ' ' | '\n' | '\t' => {
                if !string {
                    finish_token(&mut buffer, string)?;
                } else {
                    buffer.push(character);
                }
            }
            _ => {
                buffer.push(character);
            }
        }

        previous = character;
    }

    // flush buffer in case there's a trailing token
    println!("Flushing buffer");
    finish_token(&mut buffer, string)?;

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_int_set() {
        let result = tokenize("Integer x = 15;");
        let expected = Vec::from([Token::Word(String::from("Integer")), Token::Word(String::from("x")), Token::Assignment, Token::Integer(15), Token::ChainEnd]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn tokenize_float_set() {
        let result = tokenize("Float var_name = 3.14159;");
        let expected = Vec::from([Token::Word(String::from("Float")), Token::Word(String::from("var_name")), Token::Assignment, Token::Float(3.14159), Token::ChainEnd]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn tokenize_path_separator() {
        let result = tokenize("Bool var_name = other_var.boolean_attribute_i_guess;");
        let expected = Vec::from([Token::Word(String::from("Bool")), Token::Word(String::from("var_name")), Token::Assignment, Token::Word(String::from("other_var")), Token::PathSeparator, Token::Word(String::from("boolean_attribute_i_guess")), Token::ChainEnd]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn tokenize_inequality() {
        let result = tokenize("!=");
        let expected = Vec::from([Token::NotEqualTo]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn tokenize_equality_check() {
        let result = tokenize("==;");
        let expected = Vec::from([Token::Equality, Token::ChainEnd]);
        assert_eq!(result.unwrap(), expected);
    }
}
