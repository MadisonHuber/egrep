pub mod helpers;

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

        let start = nfa.add(Start(None));
        nfa.start = start;

        // Parse the Abstract Syntax Tree of the Regular Expression
        let ast = &Parser::parse(Tokenizer::new(regular_expression))?;
        // The "body" of the NFA is made of the states between Start and End
        let body = nfa.gen_fragment(ast);
        nfa.join(nfa.start, body.start);

        let end = nfa.add(End);
        nfa.join_fragment(&body, end);

        Ok(nfa)
    }

    /**
     * Given an input string, simulate the NFA to determine if the
     * input is accepted by the input string.
     */
    pub fn accepts(&self, input: &str) -> bool {
        false
    }
}

/**
 * ===== Internal API =====
 */
type StateId = usize;

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
    fn add(&mut self, state: State) -> StateId {
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
        let state = self.add(Match(Char::Any, None));
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
        let state = self.add(Match(Char::Literal(c), None));
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
        let split = self.add(Split(Some(left.start), Some(right.start)));

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
        let split = self.add(Split(Some(child.start), None));
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
