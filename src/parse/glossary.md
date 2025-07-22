
# Some useful words to understand parsing

## General

- parsing: in the context of Skribi, we transform a text into a tree.
    This allows us to execute it easily later, without the need of reading the
    text each time.
- native call: an explicit call to a function implemented in a parent language.
    These functions could not be easily implemented directly in Skribi.
    So we are calling them with the skr_app keyword.
    Examples: print/println, read an input, generate an error, ...
    In Skribi, a native call must take a line. See the NatCall node.
- identifier: a token that identify an element using letters, numbers and
    underscores. They often represent a variable name.

