# Lexicon
A simple string interner for Rust. 

# Examples
Suppose we want to intern the letters of the alphabet (both uppercase and lowercase). We can initialize the interner as such like so.

```rs
// the interner
use lexicon::Lexicon;
// newtype wrapper around `u32`, used to index `Lexicon`
use lexicon::Sym;
// helper trait to aid in extracting a contained `Sym` value for 
// data structures holding data representing strings
use lexicon::Symbolic;

// initialize the lexicon with lower and uppercase letters of the alphabet
fn init_with_alphabet() -> Lexicon {
    // the `Default` trait is automatically derived and 
    // does not contain any stored data.
    let mut lexicon = Lexicon::default();
    for c in ('a'..='z').chain('A'..='Z') {
        lexicon.intern(&*c.to_string());
    }
    lexicon
}

```