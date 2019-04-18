pub mod helpers;
use std::ops::Add;

// Starter code for PS06 - thegrep
use self::State::*;
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
use super::parser::Parser;
use super::parser::AST;
use super::tokenizer::Tokenizer;

/**
 * ===== Public API =====
 */

/**
 * An NFA is represented by an arena Vec of States
 * and a start state.
 */
#[derive(Debug)]
pub struct NFA {
    start: StateId,
    states: Vec<State>,
}

impl NFA {
    /**
     * Construct an NFA from a regular expression pattern.
     */
    pub fn from(regular_expression: &str) -> Result<NFA, String> {
        let mut nfa = NFA::new();

        let start = nfa.add_state(Start(None));
        nfa.start = start;

        // Parse the Abstract Syntax Tree of the Regular Expression
        let ast = &Parser::parse(Tokenizer::new(regular_expression))?;
        // The "body" of the NFA is made of the states between Start and End
        let body = nfa.gen_fragment(ast);
        nfa.join(nfa.start, body.start);

        let end = nfa.add_state(End);
        nfa.join_fragment(&body, end);

        Ok(nfa)
    }

    /**
     * Given an input string, simulate the NFA to determine if the
     * input is accepted by the input string.
     */
    pub fn accepts(&self, input: &str) -> bool {
        let mut itr = input.chars();

        // List of next states after Start is generated in helper function
        let mut next_states = Vec::new();
        self.find_next(0, &mut next_states);

        // Step forward by making next states the current states
        let mut curr_states = next_states;

        while let Some(curr) = itr.next() {
            // Reset next states so the next states can be regenerated
            next_states = Vec::new();

            // Add to next states all possible next states for all current states
            for state in curr_states {
                // curr_states only holds state indices, so actual State must be matched against
                match self.states[state] {
                    Split(Some(lhs), Some(rhs)) => {
                        // Call helper twice for both ends of Split
                        self.find_next(lhs, &mut next_states);
                        self.find_next(rhs, &mut next_states);
                    }
                    Match(Char::Any, Some(idx)) => {
                        self.find_next(idx, &mut next_states);
                    }
                    Match(Char::Literal(c), Some(idx)) => {
                        // If char in input matches char in Match, call helper
                        if c == curr {
                            self.find_next(idx, &mut next_states);
                        }
                    }
                    Start(Some(idx)) => {
                        self.find_next(idx, &mut next_states);
                    }
                    _ => { /*need this for Start(None) so nothing*/ }
                }
            }

            // Step forward by making next states the current states
            curr_states = next_states;

            // Check to see if End state is in current states, if so
            // We found a matching input string and return true!
            if curr_states.contains(&(self.states.len() - 1)) {
                return true;
            }
        }

        // Checks for End states in case where input string is a blank line.
        if curr_states.contains(&(self.states.len() - 1)) {
            return true;
        }

        // If End state is never reached, not a match,
        // return false
        false
    }

    /**
     * Given a current StateId, find all possible next states
     * from that State.
     */
    fn find_next(&self, curr_state: StateId, next_states: &mut Vec<StateId>) {
        // curr_state is a StateId, so actual State must be matched against
        match self.states[curr_state] {
            Start(Some(id)) => {
                self.find_next(id, next_states);
            }
            Match(_, Some(_)) => {
                // Base case, add StateId to next states
                next_states.push(curr_state);
            }
            Split(Some(id_1), Some(id_2)) => {
                // Recursive case, recursive call for both ends of Split
                // to zoom past epsilon transitions
                self.find_next(id_1, next_states);
                self.find_next(id_2, next_states);
            }
            End => {
                // Base case, add StateId to next states
                next_states.push(curr_state);
            }
            _ => { /*for State pointing to None*/ }
        }
    }
}

/**
 * Unit tests for `accepts` method.
 */
#[cfg(test)]
mod accepts_tests {
    use super::*;

    #[test]
    fn single_lit_char() {
        let nfa = NFA::from("a").unwrap();
        let input = "a";
        assert!(nfa.accepts(input));
    }

    #[test]
    fn single_lit_char_wrong() {
        let nfa = NFA::from("a").unwrap();
        let input = "h";
        assert_eq!(nfa.accepts(input), false);
    }

