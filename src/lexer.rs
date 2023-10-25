use crate::position::{Located, Positon};

pub trait Lexable
where
    Self: Sized,
{
    type Error;
    /// provides a lexer to lex the next token with
    fn lex(lexer: &mut Lexer) -> Result<Option<Located<Self>>, Located<Self::Error>>;
}

pub struct Lexer {
    pub text: String,
    pub idx: usize,
    pub ln: usize,
    pub col: usize,
}
impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text,
            idx: 0,
            ln: 0,
            col: 0,
        }
    }
    /// lexes and collects all the tokens in a vector until `T::lex` returns `Ok(None)`
    pub fn lex<T: Lexable>(&mut self) -> Result<Vec<Located<T>>, Located<T::Error>> {
        let mut tokens = vec![];
        while let Some(token) = T::lex(self)? {
            tokens.push(token)
        }
        Ok(tokens)
    }
    /// returns the current character
    pub fn get(&self) -> Option<char> {
        self.text.get(self.idx..=self.idx)?.chars().next()
    }
    /// returns the current characters position
    pub fn pos(&self) -> Positon {
        Positon::new(self.ln..self.ln, self.col..self.col + 1)
    }
    /// advances to the next character updating the `ln`, `col` and `idx`
    pub fn advance(&mut self) {
        if self.get() == Some('\n') {
            self.ln += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        self.idx += 1;
    }
}
