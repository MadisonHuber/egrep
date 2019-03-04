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
pub enum AST {
    Alternation(Box<AST>, Box<AST>),
    Catenation(Box<AST>, Box<AST>),
    Closure(Box<AST>),
    Char(char),
    AnyChar,
}
