#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub types);

#[test]
fn calculator1() {
    assert!(types::TermParser::new().parse("22").is_ok());
    assert!(types::TermParser::new().parse("(22)").is_ok());
    assert!(types::TermParser::new().parse("((((22))))").is_ok());
    assert!(types::TermParser::new().parse("((22)").is_err());
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}