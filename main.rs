use std::io::{self, Write};
 
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}
 
// Takes a borrowed &str, returns an owned Vec<Token> the caller will own.
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
 
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num_str.parse().unwrap_or(0.0)));
            }
            _ => {
                chars.next();
            }
        }
    }
    tokens
}
 
/// A parser struct holding the tokens and a mutable position cursor.
/// `tokens` is owned by the parser for its lifetime.
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
 
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }
 
    // Borrows self immutably to look at the current token.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
 
    // Borrows self mutably to advance the cursor.
    fn advance(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }
 
    // expr := term (('+' | '-') term)*
    fn parse_expr(&mut self) -> f64 {
        let mut result = self.parse_term();
 
        while let Some(op) = self.peek() {
            match op {
                Token::Plus => {
                    self.advance();
                    result += self.parse_term();
                }
                Token::Minus => {
                    self.advance();
                    result -= self.parse_term();
                }
                _ => break,
            }
        }
        result
    }
 
    // term := factor (('*' | '/') factor)*
    fn parse_term(&mut self) -> f64 {
        let mut result = self.parse_factor();
 
        while let Some(op) = self.peek() {
            match op {
                Token::Star => {
                    self.advance();
                    result *= self.parse_factor();
                }
                Token::Slash => {
                    self.advance();
                    let divisor = self.parse_factor();
                    result = if divisor != 0.0 {
                        result / divisor
                    } else {
                        eprintln!("Warning: division by zero, returning 0");
                        0.0
                    };
                }
                _ => break,
            }
        }
        result
    }
 
    // factor := NUMBER | '(' expr ')' | '-' factor
    fn parse_factor(&mut self) -> f64 {
        match self.advance() {
            Some(Token::Number(n)) => n,
            Some(Token::LParen) => {
                let result = self.parse_expr();
                self.advance(); // consume ')'
                result
            }
            Some(Token::Minus) => -self.parse_factor(), // unary minus, recursive
            other => {
                eprintln!("Unexpected token: {:?}", other);
                0.0
            }
        }
    }
}
 
// Owns its input Vec<Token>, since evaluation consumes the tokens.
fn evaluate(tokens: Vec<Token>) -> f64 {
    let mut parser = Parser::new(tokens);
    parser.parse_expr()
}
 
fn run_calculator(input: &str) -> f64 {
    let tokens = tokenize(input);
    evaluate(tokens)
}
 
fn main() {
    println!("Rust Text Calculator (supports + - * / and parentheses)");
    println!("Type an expression, or 'quit' to exit.\n");
 
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
 
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
 
        let input = input.trim();
 
        match input {
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "" => continue,
            _ => {
                let result = run_calculator(input);
                println!("= {}", result);
            }
        }
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_precedence() {
        assert_eq!(run_calculator("3 + 4 * 2"), 11.0);
    }
 
    #[test]
    fn test_parens() {
        assert_eq!(run_calculator("(3 + 4) * 2"), 14.0);
    }
 
    #[test]
    fn test_unary_minus() {
        assert_eq!(run_calculator("-5 + 3"), -2.0);
    }
 
    #[test]
    fn test_division() {
        assert_eq!(run_calculator("10 / 2 / 5"), 1.0);
    }
}
 