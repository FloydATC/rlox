

use super::ParserPrec;


// The precedense rules should form a complete chain from None to Primary
#[test]
fn next() {
    let first = ParserPrec::None;
    let mut prec = first;
    loop {
        if prec == prec.next() { break; }
        prec = prec.next();
    }
    assert_eq!(prec, ParserPrec::Primary)
}
