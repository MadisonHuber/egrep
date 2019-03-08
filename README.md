# ps05-thegrep-badasscoderladies-mh-af
ps05-thegrep-badasscoderladies-mh-af created by GitHub Classroom


_Design –_

We broke the program into stages: tokenization and parsing. Each part had its own file so as to abstract logic from `main.rs`. Each file adds a full unit of functionality which when combined in main create a sequence of processes that convert the input into what is desired in a step-by-step manner.

Tokenization was approached by creating an enumeration of `Token` types and creating a peekable iterator over the input characters from which the tokens were produced. Each character was consumed one at a time, and a corresponding enumeration type was returned as an `Option<Token>`.

Parsing was approached by creating an enumeration of `AST` types, which represent the different components of a regular expression. Each enumeration variant had its own factory function that abstracted their production. We created a peekable iterator over the `Token`s produced from `Tokenizer`. The `parse()` method begins the process by calling a series of internal helper methods that use mutual recursion and recursive descent to process the tokens according to the grammar. Additional helper methods were used to abstract significant portions of logic from other helper methods, to promote the design principle that all methods should handle only one task. 

The program begins in `main.rs`, which puts Tokenization and Parsing together. We use the structopt crate to handle command-line arguments that alter the behavior of the program. The `-t` or `--tokens` flag prints to `stdout` the `Token`s produced during tokenization, and the `-p` or `--parse` flag prints to stdout a representation of the parsed regular expression. Any errors encountered during this process are sent to `stderr`.

_Collaboration –_

We took the approach of pair programming in a "ping pong" pairing style throughout the entire process. This meant that we followed the test-driven development style of programming. For each individual unit of functionality, we began with a function stub returning a meaningless value and a failing test. We then swapped roles and took turns making the failing tests pass by implementing logic and adding more comprehensive tests, repeating this process until all the functionality was complete. All programming was completed together, in person.

_Contributions –_

Since all programming was carried out together following the "ping pong" pairing style, contributions were comprised of individual tests or units of logic. All decisions were made together – each person wrote unit tests and implemented logic in main, tokenizer, and parser. Since we collaborated on all parts, each member contributed to each discrete unit of functionality. Even when not physically writing the code, each person was always an active participant in the development process.
