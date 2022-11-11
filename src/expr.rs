use nom::{
    branch::alt,
    multi::many0,
    bytes::complete::{ take_while1, take_while_m_n }
};

pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut r = many0::<_, _, nom::error::Error<&str>, _>(
        alt((
            take_while1(|c: char| c.is_numeric() || c == ',' || c == '.'),
            take_while_m_n(1, 1, |c: char| ['+', '-', '/', '*', '%', '(', ')'].contains(&c)),
            take_while1(|c: char| c.is_alphabetic()),
            take_while1(|c: char| c == ' ')
        ))
    )(s);

    match r {
        Ok((_, ref mut v)) => {
            v.retain(|&x| x != " ");
            return Ok(v.to_vec().into_iter().map(|x| coerce(x)).collect());
        },
        Err(n) => {
            return Err(format!("Unable to tokenize: {}", n));
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    OpenParens,
    CloseParens,
    Sin,
    Cos,
    Tan,
    Sec,
    Csc,
    Arctan
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Unsigned(u128),
    Signed(i128),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Operator(Operator),
    Literal(Literal),
    Variable(&'a str)
}

pub fn coerce(s: &str) -> Token { 
    if let Ok(n) = s.parse::<u128>() {
        return Token::Literal(Literal::Unsigned(n));
    }
    
    if let Ok(n) = s.parse::<i128>() {
        return Token::Literal(Literal::Signed(n));
    }
    
    if let Ok(n) = s.parse::<f64>() {
        return Token::Literal(Literal::Float(n));
    }

    match s.to_lowercase().as_str() {
        "+" => { Token::Operator(Operator::Add) },
        "-" => { Token::Operator(Operator::Sub) },
        "*" => { Token::Operator(Operator::Mul) },
        "/" => { Token::Operator(Operator::Div) },
        "%" => { Token::Operator(Operator::Mod) },
        "^" => { Token::Operator(Operator::Exp) },
        "(" => { Token::Operator(Operator::OpenParens) },
        ")" => { Token::Operator(Operator::CloseParens) },
        "sin" => { Token::Operator(Operator::Sin) },
        "cos" => { Token::Operator(Operator::Cos) },
        "tan" => { Token::Operator(Operator::Tan) },
        "sec" => { Token::Operator(Operator::Sec) },
        "csc" => { Token::Operator(Operator::Csc) },
        "arctan" => { Token::Operator(Operator::Arctan) },
        _ => { Token::Variable(s) }
    }
}
