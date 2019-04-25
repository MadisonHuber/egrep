# Tar Heel Extended Global Regular Expression Print
Alana Fiordalisi and Madison Huber

ps05-thegrep-badasscoderladies-mh-af created by GitHub Classroom


_Design –_

We broke the program into stages: tokenization, parsing, construction of the NFA, and usage of the NFA to test input strings against a regular expression. Parts were broken down into multiple files so as to abstract logic from `main.rs`. Each file adds a full unit of functionality which when combined in main create a sequence of processes that convert the input into what is desired in a step-by-step manner.

Tokenization was approached by creating an enumeration of `Token` types and creating an iterator over the input characters from which the tokens were produced. Each character was consumed one at a time, and a corresponding enumeration type was returned as an `Option<Token>`.

Parsing was approached by creating an enumeration of `AST` types, which represent the different components of a regular expression. Each enumeration variant had its own factory function that abstracted their production. We created an iterator over the `Token`s produced from `Tokenizer`. The `parse()` method begins the process by calling a series of internal helper methods that use mutual recursion and recursive descent to process the tokens according to the grammar. Additional helper methods were used to abstract significant portions of logic from other helper methods, to promote the design principle that all methods should handle only one task.

Construction of the NFA was approached by using an adaptation of Thompson's Algorithm and connecting `Match` and `Split` `State`s as we recursively traversed the `AST` we produced from parsing. The NFA guided the process of accepting or rejecting input strings based on a regular expression. The program will accept and output an entire line of input if a sequence of characters accepted by the NFA is found within that line. Input strings were taken either from `stdin` or from files given as command line arguments, and lines containing accepted strings were printed to `stdout`.

After the initial construction and use of the NFA was implemented, we extended `nfa.rs` to allow for the concatenation of two NFAs by overloading the addition operator (`+`). The concatenation process does not mutate the two NFAs being operated on. The operator can be used with either two NFAs or two &NFAs. When used with &NFAs, the addition operator does not move the two original NFAs, and so they can be accessed after the concatenation process.

The program begins in `main.rs`, which puts Tokenization and Parsing together. We use the `structopt` crate to handle command-line arguments that alter the behavior of the program. The `-t` or `--tokens` flag prints to `stdout` the `Token`s produced during tokenization, the `-p` or `--parse` flag prints to `stdout` a representation of the parsed regular expression, and the `-d` or `--dot` flag will produce a DOT representation of the NFA. The `-g` or `--gen` flag generates a specified number of random strings that will be accepted by the provided NFA. The number of strings produced is passed as a command-line argument immediately following the flag. Any errors encountered during this process are sent to `stderr`.

We decided to put the `nfa_dot` function (used for creating the DOT representation of the NFA) and the `gen` function (used for producing random strings that are accepted by the NFA) in `nfa/helpers.rs`. This decision was made because these are the only two units of functionality that are not used to build up the NFA and that do not rely on reading strings from `stdin` or from files.

_Collaboration –_

We took the approach of pair programming in a "ping pong" pairing style throughout the entire process. This meant that we followed the test-driven development style of programming. For each individual unit of functionality, we began with a function stub returning a meaningless value and a failing test. We then swapped roles and took turns making the failing tests pass by implementing logic and adding more comprehensive tests, repeating this process until all the functionality was complete. All programming was completed together, in person.

_Contributions –_

Since all programming was carried out together following the "ping pong" pairing style, contributions were comprised of individual tests or units of logic. All decisions were made together – each person wrote unit tests and implemented logic in main, tokenizer, and parser. Since we collaborated on all parts, each member contributed to each discrete unit of functionality. Even when not physically writing the code, each person was always an active participant in the development process.