    #[test]
    fn single_any_char() {
        let nfa = NFA::from(".").unwrap();
        let input = ".";
        assert!(nfa.accepts(input));
        let input2 = "b";
        assert!(nfa.accepts(input2));
    }

    #[test]
    fn extra_input() {
        // need it at end too? (it being .*)
        let nfa = NFA::from(".*b").unwrap();
        let input = "abc";
        assert!(nfa.accepts(input));
    }

    #[test]
    fn catenation_pattern_exact() {
        let nfa = NFA::from("abc").unwrap();
        let input = "abc";
        assert!(nfa.accepts(input));
    }

    #[test]
    fn catenation_pattern_string() {
        let nfa = NFA::from(".*amin").unwrap();
        let input = "flamingo";
        assert!(nfa.accepts(input));
    }

    #[test]
    fn alternation_basic() {
        let nfa = NFA::from("a|b").unwrap();
        let input = "b";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn alternation_intermediate() {
        let nfa = NFA::from("ab|c").unwrap();
        let input = "ab";
        assert_eq!(nfa.accepts(input), true);
        let input = "c";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn alternation_repeated() {
        let nfa = NFA::from("a|b|c").unwrap();
        let input = "ab";
        assert_eq!(nfa.accepts(input), true);
        let input = "c";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn closure_basic() {
        let nfa = NFA::from("a*").unwrap();
        let input = "aa";
        assert!(nfa.accepts(input));
    }

    #[test]
    fn closure_extra_input() {
        let nfa = NFA::from(".*a*").unwrap();
        let input = "baa";
        assert!(nfa.accepts(input));
    }

    #[test]
    fn closure_longer_input() {
        let nfa = NFA::from("a*").unwrap();
        let input = "aaa";
        assert!(nfa.accepts(input));
    }

    #[test]
    fn closure_in_middle_of_pattern() {
        let nfa = NFA::from("ab*c").unwrap();
        let input = "abbbbbbc";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn closure_fail() {
        let nfa = NFA::from("ab*c").unwrap();
        let input = "abbbb";
        assert_eq!(nfa.accepts(input), false);
    }

    #[test]
    fn stress_test_any() {
        let nfa = NFA::from("(a|b.)*").unwrap();
        let input = "bobo";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn multiple_alt_closure() {
        let nfa = NFA::from(".*(a|b|c)*").unwrap();
        let input = "fbc";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn stress_test_lit_long() {
        let nfa = NFA::from("(a|bc)*").unwrap();
        let input = "bcbcaa";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn stress_test_lit_short() {
        let nfa = NFA::from("(a|bc)*").unwrap();
        let input = "bcbc";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn any_closure() {
        let nfa = NFA::from("a.*c").unwrap();
        let input = "adfgc";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn closure_alt() {
        let nfa = NFA::from("a.*(d|c)").unwrap();
        let input = "adfgc";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn fab() {
        let nfa = NFA::from(".*fab").unwrap();
        // not getting state 6 on faF
        let input = "fafab";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn aaab() {
        let nfa = NFA::from(".*aaab").unwrap();
        let input = "abaaaaabc";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn abaa() {
        let nfa = NFA::from(".*abaa").unwrap();
        let input = "ababaa";
        assert_eq!(nfa.accepts(input), true);
    }

    #[test]
    fn empty() {
        let nfa = NFA::from(".*").unwrap();
        let input = "
        ";
        assert_eq!(nfa.accepts(input), true);
    }
}

/**
 * ===== Internal API =====
 */
type StateId = usize;
// pass with IDs
// unless split in which case two back to back calls (seems wroooonnnnggggg) with each l and r ID
// in the Split
// then in that helper which we just called
// match the state we passed in until base case of a Match state and match the char we're dealing
// with

/**
 * States are the elements of our NFA Graph
 * - Start is starting state
 * - Match is a state with a single matching transition out
 * - Split is a state with two epsilon transitions out
 * - End is the final accepting state
 */
#[derive(Debug)]
enum State {
    Start(Option<StateId>),
    Match(Char, Option<StateId>),
    Split(Option<StateId>, Option<StateId>),
    End,
}

/**
 * Chars are the matching label of a non-epsilon edge in the
 * transition diagram representation of the NFA.
 */
#[derive(Debug)]
enum Char {
    Literal(char),
    Any,
}

/**
 * Internal representation of a fragment of an NFA being constructed
 * that keeps track of the start ID of the fragment as well as all of
 * its unjoined end states.
 */
#[derive(Debug)]
struct Fragment {
    start: StateId,
    ends: Vec<StateId>,
}

/**
 * Private methods of the NFA structure.
 */
impl NFA {
    /**
     * Constructor establishes an empty states Vec.
     */
    fn new() -> NFA {
        NFA {
            states: vec![],
            start: 0,
        }
    }

    /**
     * Add a state to the NFA and get its arena ID back.
     */
    fn add_state(&mut self, state: State) -> StateId {
        let idx = self.states.len();
        self.states.push(state);
        idx
    }

    /**
     * Given an AST node, this method returns a Fragment of the NFA
     * representing it and its children.
     */
    fn gen_fragment(&mut self, ast: &AST) -> Fragment {
        match ast {
            AST::AnyChar => self.gen_any(),
            AST::Char(c) => self.gen_char(*c),
            AST::Catenation(lhs, rhs) => self.gen_cat(lhs, rhs),
            AST::Alternation(lhs, rhs) => self.gen_alt(lhs, rhs),
            AST::Closure(c) => self.gen_closure(c),
        }
    }

    /**
     * Helper for gen_fragment AST::AnyChar.
     * Creates a Match state with AnyChar and
     * returns corresponding Fragment.
     */
    fn gen_any(&mut self) -> Fragment {
        let state = self.add_state(Match(Char::Any, None));
        Fragment {
            start: state,
            ends: vec![state],
        }
    }

    /**
     * Helper for gen_fragment AST::Char
     * Creates a Match state with Char and
     * returns corresponding Fragment.
     */
    fn gen_char(&mut self, c: char) -> Fragment {
        let state = self.add_state(Match(Char::Literal(c), None));
        Fragment {
            start: state,
            ends: vec![state],
        }
    }

    /**
     * Helper for gen_fragment AST::Catenation
     * Creates Fragments from left and right hand sides,
     * and concatenates them (left to right). Returns
     * corresponding Fragment.
     */
    fn gen_cat(&mut self, lhs: &Box<AST>, rhs: &Box<AST>) -> Fragment {
        let left = self.gen_fragment(&lhs);
        let right = self.gen_fragment(&rhs);
        self.join_fragment(&left, right.start);
        Fragment {
            start: left.start,
            ends: right.ends,
        }
    }

    /**
     * Helper for gen_fragment AST::Alternation
     * Creates Fragments from left and right hand sides,
     * and creates a Split state that connects to both.
     * Returns corresponding Fragment.
     */
    fn gen_alt(&mut self, lhs: &Box<AST>, rhs: &Box<AST>) -> Fragment {
        let left = self.gen_fragment(&lhs);
        let right = self.gen_fragment(&rhs);
        let split = self.add_state(Split(Some(left.start), Some(right.start)));

        // Take states from ends of left and right Fragments
        // and combine them into one vector that becomes the
        // ends of the returned Fragment
        let mut endings = left.ends.clone();
        let mut rights = right.ends.clone();
        endings.append(&mut rights);

        Fragment {
            start: split,
            ends: endings,
        }
    }

    /**
     * Helper for gen_fragment AST::Closure
     * Creates Fragment from child, and creates a
     * Split state that connects to child and has an
     * unconnected arm. Returns corresponding Fragment.
     */
    fn gen_closure(&mut self, c: &Box<AST>) -> Fragment {
        let child = self.gen_fragment(&c);
        let split = self.add_state(Split(Some(child.start), None));
        self.join_fragment(&child, split);
        Fragment {
            start: split,
            ends: vec![split],
        }
    }

    /**
     * Join all the loose ends of a fragment to another StateId.
     */
    fn join_fragment(&mut self, lhs: &Fragment, to: StateId) {
        for end in &lhs.ends {
            self.join(*end, to);
        }
    }

    /**
    ??? from here until ???END lines may have been inserted/deleted
         * Join a loose end of one state to another by IDs.
         * Note in the Split case, only the 2nd ID (rhs) is being bound.
         * It is assumed when building an NFA with these constructs
         * that the lhs of an Split state will always be known and bound.
         */
    fn join(&mut self, from: StateId, to: StateId) {
        match self.states[from] {
            Start(ref mut next) => *next = Some(to),
            Match(_, ref mut next) => *next = Some(to),
            Split(_, ref mut next) => *next = Some(to),
            End => {}
        }
    }
}

#[cfg(test)]
mod fragment_tests {
    use super::*;
    use crate::nfa::helpers::nfa_dot;

    #[test]
    fn char() {
        let nfa = NFA::from("a").unwrap();
        let dot_rep = nfa_dot(&nfa);
        let dot_string = format!(
            "digraph nfa {{rankdir=LR; \n\tnode [shape = circle];\n\tstart [shape=\"none\"]\n\tstart -> 1\n\t1 -> 2 [label=\"a\"]\n\t2 [shape=\"doublecircle\"]\n}}"
        );
        assert_eq!(dot_rep, dot_string);
    }

    #[test]
    fn cat() {
        let nfa = NFA::from("ab").unwrap();
        let dot_rep = nfa_dot(&nfa);
        let dot_string = format!("digraph nfa {{rankdir=LR; \n\tnode [shape = circle];\n\tstart [shape=\"none\"]\n\tstart -> 1\n\t1 -> 2 [label=\"a\"]\n\t2 -> 3 [label=\"b\"]\n\t3 [shape=\"doublecircle\"]\n}}");
        assert_eq!(dot_rep, dot_string);
    }

    #[test]
    fn alt() {
        let nfa = NFA::from("a|b").unwrap();
        let dot_rep = nfa_dot(&nfa);
        let dot_string = format!("digraph nfa {{rankdir=LR; \n\tnode [shape = circle];\n\tstart [shape=\"none\"]\n\tstart -> 3\n\t1 -> 4 [label=\"a\"]\n\t2 -> 4 [label=\"b\"]\n\t3 -> 2 [label=\"ε\"]\n\t3 -> 1 [label=\"ε\"]\n\t4 [shape=\"doublecircle\"]\n}}");
        assert_eq!(dot_rep, dot_string);
    }

    #[test]
    fn clos() {
        let nfa = NFA::from(".*").unwrap();
        let dot_rep = nfa_dot(&nfa);
        let dot_string = format!("digraph nfa {{rankdir=LR; \n\tnode [shape = circle];\n\tstart [shape=\"none\"]\n\tstart -> 2\n\t1 -> 2 [label=\"ANY\"]\n\t2 -> 3 [label=\"ε\"]\n\t2 -> 1 [label=\"ε\"]\n\t3 [shape=\"doublecircle\"]\n}}");
        assert_eq!(dot_rep, dot_string);
    }

    #[test]
    fn stress() {
        let nfa = NFA::from("(a|b.)*").unwrap();
        let dot_rep = nfa_dot(&nfa);
        let dot_string = format!("digraph nfa {{rankdir=LR; \n\tnode [shape = circle];\n\tstart [shape=\"none\"]\n\tstart -> 5\n\t1 -> 5 [label=\"a\"]\n\t2 -> 3 [label=\"b\"]\n\t3 -> 5 [label=\"ANY\"]\n\t4 -> 2 [label=\"ε\"]\n\t4 -> 1 [label=\"ε\"]\n\t5 -> 6 [label=\"ε\"]\n\t5 -> 4 [label=\"ε\"]\n\t6 [shape=\"doublecircle\"]\n}}");
        assert_eq!(dot_rep, dot_string);
    }

}

impl Add for NFA {
    type Output = NFA;
    fn add(self, rhs: NFA) -> NFA {
        let mut idx = 0;
        let mut new_nfa = Vec::new();
        while idx < self.states.len() - 1 {
            //new_nfa.push(self.states[idx]);
        }
        let length = new_nfa.len();
        while idx < rhs.states.len() {
            match &rhs.states[idx] {
                Start(Some(id)) => {
                    let s = Start(Some(*id + length));
                    new_nfa.push(s);
                }
                Match(c, Some(id)) => {
                    if let Char::Literal(ch) = &c {
                        new_nfa.push(Match(Char::Literal(*ch), Some(*id + length)));
                    } else {
                        new_nfa.push(Match(Char::Any, Some(*id + length)));
                    }
                }
                Split(Some(id_1), Some(id_2)) => {
                    new_nfa.push(Split(Some(*id_1 + length), Some(*id_2 + length)));
                }
                End => {
                    new_nfa.push(End);
                }
                _ => {
                    break;
                }
            }
            idx += 1;
        }
        NFA {
            start: 0,
            states: new_nfa,
        }
    }
}
