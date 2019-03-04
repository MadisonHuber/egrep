#[allow(unused)]

use super::tokenizer::{Token, Tokenizer};
use std::iter::Peekable;

/**
 * thegrep - Tar Heel egrep
 * 
 * Author(s): Alana Fiordalisi, Madison Huber
 * ONYEN(s): fiordali, hubermm
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff and partner.
 */

/* == Begin Syntax Tree Elements == */
#[derive(Debug, PartialEq)]
pub enum AST {
    Alternation(Box<AST>, Box<AST>),
    Catenation(Box<AST>, Box<AST>),
    Closure(Box<AST>),
    Char(char),
    AnyChar,
}

/* == End Syntax Tree Elements == */

pub struct Parser<'tokens> {
    tokens: Peekable<Tokenizer<'tokens>>,
}

impl<'tokens> Parser<'tokens> {
    pub fn parse(tokenizer: Tokenizer<'tokens>) -> Result<AST, String> {
        let mut parser = Parser {
            tokens: tokenizer.peekable(),
        };

        // Parse, and then ensure that all tokens in the expr were used.
        // Extra tokens causes error, else the structure (usually binop tree) is returned.
        let result = parser.reg_expr();
        if let Some(c) = parser.tokens.next() {
            Err(format!("Expected end of input, found {:?}", c))
        } else {
            result
        }
    }
}

/**
 * Internal-only parser methods to process grammar through recursive descent.
 */

impl<'tokens> Parser<'tokens> {
    // RegExpr -> atom
    fn reg_expr(&mut self) -> Result<AST, String> {
        self.atom()
    }
    
    // Atom -> LParen <RegExpr> RParen | AnyChar | Char
    fn atom(&mut self) -> Result<AST, String> {
        let t = self.take_next_token()?;

        // if AnyChar or Char, return an Ok of that enum variant
        // if LParen, get RegExpr coming up then get the RParen too
        // and return Ok of that RegExpr
        // otherwise error
        match t {
            Token::AnyChar => Ok(AST::AnyChar),
            Token::Char(c) => Ok(AST::Char(c)),
            Token::LParen => {
                let expr = self.reg_expr()?;
                self.consume_token(Token::RParen)?;
                Ok(expr)
            },
            _ => Err(format!("Unexpected token: {:?}", t)),
        }
    }
}

/**
 * Tests for helper methods.
 */

#[cfg(test)]
mod private_api {
    use super::*;

    mod lvl0 {
        use super::*;

        #[test]
        fn atom_anychar() {
            assert_eq!(Parser::from(".").atom().unwrap(), AST::AnyChar);
        }

        #[test]
        fn atom_char() {
            assert_eq!(Parser::from("a").atom().unwrap(), AST::Char('a'));
        }

        #[test]
        fn atom_parens_anychar() {
            assert_eq!(Parser::from("(.)").atom().unwrap(), AST::AnyChar);
        }

        #[test]
        fn atom_parens_char() {
            assert_eq!(Parser::from("(h)").atom().unwrap(), AST::Char('h'));
        }

        #[test]
        fn atom_nested_parens() {
            assert_eq!(Parser::from("((.))").atom().unwrap(), AST::AnyChar);
        }

        #[test]
        fn atom_parens_err() {
            assert_eq!(Parser::from("(").atom(), Err(format!("Unexpected end of input")));
            assert_eq!(Parser::from("()").atom(), Err(format!("Unexpected token: {:?}", Token::RParen)));
        }
    }

}

/* Parser's Helper Methods to improve ergonomics of parsing */
impl<'tokens> Parser<'tokens> {
    /**
     * Static helper method used in unit tests to establish a
     * parser given a string.
     */
    fn from(input: &'tokens str) -> Parser<'tokens> {
        Parser {
            tokens: Tokenizer::new(input).peekable(),
        }
    }

    /**
     * When you expect another token and want to take it directly
     * or raise an error that you expected another token here but
     * found the end of input. Example usage:
     *
     * let t: Token = self.take_next_token()?;
     *
     * Notice the ? usage will automatically propagate the Err or
     * unwrap the value of Ok.
     */
    fn take_next_token(&mut self) -> Result<Token, String> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }

    /**
     * When there's a specific token you expect next in the grammar
     * use this helper method. It will raise an Err if there is no
     * next token or if it is not _exactly_ the Token you expected
     * next. If it is the token you expected, it will return Ok(Token).
     */
    fn consume_token(&mut self, expected: Token) -> Result<Token, String> {
        if let Some(next) = self.tokens.next() {
            if next != expected {
                Err(format!("Expected: {:?} - Found {:?}", expected, next))
            } else {
                Ok(next)
            }
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }
}

