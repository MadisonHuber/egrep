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

    #[structopt(short = "d", long = "dot", help = "Produce dot representation of NFA")]
    dot: bool,

    #[structopt(short = "g", long = "gen", help = "Generates random acceptable strings from regex", default_value = "0")]
    num: u64,

    #[structopt(help = "FILES")]
    paths: Vec<String>,
}

pub mod tokenizer;
use self::tokenizer::Tokenizer;
pub mod parser;
use self::parser::Parser;

pub mod nfa;
use self::nfa::NFA;
use self::nfa::helpers::nfa_dot;

fn main() {
    let opt = Options::from_args();
    eval(&opt.pattern, &opt);
}

fn eval(input: &str, options: &Options) {
    if options.tokens {
        eval_show_tokens(input);
    }

    if options.parse {
        eval_show_parse(input);
    }

    if options.dot {
        let nfa = NFA::from(input).unwrap();
        println!("{}", nfa_dot(&nfa));
        std::process::exit(0);
    }
     
    if options.num > 0 {
        let nfa = NFA::from(input).unwrap();
        // string_gen(&nfa);
    }

    let mut input_mod = String::from(".*");
    input_mod.push_str(input);
    let nfa = NFA::from(&input_mod).unwrap();
    // let nfa = NFA::from("nfa").unwrap();
    let result = if options.paths.len() > 0 {
        eval_files(&options, &nfa)
    } else {
        eval_stdin(&nfa)
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}

fn eval_show_tokens(input: &str) {
    let mut tokens = Tokenizer::new(input);
    while let Some(token) = tokens.next() {
        println!("{:?}", token);
    }
}

fn eval_show_parse(input: &str) {
    match Parser::parse(Tokenizer::new(input)) {
        Ok(statement) => {
            println!("{:?}", statement);
        }
        Err(msg) => eprintln!("thegrep: {}", msg),
    }
}

use std::fs::File;
use std::io::BufRead;
use std::io;

fn eval_files(opt: &Options, nfa: &NFA) -> io::Result<()> {
    for path in opt.paths.iter() {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        eval_lines(reader, nfa)?;
    }
    Ok(())
}

fn eval_stdin(nfa: &NFA) -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    eval_lines(reader, nfa)
}

fn eval_lines<R: BufRead>(reader: R, nfa: &NFA) -> io::Result<()> {
    for line_result in reader.lines() {
        let line = line_result?;
        if nfa.accepts(&line) {
            println!("{}", line);
        } 
    }
    Ok(())
}

