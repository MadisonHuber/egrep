use super::NFA;
use super::Char;
use super::State::*;

/**
 * Helper functions for visualizing our NFA
 * Both at the internal representation level and in dot format
 * to generate a graphical representation.
 */

/**
 * Generate a string of the internal structure of the NFA.
 */
pub fn nfa_dump(nfa: &NFA) -> String {
    let mut s = String::new();
    for (id, state) in nfa.states.iter().enumerate() {
        s.push_str(&format!("{:03} | {:?}\n", id, state));
    }
    s
}

/**
 * Generate a DOT structured string.
 */
pub fn nfa_dot(nfa: &NFA) -> String {
    let mut dot = String::from("digraph nfa {rankdir=LR; \n\tnode [shape = circle];\n");
    for (id, state) in nfa.states.iter().enumerate() {
        dot.push_str(&match state {
            Start(Some(next)) => format!("\tstart [shape=\"none\"]\n\tstart -> {}\n", next),
            Match(c, Some(next)) => format!("\t{} -> {} [label=\"{}\"]\n", id, next, c),
            Split(Some(lhs), Some(rhs)) => format!(
                "\t{0} -> {1} [label=\"ε\"]\n\t{0} -> {2} [label=\"ε\"]\n",
                id, rhs, lhs
            ),
            End => format!("\t{} [shape=\"doublecircle\"]\n", id),
            _ => String::new(),
        });
    }
    dot += "}";
    dot
}

/**
 * Used by the DOT helper function to generate labels for each edge.
 */
impl std::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Char::Literal(c) => write!(f, "{}", c),
            Char::Any => write!(f, "ANY"),
        }
    }
}

use rand::prelude::*;
use rand::distributions::Alphanumeric;

pub fn gen(nfa: &NFA, num: usize) -> Vec<String> {
    let mut strings: Vec<String> = Vec::new();
    while strings.len() < num {
        let mut s = String::new();
        let mut idx = 0;
        loop {
            match &nfa.states[idx] {
                Start(Some(id)) => idx = *id,
                Match(c, Some(id)) => {
                    // if c is a char literal, push that onto s
                    if let Char::Literal(ch) = &c {
                        s.push(*ch);
                    } else {
                        // push random char since any
                        s.push(thread_rng().sample(Alphanumeric));
                    }
                    idx = *id;
                },
                Split(Some(lhs), Some(rhs)) => {
                    if random() {
                        idx = *lhs;
                    } else {
                        idx = *rhs;
                    }
                },
                End => {
                    strings.push(s);
                    break; // exits loop
                },
                _ => break,
            }
        }
    }
    strings
}

#[cfg(test)]
mod gen_tests {
    use super::*;

    #[test]
    fn gen_4_a() {
        let nfa = NFA::from("a").unwrap();
        let strings = gen(&nfa, 4);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }

    #[test]
    fn gen_4_any() {
        let nfa = NFA::from(".").unwrap();
        let strings = gen(&nfa, 4);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }

    #[test]
    fn gen_4_any_closure() {
        let nfa = NFA::from(".*").unwrap();
        let strings = gen(&nfa, 4);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }

    #[test]
    fn gen_15_any_plus() {
        let nfa = NFA::from(".+").unwrap();
        let strings = gen(&nfa, 15);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }

    #[test]
    fn gen_1_alt() {
        let nfa = NFA::from("a|b").unwrap();
        let strings = gen(&nfa, 1);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }

    #[test]
    fn gen_20_alt_cat() {
        let nfa = NFA::from("ab|cd").unwrap();
        let strings = gen(&nfa, 20);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }

    #[test]
    fn gen_7_all() {
        let nfa = NFA::from("a(b|c)d*e+f").unwrap();
        let strings = gen(&nfa, 7);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }

    #[test]
    fn gen_7_alt_plus() {
        // let nfa = NFA::from("(a*|bc)+").unwrap();
        let nfa = NFA::from("(a|b)+").unwrap();
        let strings = gen(&nfa, 3);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }
    
    #[test]
    fn gen_7_precedence() {
        let nfa = NFA::from("(a*|bc)+").unwrap();
        let strings = gen(&nfa, 3);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }

    /*
    #[test]
    fn gen_5_empty() {
        let nfa = NFA::from("").unwrap();
        println!("{:?}", nfa);
        let strings = gen(&nfa, 5);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }
    */
}
