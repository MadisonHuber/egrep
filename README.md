# ps05-thegrep-badasscoderladies-mh-af
ps05-thegrep-badasscoderladies-mh-af created by GitHub Classroom


_Design –_

We broke the program into stages: tokenization and parsing. Tokenization was approached by creating an enumeration of Token types and creating a peekable iterator over the input characters from which the tokens were produced. Each character was consumed one at a time, and a corresponding enumeration type was returned as an Option<Token>.

Parsing was approached by creating an enumeration of AST types, which represent the different components of a regular expression. Each enumeration variant had its own factory function that abstracted their production. We created a peekable iterator over the Tokens produced from Tokenizer. The parse() method begins the process by calling a series of internal helper methods that use mutual recursion and recursive descent to process the tokens according to the grammar. Additional helper methods were used to abstract significant portions of logic from other helper methods, to promote the design principle that all methods should handle only one task. 

The program begins in main.rs, which puts Tokenization and Parsing together. We use the structopt crate to handle command-line arguments that alter the behavior of the program. The -t or --tokens flag prints to stdout the Tokens produced during tokenization, and the -p or --parse flag prints to stdout a representation of the parsed regular expression. Any errors encountered during this process are sent to stderr.
