use std::collections::HashMap;
use std::mem;

/// Key used by string interner. Instead of passing strings around, instances
/// of `Sym` are used, which can in turn be used to query the string interner
/// for the original string slice.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Sym(u32);

impl Sym {
    pub fn new(n: u32) -> Self {
        Sym(n)
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

impl Symbolic for Sym {
    fn get_sym(&self) -> Sym {
        *self
    }
}

impl std::fmt::Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}

impl From<Sym> for u32 {
    fn from(Sym(n): Sym) -> Self {
        n
    }
}

impl From<usize> for Sym {
    fn from(n: usize) -> Self {
        Sym(n as u32)
    }
}

/// Trait allowing for retrieval of underlying `Sym` for data types with a
/// `Sym` payload. This allows for a slightly more ergonomic way of looking up
/// stored strings associated with a given type.
pub trait Symbolic {
    fn get_sym(&self) -> Sym;
}

/// Trait for types that may facilitate interning. This should be implemented
/// by all types that can store or retrieve data.
pub trait Interner {
    type Key;
    type Value: ?Sized;

    /// Stores the given value if it is not currently already stored.
    /// Returns the key used by the `Self` to retrieve the stored value.
    fn intern(&mut self, value: &Self::Value) -> Self::Key;

    fn lookup(&self, key: &Self::Key) -> &Self::Value;
}

/// String interner. Instead of allocating a new string during the compilation
/// process, all strings are instead interned and mapped to instances of type
/// `Symbol`, which unlike `&str` and `String`, are [`Copy`] and additionally
/// more lightweight.
///
/// Note that the interned string slice itself is stored as the `key`, while
/// the client effectively uses the hashmap's entry value as the *value*.
#[derive(Clone, Debug, Default)]
pub struct Lexicon {
    map: HashMap<&'static str, Sym>,
    vec: Vec<&'static str>,
    buf: String,
    all: Vec<String>,
}

impl Lexicon {
    // Initial value just randomly guessed.
    // This could/should maybe be optimized later.
    pub const BASE_CAPACITY: usize = 100;

    pub fn with_capacity(capacity: usize) -> Self {
        let cap = capacity.next_power_of_two();
        Self {
            map: HashMap::default(),
            vec: Vec::new(),
            buf: String::with_capacity(cap),
            all: Vec::new(),
        }
    }

    /// Stores a string slice in the interner, returning the `Sym` item
    /// which can be used to retrieve the stored string.
    pub fn intern(&mut self, string: &str) -> Sym {
        if let Some(&id) = self.map.get(string) {
            return id;
        }

        let string = unsafe { self.alloc(string) };
        let sym = Sym::new(self.map.len() as u32);

        self.map.insert(string, sym);
        self.vec.push(string);

        debug_assert!(self.lookup(sym) == string);
        debug_assert!(self.intern(string) == sym);

        sym
    }

    pub fn lookup(&self, sym: Sym) -> &str {
        self.vec[sym.get() as usize]
    }

    unsafe fn alloc(&mut self, string: &str) -> &'static str {
        let cap = self.buf.capacity();
        if cap < self.buf.len() + string.len() {
            // just doubling isn't enough -- need to ensure the new string actually fits
            let new_cap = (cap.max(string.len()) + 1).next_power_of_two();
            let new_buf = String::with_capacity(new_cap);
            let old_buf = mem::replace(&mut self.buf, new_buf);
            self.all.push(old_buf);
        }

        let interned = {
            let start = self.buf.len();
            self.buf.push_str(string);
            &self.buf[start..]
        };

        &*(interned as *const str)
    }
}

impl std::ops::Index<Sym> for Lexicon {
    type Output = str;

    fn index(&self, index: Sym) -> &Self::Output {
        self.lookup(index)
    }
}

pub fn init_with_alphabet() -> Lexicon {
    // the `Default` trait is automatically derived and
    // does not contain any stored data.
    let mut lexicon = Lexicon::default();
    for c in ('a'..='z').chain('A'..='Z') {
        lexicon.intern(&*c.to_string());
    }
    lexicon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_lexicon_with_alphas() {
        let lexicon = init_with_alphabet();

        let zero: Sym = 0.into();
        let one: Sym = 1.into();
        assert_eq!(&lexicon[zero], "a");
        assert_eq!(lexicon.lookup(one), "b");
        assert_eq!(lexicon.lookup(26.into()), "A")
    }
}
