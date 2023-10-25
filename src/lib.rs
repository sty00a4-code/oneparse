#![allow(unused)]

#[cfg(test)]
mod tests;

pub mod lexer;
pub mod parser;
pub mod position;

pub fn lex<T: lexer::Lexable>(text: String) -> Result<Vec<position::Located<T>>, position::Located<String>>
where
    T::Error: ToString,
{
    lexer::Lexer::new(text)
        .lex::<T>()
        .map_err(|err| err.map(|err| err.to_string()))
}

pub fn parse<T: lexer::Lexable, P: parser::Parsable<T>>(text: String) -> Result<position::Located<P>, position::Located<String>>
where
    P::Error: ToString,
    T::Error: ToString,
{
    let tokens = lex::<T>(text)?;
    let mut parser = parser::Parser::new(tokens);
    P::parse(&mut parser).map_err(|err| err.map(|err| err.to_string()))
}
