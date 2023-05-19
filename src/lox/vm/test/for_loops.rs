

use super::compile_and_execute;


#[test]
fn for_loop_no_conditional_is_true() {
    let code = "for (;;) { exit 1; } exit 0;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_true_conditional() {
    let code = "for (; true;) { exit 1; } exit 0;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_false_conditional() {
    let code = "for (; false;) { exit 0; } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_local_var() {
    let code = "for (var i=1;;) { exit i; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_local_var_shadows_global() {
    let code = "var i=0; for (var i=1;;) { exit i; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_using_global_var() {
    let code = "var i=0; for (i=1;;) { exit i; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_count_up() {
    let code = "var i; for (i=0; i<5; i=i+1) {} exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 5);
}

#[test]
fn for_loop_count_down() {
    let code = "var i; for (i=5; i>0; i=i-1) {} exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

// Iterator-style loops using FOR..IN

#[test]
fn for_in_list_using_global() {
    let code = "var i; var sum=0; for i in (1,2,4) { sum=sum+i; } exit sum;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn for_in_list_using_local() {
    let code = "var sum=0; for var i in (1,2,4) { sum=sum+i; } exit sum;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn for_in_empty_list_using_global() {
    let code = "var i; var sum=7; for i in () { sum=sum+i; } exit sum;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn for_in_empty_list_using_local() {
    let code = "var sum=7; for var i in () { sum=sum+i; } exit sum;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn for_in_array_using_global() {
    let code = "var a=[1,2,4]; var i; var sum=0; for i in a { sum=sum+i; } exit sum;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn for_in_array_using_local() {
    let code = "var a=[1,2,4]; var sum=0; for var i in a { sum=sum+i; } exit sum;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn for_in_instance_using_global() {
    let code = "
        class c { 
            init() { 
                this.i=1; 
            } 
            next(last) { 
                if (last is null) 
                    return this.i; 
                last=last*2; 
                if (last > 4) 
                    return null; 
                else 
                    return last; 
            } 
        } 
        var ix=c(); 
        var i; 
        var sum=0; 
        for i in ix { sum=sum+i; } 
        exit sum;
    ";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn for_in_instance_using_local() {
    let code = "
        class c { 
            init() { 
                this.i=1; 
            } 
            next(last) { 
                if (last is null) 
                    return this.i; 
                last=last*2; 
                if (last > 4) 
                    return null; 
                else 
                    return last; 
            } 
        } 
        var ix=c(); 
        var sum=0; 
        for var i in ix { sum=sum+i; } 
        exit sum;
    ";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

