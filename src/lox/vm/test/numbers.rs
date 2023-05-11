

use super::compile_and_execute;

#[test]
fn vm_base2numbers() {
    for number in 0..=100 {
        let code = format!("exit 0b{:08b};", number);
        println!("number={} code={}", number, code);
        let res = compile_and_execute(code.as_str());
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), number);
    }
}

#[test]
fn vm_base8numbers() {
    for number in 0..=100 {
        let code = format!("exit 0o{:04o};", number);
        println!("number={} code={}", number, code);
        let res = compile_and_execute(code.as_str());
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), number);
    }
}

#[test]
fn vm_base10numbers() {
    for number in 0..=100 {
        let code = format!("exit {};", number);
        println!("number={} code={}", number, code);
        let res = compile_and_execute(code.as_str());
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), number);
    }
}

#[test]
fn vm_base16numbers() {
    for number in 0..=100 {
        let code = format!("exit 0x{:02x};", number);
        println!("number={} code={}", number, code);
        let res = compile_and_execute(code.as_str());
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), number);
    }
}

