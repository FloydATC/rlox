use super::test;


#[test]
fn parser_base2numbers_add() {
    for number in 0..=100 {
        let code = format!("print 0b{:08b}+0b{:08b};", number, number);
        println!("number={} code={}", number, code);
        let res = test(code.as_str());
        assert_eq!(res.is_ok(), true);
        let bytecode = res.unwrap();
        assert_eq!(bytecode.globals().count(), 0);
        assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
    }
}

#[test]
fn parser_base2numbers_subtract() {
    for number in 0..=100 {
        let code = format!("print 0b{:08b}-0b{:08b};", number, number);
        println!("number={} code={}", number, code);
        let res = test(code.as_str());
        assert_eq!(res.is_ok(), true);
        let bytecode = res.unwrap();
        assert_eq!(bytecode.globals().count(), 0);
        assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
    }
}

#[test]
fn parser_base2numbers_multiply() {
    for number in 0..=100 {
        let code = format!("print 0b{:08b}*0b{:08b};", number, number);
        println!("number={} code={}", number, code);
        let res = test(code.as_str());
        assert_eq!(res.is_ok(), true);
        let bytecode = res.unwrap();
        assert_eq!(bytecode.globals().count(), 0);
        assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
    }
}

#[test]
fn parser_base2numbers_divide() {
    for number in 0..=100 {
        let code = format!("print 0b{:08b}/0b{:08b};", number, number);
        println!("number={} code={}", number, code);
        let res = test(code.as_str());
        assert_eq!(res.is_ok(), true);
        let bytecode = res.unwrap();
        assert_eq!(bytecode.globals().count(), 0);
        assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
    }
}
