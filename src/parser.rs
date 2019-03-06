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

/* Helper factory functions for building ASTs */
pub fn ast_alternation(lhs: AST, rhs: AST) -> AST {
    AST::Alternation(Box::new(lhs), Box::new(rhs))
}

pub fn ast_catenation(lhs: AST, rhs: AST) -> AST {
    AST::Catenation(Box::new(lhs), Box::new(rhs))
}

pub fn ast_closure(val: AST) -> AST {
    AST::Closure(Box::new(val))
}

pub fn ast_char(c: char) -> AST {
    AST::Char(c)
}

pub fn ast_any_char() -> AST {
    AST::AnyChar
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
        let expression = self.catenation()?;

        // 0 or 1 (UnionBar <RegExp>)
        // peek because going to take later on in other methods
        if let Some(t) = self.tokens.peek() {
            match t {
                Token::UnionBar => {
                    self.take_next_token()?;
                    let rhs = self.reg_expr()?;
                    Ok(ast_alternation(expression, rhs))
                },
                _ => Ok(expression),
            }
        } else {
            Ok(expression)
        }
    }
    
    // Atom -> LParen <RegExpr> RParen | AnyChar | Char
    fn atom(&mut self) -> Result<AST, String> {
        let t = self.take_next_token()?;

        // if AnyChar or Char, return an Ok of that enum variant
        // if LParen, get RegExpr coming up then get the RParen too
        // and return Ok of that RegExpr
        // otherwise error
        match t {
            Token::AnyChar => self.handle_any_char(),
            Token::Char(c) => self.handle_char(c),
            Token::LParen => self.handle_parens(),
            _ => Err(format!("Unexpected token: {:?}", t)),
        }
    }

    fn handle_any_char(&mut self) -> Result<AST, String> {
        Ok(ast_any_char())
    }

    fn handle_char(&mut self, c: char) -> Result<AST, String> {
        Ok(ast_char(c))
    }

    fn handle_parens(&mut self) -> Result<AST, String> {
        // get expression coming after lparen
        // if not valid, propagate error up
        let express = self.reg_expr()?;

        // after getting lparen and expression, consume rparen or if that's not next
        // then propagate the corresponding error
        self.consume_token(Token::RParen)?;
        Ok(express)
    }

    fn closure(&mut self) -> Result<AST, String> {
        let atm = self.atom()?;
        if let Some(c) = self.tokens.peek() {
            match c {
                Token::KleeneStar => {
                    self.take_next_token()?;
                    Ok(ast_closure(atm))
                },
                _ => Ok(atm),
            }
        } else {
            Ok(atm)
        }
    }

    fn catenation(&mut self) -> Result<AST, String> {
        let clos = self.closure()?;
        if let Some(t) = self.tokens.peek() {
            match t {
                Token::LParen | Token::AnyChar | Token::Char(_) => Ok(ast_catenation(clos, self.catenation()?)),
                _ => Ok(clos), 
            }
        } else {
            Ok(clos)
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

    mod lvl1 {
        use super::*;

        #[test]
        fn closure_atom() {
            assert_eq!(Parser::from("a").closure().unwrap(), ast_char('a'));
        }

        #[test]
        fn closure() {
            assert_eq!(Parser::from("b*").closure().unwrap(), ast_closure(ast_char('b')));
        }

        #[test]
        fn closure_parents() {
            assert_eq!(Parser::from("(a)*").closure().unwrap(), ast_closure(ast_char('a')));
        }

        // add tests with concatenation and closure combined
    }

    mod lvl2 {
        use super::*;

        #[test]
        fn catenation_atom() {
            assert_eq!(Parser::from("a").catenation().unwrap(), ast_char('a'));
        }

        #[test]
        fn catenation_closure() {
            assert_eq!(Parser::from("a*").catenation().unwrap(), ast_closure(ast_char('a')));
        }

        #[test]
        fn catenation() {
            assert_eq!(Parser::from("ab*").catenation().unwrap(), ast_catenation(ast_char('a'), ast_closure(ast_char('b'))));
        }

        #[test]
        fn catenation_parens() {
            assert_eq!(Parser::from("(ab)*").catenation().unwrap(), ast_closure(ast_catenation(ast_char('a'), ast_char('b'))));
        }
    }

    mod lvl3 {
        use super::*;

        #[test]
        fn reg_expr_atom() {
            assert_eq!(Parser::from("a").reg_expr().unwrap(), ast_char('a'));
        }

        #[test]
        fn reg_expr_cat() {
            assert_eq!(Parser::from("ab").reg_expr().unwrap(), ast_catenation(ast_char('a'), ast_char('b')));
        }

        #[test]
        fn reg_expr_cat_closure() {
            assert_eq!(Parser::from("ab*").reg_expr().unwrap(), ast_catenation(ast_char('a'), ast_closure(ast_char('b'))));
        }

        #[test]
        fn reg_expr_alternation() {
            assert_eq!(Parser::from("a|b").reg_expr().unwrap(), ast_alternation(ast_char('a'), ast_char('b')));
        }

        #[test]
        fn reg_expr_any_star() {
            assert_eq!(Parser::from(".*").reg_expr().unwrap(), ast_closure(ast_any_char()));
        }

        #[test]
        fn reg_expr_all() {
            assert_eq!(Parser::from("(a|b.)*").reg_expr().unwrap(), ast_closure(ast_alternation(ast_char('a'), ast_catenation(ast_char('b'), ast_any_char()))));
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

