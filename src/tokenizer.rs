use std::str::Chars;

/**
 * thegrep - Tar Heel Extended Global Regular Expression Print
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
    chars: Chars<'str>,
}

impl<'str> Tokenizer<'str> {
    pub fn new(input: &'str str) -> Tokenizer {
        Tokenizer {
            chars: input.chars(),
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
        if let Some(c) = self.chars.next() {
            Some(match c {
                '(' => Token::LParen,
                ')' => Token::RParen,
                '|' => Token::UnionBar,
                '*' => Token::KleeneStar,
                '.' => Token::AnyChar,
                _ => Token::Char(c),
            })
        } else {
            None
        }
    }
}

/**
 * Unit Tests for the `next` method.
 */
#[cfg(test)]
mod iterator {
    use super::*;

    #[test]
    fn empty() {
        let mut tokens = Tokenizer::new("");
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn next_lparen() {
        let mut tokens = Tokenizer::new("(");
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn next_rparen() {
        let mut tokens = Tokenizer::new(")");
        assert_eq!(tokens.next(), Some(Token::RParen));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn next_union_bar() {
        let mut tokens = Tokenizer::new("|");
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn next_kleene_star() {
        let mut tokens = Tokenizer::new("*");
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn next_any() {
        let mut tokens = Tokenizer::new(".");
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn next_char() {
        let mut tokens = Tokenizer::new("a");
        assert_eq!(tokens.next(), Some(Token::Char('a')));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn next_stress_test() {
        let mut tokens = Tokenizer::new("ab|().*");
        assert_eq!(tokens.next(), Some(Token::Char('a')));
        assert_eq!(tokens.next(), Some(Token::Char('b')));
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), Some(Token::RParen));
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), None);
    }
}
