

use super::Global;


#[test]
fn new() {
    let _ = Global::new(0);
}

#[test]
fn new_id() {
    let global = Global::new(123);
    assert_eq!(global.index(), 123);
}

#[test]
fn new_id_undefined() {
    let global = Global::new(123);
    assert_eq!(global.is_defined(), false);
}

#[test]
fn new_id_define() {
    let mut global = Global::new(123);
    global.define();
    assert_eq!(global.is_defined(), true);
}

