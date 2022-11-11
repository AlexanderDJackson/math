mod expr;
use crate::expr::*;

fn main() {
    /*
    assert_eq!(coerce("4"), Ok(Token::Literal(Literal::Unsigned(4))));
    assert_eq!(coerce("4.1"), Ok(Token::Literal(Literal::Float(4.1))));
    assert_eq!(coerce("-4"), Ok(Token::Literal(Literal::Signed(-4))));
    assert_eq!(coerce("*"), Ok(Token::Operator(Operator::Mul)));
    assert_eq!(coerce("sin"), Ok(Token::Operator(Operator::Sin)));
    assert_eq!(tokenize("a / (7 + sin(x))"), Ok(vec!["a", "/", "(", "7", "+", "sin", "(", "x", ")", ")"]));
    */

    let v = vec![
        "a + (7 - sin(x))",
        "alpha + (7.0 - xray)",
        "7 / 8,902.002 + 8"
    ];

    for i in v {
        println!("{:#?}", tokenize(i));
    }
}
