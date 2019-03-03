#[allow(unused)]
// remove this line later

use std::iter::Peekable;
use std::str::Chars;

/**
 * thbc - Tar Heel Basic Calculator
 *
 * Author(s): Alana Fiordalisi, Madison Huber
 * ONYEN(s): fiordali, hubermm
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff and partner.
 */

/**
 *
 * thegrep - Tar Heel Extended Global Regular Expressions Print
 *
 */

/**
 * The tokens types of 'thegrep' are defined below.
 */

#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    UnionBar,
    KleeneStar,
    AnyChar,
    Char(char),
}

/**
 * The internal state of a Tokenizer is maintained by a peekable character
 * iterator over a &str's Chars.
 */
pub struct Tokenizer<'str> {
    chars: Peekable<Chars<'str>>,
}

impl<'str> Tokenizer<'str> {
    pub fn new(input: &'str str) -> Tokenizer {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }
}

/**
 * The Iterator trait is implemented for Tokenizer. It will produce items of 
 * type Token and has a `next` method that returns Option<Token>.
 */
impl<'str> Iterator for Tokenizer<'str> {
    type Item = Token;

    /**
     * The `next` method ignores leading whitespace and returns the next
     * complete Some(Token) in the Tokenizer's input string or None at all.
     */
    fn next(&mut self) -> Option<Token> {
        Some(self.take_paren())
        // continue implementing
    }
}

impl<'str> Tokenizer<'str> {
    fn take_paren(&mut self) -> Token  {
        let c = self.chars.next().unwrap();
        match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            _ => panic!("Not a parenthesis"),
        }
    }

    fn take_union_bar(&mut self) -> Token {
        self.chars.next();
        Token::UnionBar
    }

    fn take_kleene_star(&mut self) -> Token {
        self.chars.next();
        Token::KleeneStar
    }

    fn take_any_char(&mut self) -> Token {
        Token::AnyChar
    }
}

/**
 * Unit tests for helper methods.
 */

#[cfg(test)]
mod helper_method {
    use super::*;
    
    #[test]
    fn take_lparen() {
        let mut tokens = Tokenizer::new("(");
        assert_eq!(tokens.take_paren(), Token::LParen);
        assert_eq!(tokens.chars.next(), None);
    }

    #[test]
    fn take_rparen() {
        let mut tokens = Tokenizer::new(")");
        assert_eq!(tokens.take_paren(), Token::RParen);
        assert_eq!(tokens.chars.next(), None);
    }

    #[test]
    fn take_union_bar() {
        let mut tokens = Tokenizer::new("|");
        assert_eq!(tokens.take_union_bar(), Token::UnionBar);
        assert_eq!(tokens.chars.next(), None);
    }

    #[test]
    fn take_kleene_star() {
        let mut tokens = Tokenizer::new("*");
        assert_eq!(tokens.take_kleene_star(), Token::KleeneStar);
        assert_eq!(tokens.chars.next(), None);
    }

    #[test]
    fn take_any_char() {
        let mut tokens = Tokenizer::new(".");
        assert_eq!(tokens.take_any_char(), Token::AnyChar);
        assert_eq!(tokens.chars.next(), None);
    }

}

