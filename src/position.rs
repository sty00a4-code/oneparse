use std::{
    fmt::{Debug, Display},
    ops::Range,
};

#[derive(Debug, Clone, Default)]
/// A position in a text
pub struct Positon {
    pub ln: Range<usize>,
    pub col: Range<usize>,
}

/// A wrapper to locate `T` in a text
pub struct Located<T> {
    pub value: T,
    pub pos: Positon,
}

impl Positon {
    pub fn new(ln: Range<usize>, col: Range<usize>) -> Self {
        Self { ln, col }
    }
    pub fn single(ln: usize, col: usize) -> Self {
        Self {
            ln: ln..ln + 1,
            col: col..col + 1,
        }
    }
    /// entends the ends of `self.ln` and `self.col` by the ends of `other`
    pub fn extend(&mut self, other: &Self) {
        self.ln.end = other.ln.end;
        self.col.end = other.col.end;
    }
}
impl<T> Located<T> {
    pub fn new(value: T, pos: Positon) -> Self {
        Self { value, pos }
    }
    /// map the inner `value`
    pub fn map<F: FnOnce(T) -> U, U>(self, f: F) -> Located<U> {
        Located {
            value: f(self.value),
            pos: self.pos,
        }
    }
    /// unwraps the inner `value`
    pub fn unwrap(self) -> T {
        self.value
    }
}

impl<T: Debug> Debug for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
impl<T: Display> Display for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
impl<T: Clone> Clone for Located<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            pos: self.pos.clone(),
        }
    }
}
impl<T: Default> Default for Located<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            pos: Positon::default(),
        }
    }
}
impl<T: PartialEq> PartialEq for Located<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<(Range<usize>, Range<usize>)> for Positon {
    fn from(value: (Range<usize>, Range<usize>)) -> Self {
        Self {
            ln: value.0,
            col: value.1,
        }
    }
}
