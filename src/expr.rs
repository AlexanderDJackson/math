use nom::{
    Needed,
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
            return Ok(v.to_vec().iter(|&x| coerce(x)));
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
pub enum Token {
    Operator(Operator),
    Literal(Literal),
}

pub fn coerce(s: &str) -> Result<Token, &str> { 
    if let Ok(n) = s.parse::<u128>() {
        return Ok(Token::Literal(Literal::Unsigned(n)));
    }
    
    if let Ok(n) = s.parse::<i128>() {
        return Ok(Token::Literal(Literal::Signed(n)));
    }
    
    if let Ok(n) = s.parse::<f64>() {
        return Ok(Token::Literal(Literal::Float(n)));
    }

    match s.to_lowercase().as_str() {
        "+" => { Ok(Token::Operator(Operator::Add)) },
        "-" => { Ok(Token::Operator(Operator::Sub)) },
        "*" => { Ok(Token::Operator(Operator::Mul)) },
        "/" => { Ok(Token::Operator(Operator::Div)) },
        "%" => { Ok(Token::Operator(Operator::Mod)) },
        "^" => { Ok(Token::Operator(Operator::Exp)) },
        "sin" => { Ok(Token::Operator(Operator::Sin)) },
        "cos" => { Ok(Token::Operator(Operator::Cos)) },
        "tan" => { Ok(Token::Operator(Operator::Tan)) },
        "sec" => { Ok(Token::Operator(Operator::Sec)) },
        "csc" => { Ok(Token::Operator(Operator::Csc)) },
        "arctan" => { Ok(Token::Operator(Operator::Arctan)) },
        _ => { Err("Unable to parse value") }
    }
}
