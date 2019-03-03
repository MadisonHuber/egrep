#![allow(unused)]
// remove above line before done

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

/**
 *
 * thegrep – Tar Heel Extended Global Regular Expressions Print
 * 
 */
extern crate structopt;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "thegrep", about = "Tar Heel egrep")]
struct Options {
    #[structopt(help = "Regular Expression Pattern")]
    pattern: String,

    #[structopt(short = "p", long = "parse", help = "Show Parsed AST")]
    parse: bool,

    #[structopt(short = "t", long = "tokens", help = "Show Tokens")]
    tokens: bool,
}

pub mod tokenizer;
use self::tokenizer::Tokenizer;

fn main() {
    let opt = Options::from_args();
    eval(&"(ab*c)", &opt);
}

fn eval(input: &str, options: &Options) {
    if options.tokens {
        eval_show_tokens(input);
    }
}

fn eval_show_tokens(input: &str) {
    let mut tokens = Tokenizer::new(input);
    while let Some(token) = tokens.next() {
        println!("{:?}", token);
    }
    print!("\n");
}
