

use crate::lox::common::IdentifierKind;


use super::Global;


#[test]
fn new() {
    let _ = Global::new(0, IdentifierKind::Variable);
}

#[test]
fn new_id() {
    let global = Global::new(123, IdentifierKind::Variable);
    assert_eq!(global.index(), 123);
}

#[test]
fn new_id_undefined() {
    let global = Global::new(123, IdentifierKind::Variable);
    assert_eq!(global.is_defined(), false);
}

#[test]
fn new_id_define() {
    let mut global = Global::new(123, IdentifierKind::Variable);
    global.define();
    assert_eq!(global.is_defined(), true);
}

#[test]
fn new_id_variable_mutable() {
    let global = Global::new(123, IdentifierKind::Variable);
    assert_eq!(global.is_mutable(), true);
}

#[test]
fn new_id_variable_kind() {
    let global = Global::new(123, IdentifierKind::Variable);
    assert_eq!(global.kind().as_str(), "Variable");
}

#[test]
fn new_id_constant_not_mutable() {
    let global = Global::new(123, IdentifierKind::Constant);
    assert_eq!(global.is_mutable(), false);
}

#[test]
fn new_id_constant_kind() {
    let global = Global::new(123, IdentifierKind::Constant);
    assert_eq!(global.kind().as_str(), "Constant");
}

