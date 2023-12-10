#![allow(unused)]

use std::{error::Error, fmt::Display};

#[cfg(test)]
mod tests;

pub mod lexer;
pub mod parser;
pub mod position;

pub fn lex<T: lexer::Lexable>(
    text: String,
) -> Result<Vec<position::Located<T>>, position::Located<T::Error>> {
    lexer::Lexer::new(text).lex::<T>()
}

pub fn parse<T: lexer::Lexable, P: parser::Parsable<T>, E: Error>(
    text: String,
) -> Result<position::Located<P>, position::Located<E>>
where
    P::Error: Display + Into<E>,
    T::Error: Display + Into<E>,
{
    let tokens = lex::<T>(text).map_err(|err| err.map(|err| err.into()))?;
    let mut parser = parser::Parser::new(tokens);
    P::parse(&mut parser).map_err(|err| err.map(|err| err.into()))
}
