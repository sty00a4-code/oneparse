use crate::position::Positon;

use super::*;

#[test]
fn simple_lexer() -> Result<(), String> {
    use lexer::Lexable;
    use position::Located;

    #[derive(Debug, PartialEq)]
    pub enum Token {
        Add, Sub, Div, Mul,
        Number(f64)
    }
    impl Lexable for Token {
        type Error = String;
        fn lex(lexer: &mut lexer::Lexer) -> Result<Option<position::Located<Self>>, position::Located<Self::Error>> {
            while let Some(c) = lexer.get() {
                if !c.is_whitespace() { break; }
                lexer.advance();
            }
            let mut pos = lexer.pos();
            let Some(c) = lexer.get() else {
                return Ok(None)
            };
            lexer.advance();
            match c {
                '+' => Ok(Some(Located::new(Token::Add, pos))),
                '-' => Ok(Some(Located::new(Token::Sub, pos))),
                '*' => Ok(Some(Located::new(Token::Mul, pos))),
                '/' => Ok(Some(Located::new(Token::Div, pos))),
                c if c.is_ascii_digit() => {
                    let mut number = String::from(c);
                    while let Some(c) = lexer.get() {
                        if !c.is_ascii_digit() { break; }
                        number.push(c);
                        pos.extend(&lexer.pos());
                        lexer.advance();
                    }
                    if lexer.get() == Some('.') {
                        number.push('.');
                        pos.extend(&lexer.pos());
                        lexer.advance();
                        while let Some(c) = lexer.get() {
                            if !c.is_ascii_digit() { break; }
                            number.push(c);
                            pos.extend(&lexer.pos());
                            lexer.advance();
                        }
                    }
                    Ok(Some(Located::new(Token::Number(number.parse().unwrap()), pos)))
                }
                c => Err(Located::new(format!("bad character {c:?}"), pos))
            }
        }
    }

    let tokens = lex::<Token>("1 + 2".to_string()).map_err(|err| err.unwrap())?;
    let mut tokens = tokens.into_iter();
    assert_eq!(tokens.next().unwrap().unwrap(), Token::Number(1.));
    assert_eq!(tokens.next().unwrap().unwrap(), Token::Add);
    assert_eq!(tokens.next().unwrap().unwrap(), Token::Number(2.));
    Ok(())
}

#[test]
fn simple_parser() -> Result<(), String> {
    use lexer::Lexable;
    use parser::Parsable;
    use position::Located;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Token {
        Add, Sub, Div, Mul,
        Number(f64)
    }
    impl Lexable for Token {
        type Error = String;
        fn lex(lexer: &mut lexer::Lexer) -> Result<Option<position::Located<Self>>, position::Located<Self::Error>> {
            while let Some(c) = lexer.get() {
                if !c.is_whitespace() { break; }
                lexer.advance();
            }
            let mut pos = lexer.pos();
            let Some(c) = lexer.get() else {
                return Ok(None)
            };
            lexer.advance();
            match c {
                '+' => Ok(Some(Located::new(Token::Add, pos))),
                '-' => Ok(Some(Located::new(Token::Sub, pos))),
                '*' => Ok(Some(Located::new(Token::Mul, pos))),
                '/' => Ok(Some(Located::new(Token::Div, pos))),
                c if c.is_ascii_digit() => {
                    let mut number = String::from(c);
                    while let Some(c) = lexer.get() {
                        if !c.is_ascii_digit() { break; }
                        number.push(c);
                        pos.extend(&lexer.pos());
                        lexer.advance();
                    }
                    if lexer.get() == Some('.') {
                        number.push('.');
                        pos.extend(&lexer.pos());
                        lexer.advance();
                        while let Some(c) = lexer.get() {
                            if !c.is_ascii_digit() { break; }
                            number.push(c);
                            pos.extend(&lexer.pos());
                            lexer.advance();
                        }
                    }
                    Ok(Some(Located::new(Token::Number(number.parse().unwrap()), pos)))
                }
                c => Err(Located::new(format!("bad character {c:?}"), pos))
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atom {
        Number(f64)
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum Expression {
        Binary { left: Located<Atom>, op: Token, right: Located<Atom> }
    }
    impl Parsable<Token> for Atom {
        type Error = String;
        fn parse(parser: &mut parser::Parser<Token>) -> Result<Located<Self>, Located<Self::Error>> {
            let Some(Located { value: token, pos }) = parser.token() else {
                return Err(Located::new("unexpected end of file".to_string(), Positon::default()))
            };
            match token {
                Token::Number(number) => Ok(Located::new(Self::Number(number), pos)),
                token => Err(Located::new(format!("unexpected {token:?}"), pos))
            }
        }
    }
    impl Parsable<Token> for Expression {
        type Error = String;
        fn parse(parser: &mut parser::Parser<Token>) -> Result<Located<Self>, Located<Self::Error>> {
            let left = Atom::parse(parser)?;
            let mut pos = left.pos.clone();
            let Some(op) = parser.token() else {
                return Err(Located::new("expected operator".to_string(), Positon::default()))
            };
            let right = Atom::parse(parser)?;
            pos.extend(&right.pos);
            Ok(Located::new(Self::Binary { left, op: op.unwrap(), right }, pos))
        }
    }

    let ast = parse::<Token, Expression>("1 + 2".to_string()).map_err(|err| err.to_string())?;
    #[allow(irrefutable_let_patterns)]
    let Expression::Binary { left, op, right } = ast.unwrap() else {
        panic!("not binary")
    };
    #[allow(irrefutable_let_patterns)]
    let Atom::Number(number) = left.unwrap() else {
        panic!("not a number")
    };
    assert_eq!(number, 1.);
    assert_eq!(op, Token::Add);
    #[allow(irrefutable_let_patterns)]
    let Atom::Number(number) = right.unwrap() else {
        panic!("not a number")
    };
    assert_eq!(number, 2.);

    Ok(())
}