use super::*;
use crate::position::Positon;

#[test]
fn simple_lexer() -> Result<(), String> {
    use lexer::Lexable;
    use position::Located;

    #[derive(Debug, PartialEq)]
    pub enum Token {
        Add,
        Sub,
        Div,
        Mul,
        Number(f64),
    }
    pub enum LexError {
        BadCharacter(char),
    }
    impl Display for LexError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                LexError::BadCharacter(c) => write!(f, "bad character {c:?}"),
            }
        }
    }
    impl Lexable for Token {
        type Error = LexError;
        fn lex(
            lexer: &mut lexer::Lexer,
        ) -> Result<Option<position::Located<Self>>, position::Located<Self::Error>> {
            while let Some(c) = lexer.get() {
                if !c.is_whitespace() {
                    break;
                }
                lexer.advance();
            }
            let mut pos = lexer.pos();
            let Some(c) = lexer.get() else {
                return Ok(None);
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
                        if !c.is_ascii_digit() {
                            break;
                        }
                        number.push(c);
                        pos.extend(&lexer.pos());
                        lexer.advance();
                    }
                    if lexer.get() == Some('.') {
                        number.push('.');
                        pos.extend(&lexer.pos());
                        lexer.advance();
                        while let Some(c) = lexer.get() {
                            if !c.is_ascii_digit() {
                                break;
                            }
                            number.push(c);
                            pos.extend(&lexer.pos());
                            lexer.advance();
                        }
                    }
                    Ok(Some(Located::new(
                        Token::Number(number.parse().unwrap()),
                        pos,
                    )))
                }
                c => Err(Located::new(LexError::BadCharacter(c), pos)),
            }
        }
    }

    let tokens = lex::<Token>("1 + 2".to_string()).map_err(|err| err.unwrap().to_string())?;
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
        Add,
        Sub,
        Div,
        Mul,
        Number(f64),
    }
    #[derive(Debug)]
    pub enum LexError {
        BadCharacter(char)
    }
    impl Display for LexError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                LexError::BadCharacter(c) => write!(f, "bad character {c:?}"),
            }
        }
    }
    impl Error for LexError {}
    impl Lexable for Token {
        type Error = LexError;
        fn lex(
            lexer: &mut lexer::Lexer,
        ) -> Result<Option<position::Located<Self>>, position::Located<Self::Error>> {
            while let Some(c) = lexer.get() {
                if !c.is_whitespace() {
                    break;
                }
                lexer.advance();
            }
            let mut pos = lexer.pos();
            let Some(c) = lexer.get() else {
                return Ok(None);
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
                        if !c.is_ascii_digit() {
                            break;
                        }
                        number.push(c);
                        pos.extend(&lexer.pos());
                        lexer.advance();
                    }
                    if lexer.get() == Some('.') {
                        number.push('.');
                        pos.extend(&lexer.pos());
                        lexer.advance();
                        while let Some(c) = lexer.get() {
                            if !c.is_ascii_digit() {
                                break;
                            }
                            number.push(c);
                            pos.extend(&lexer.pos());
                            lexer.advance();
                        }
                    }
                    Ok(Some(Located::new(
                        Token::Number(number.parse().unwrap()),
                        pos,
                    )))
                }
                c => Err(Located::new(LexError::BadCharacter(c), pos)),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atom {
        Number(f64),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum Expression {
        Binary {
            left: Located<Atom>,
            op: Token,
            right: Located<Atom>,
        },
    }
    #[derive(Debug)]
    pub enum ParseError {
        UnexpectedEOF,
        Unexpected(Token),
        ExpectedOperator
    }
    impl Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ParseError::UnexpectedEOF => write!(f, "unexpected end of file"),
                ParseError::Unexpected(token) => write!(f, "unexpected {token:?}"),
                ParseError::ExpectedOperator => write!(f, "expected operator"),
            }
        }
    }
    impl Error for ParseError {}
    impl Parsable<Token> for Atom {
        type Error = ParseError;
        fn parse(
            parser: &mut parser::Parser<Token>,
        ) -> Result<Located<Self>, Located<Self::Error>> {
            let Some(Located { value: token, pos }) = parser.token() else {
                return Err(Located::new(
                    ParseError::UnexpectedEOF,
                    Positon::default(),
                ));
            };
            match token {
                Token::Number(number) => Ok(Located::new(Self::Number(number), pos)),
                token => Err(Located::new(ParseError::Unexpected(token), pos)),
            }
        }
    }
    impl Parsable<Token> for Expression {
        type Error = ParseError;
        fn parse(
            parser: &mut parser::Parser<Token>,
        ) -> Result<Located<Self>, Located<Self::Error>> {
            let left = Atom::parse(parser)?;
            let mut pos = left.pos.clone();
            let Some(op) = parser.token() else {
                return Err(Located::new(
                    ParseError::ExpectedOperator,
                    Positon::default(),
                ));
            };
            let right = Atom::parse(parser)?;
            pos.extend(&right.pos);
            Ok(Located::new(
                Self::Binary {
                    left,
                    op: op.unwrap(),
                    right,
                },
                pos,
            ))
        }
    }
    
    #[derive(Debug)]
    pub enum AnyError {
        LexError(LexError),
        ParseError(ParseError)
    }
    impl From<LexError> for AnyError {
        fn from(value: LexError) -> Self {
            Self::LexError(value)
        }
    }
    impl From<ParseError> for AnyError {
        fn from(value: ParseError) -> Self {
            Self::ParseError(value)
        }
    }
    impl Display for AnyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AnyError::LexError(err) => err.fmt(f),
                AnyError::ParseError(err) => err.fmt(f),
            }
        }
    }
    impl Error for AnyError {}

    let ast = parse::<Token, Expression, AnyError>("1 + 2".to_string())
        .map_err(|err| err.to_string())?;
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
