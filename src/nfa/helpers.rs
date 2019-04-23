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
   // let mut dot = String::from("digraph nfa {rankdir=LR; \n\tnode [shape = circle];\n");
    let mut dot = String::from("digraph nfa {rankdir=LR;\n");
    for (id, state) in nfa.states.iter().enumerate() {
        let concat = concat!("Start", id);
        dot.push_str(&match state {
            // Start(Some(next)) => format!("\tstart [shape=\"none\"]\n\tstart -> {}\n", next),
            Start(Some(next)) => format!("\t{} -> {} [shape=doublecircle, label=\"Start\"]\n", concat, next),
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

/**
 * Generate a specified number of random strings that will be accepted by the given nfa. Takes in a
 * reference to an NFA and a usize which will be how many strings are generated.
 * Returns a Vec<String> containing all the generated strings.
 */
pub fn gen(nfa: &NFA, num: usize) -> Vec<String> {
    // create the vector that will contain the generated strings and that will be returned
    let mut strings: Vec<String> = Vec::new();

    // loop until the vector contains the specified number of strings
    while strings.len() < num {
        // make the string that will be built up to eventually become the accepted string
        let mut s = String::new();

        // idx is the index of the current state in the nfa, start at 0 which is the start state
        let mut idx = 0;

        // loop infinitely and only break when we are at the end state
        // go through the whole nfa state by state from start to end
        loop {
            match &nfa.states[idx] {
                Start(Some(id)) => idx = *id,
                Match(c, Some(id)) => {
                    // if c is a char literal, push that onto s
                    if let Char::Literal(ch) = &c {
                        s.push(*ch);
                    } else {
                        // otherwise push a random char since we can take any character
                        s.push(thread_rng().sample(Alphanumeric));
                    }
                    idx = *id;
                },
                Split(Some(lhs), Some(rhs)) => {
                    // use random() to generate a random bool
                    // in order to randomly decide which branch of the split to follow
                    if random() {
                        idx = *lhs;
                    } else {
                        idx = *rhs;
                    }
                },
                End => {
                    // we've reached the end of the nfa
                    // so push the built-up string onto the strings vec
                    strings.push(s);
                    
                    // made it through the nfa, so break out of the loop
                    break;
                },
                _ => { /* default case so match is comprehensive
                          don't want to do anthing here
                          since we should never reach this point */ },
            }
        }
    }
    // return the vector containing all the randomly generated strings
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
        let nfa = NFA::from("(a|b)+").unwrap();
        let strings = gen(&nfa, 7);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }
    
    #[test]
    fn gen_7_alt_closure() {
        let nfa = NFA::from("(a|b)*").unwrap();
        let strings = gen(&nfa, 7);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }

    #[test]
    fn gen_7_precedence() {
        let nfa = NFA::from("(a*|bc)+").unwrap();
        let strings = gen(&nfa, 7);
        for st in &strings {
            assert!(nfa.accepts(st));
        }
    }
}
