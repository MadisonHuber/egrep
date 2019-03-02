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
        self.lex_whitespace(); // implement this!
        // continue implementing
    }
}

