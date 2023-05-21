

use super::test;


#[test]
fn parser_base8numbers_add() {
    for number in 0..=100 {
        let code = format!("print 0o{:04o}+0o{:04o};", number, number);
        println!("number={} code={}", number, code);
        let res = test(code.as_str());
        assert_eq!(res.is_ok(), true);
        let bytecode = res.unwrap();
        assert_eq!(bytecode.globals().count(), 0);
        assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
    }
}

#[test]
fn parser_base8numbers_subtract() {
    for number in 0..=100 {
        let code = format!("print 0o{:04o}-0o{:04o};", number, number);
        println!("number={} code={}", number, code);
        let res = test(code.as_str());
        assert_eq!(res.is_ok(), true);
        let bytecode = res.unwrap();
        assert_eq!(bytecode.globals().count(), 0);
        assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
    }
}

#[test]
fn parser_base8numbers_multiply() {
    for number in 0..=100 {
        let code = format!("print 0o{:04o}*0o{:04o};", number, number);
        println!("number={} code={}", number, code);
        let res = test(code.as_str());
        assert_eq!(res.is_ok(), true);
        let bytecode = res.unwrap();
        assert_eq!(bytecode.globals().count(), 0);
        assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
    }
}

#[test]
fn parser_base8numbers_divide() {
    for number in 0..=100 {
        let code = format!("print 0o{:04o}/0o{:04o};", number, number);
        println!("number={} code={}", number, code);
        let res = test(code.as_str());
        assert_eq!(res.is_ok(), true);
        let bytecode = res.unwrap();
        assert_eq!(bytecode.globals().count(), 0);
        assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
    }
}
