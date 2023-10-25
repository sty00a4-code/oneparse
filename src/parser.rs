use crate::{lexer::Lexable, position::Located};

pub trait Parsable<T: Lexable>
where
    Self: Sized,
{
    type Error;
    fn parse(parser: &mut Parser<T>) -> Result<Located<Self>, Located<Self::Error>>;
}

pub struct Parser<T: Lexable> {
    pub tokens: Vec<Located<T>>,
}

impl<T: Lexable> Parser<T> {
    pub fn new(tokens: Vec<Located<T>>) -> Self {
        Self { tokens }
    }
    /// pop of the next token if there are any tokens left
    pub fn token(&mut self) -> Option<Located<T>> {
        if self.tokens.is_empty() {
            None
        } else {
            Some(self.tokens.remove(0))
        }
    }
    /// returns a reference to the next token if there are any token left
    pub fn token_ref(&mut self) -> Option<&Located<T>> {
        self.tokens.first()
    }
}
