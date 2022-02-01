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

// Initializes the lexicon with lower and uppercase letters of the alphabet
// note: this function is included in this library
fn init_with_alphabet() -> Lexicon {
    // the `Default` trait is automatically derived and 
    // does not contain any stored data.
    let mut lexicon = Lexicon::default();
    for c in ('a'..='z').chain('A'..='Z') {
        lexicon.intern(&*c.to_string());
    }
    lexicon
}

pub fn main() {
    // initialize with alphabet
    // initialize with alphabet
    let mut lexicon = init_with_alphabet();

    // since we initialized it with all (upper and lowercase)
    // graphemes of the alphabet, interning this will simply 
    // perform a lookup
    let little_x = "x";
    let little_x_key = lexicon.intern(little_x);
    assert_eq!(little_x, lowercase_x);

    let big_x = "X";
    let big_x_key = lexicon.intern(big_x);
    assert_eq!(big_x, uppercase_x);

    // let's store something new
    let a_word = "meowdy, world!";
    // storing something always returns its key
    let a_words_key = lexicon.intern(a_word);
    
    // let's retrieve the stored strings now
    // we can use the `lookup` method, which returns a static
    // reference to a string slice
    let lowercase_x = lexicon.lookup(sym);
    
    // the lexicon also has an implementation for `std::ops::Index<Sym>` 
    // allowing for array index syntax. HOWEVER, this needs to be stored
    // as a reference, as `str` is not sized.
    let uppercase_x = &lexicon[big_x_key];
    let greeting = &lexicon[a_words_key];

    println!("Cats in Texas say \"{}\"", greeting)
}
```