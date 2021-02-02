// location information
// ex. Loc(4, 6) : Represents the interval between 5th and 7th charcters of the input charcters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annot_new() {
        let annot = Annot::new(1, Loc(2, 4));
        assert_eq!(annot.value, 1);
        assert_eq!(annot.loc, Loc(2, 4));
    }
}
