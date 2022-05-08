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

lalrpop_mod!(pub types_short);

#[test]
fn calculator2() {
    assert!(types_short::TermNumberParser::new().parse("22").is_ok());
    assert!(types_short::TermNumberParser::new().parse("(22)").is_ok());
    assert!(types_short::TermNumberParser::new().parse("212").is_ok());
    assert!(types_short::TermNumberParser::new().parse("((((22))))").is_ok());
    assert!(types_short::TermNumberParser::new().parse("(((((((22)))))))").is_ok());
    assert!(types_short::TermNumberParser::new().parse("((22)").is_err());
}

lalrpop_mod!(pub types_express);

#[test]
fn calculator3() {
    assert!(types_express::TermNumParser::new().parse("22").is_ok());
    assert!(types_express::TermNumParser::new().parse("(22)").is_ok());
    assert!(types_express::TermNumParser::new().parse("212").is_ok());
    assert!(types_express::TermNumParser::new().parse("((((22))))").is_ok());
    assert!(types_express::TermNumParser::new().parse("(((((((22)))))))").is_ok());
    assert!(types_express::TermNumParser::new().parse("((22)").is_err());
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}