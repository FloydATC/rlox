

use crate::lox::common::IdentifierKind;


use super::Local;


#[test]
fn new_var() {
    let _ = Local::new("name", 0, IdentifierKind::Variable);
}

#[test]
fn var_is_not_defined_by_default() {
    let local = Local::new("name", 0, IdentifierKind::Variable);
    assert_eq!(local.is_defined(), false);
}

#[test]
fn var_is_not_captured_by_default() {
    let local = Local::new("name", 0, IdentifierKind::Variable);
    assert_eq!(local.is_captured(), false);
}

#[test]
fn var_is_mutable() {
    let local = Local::new("name", 0, IdentifierKind::Variable);
    assert_eq!(local.is_mutable(), true);
}

#[test]
fn var_kind() {
    let local = Local::new("name", 0, IdentifierKind::Variable);
    assert_eq!(local.kind().as_str(), "Variable");
}


#[test]
fn new_const() {
    let _ = Local::new("name", 0, IdentifierKind::Constant);
}

#[test]
fn const_is_not_defined_by_default() {
    let local = Local::new("name", 0, IdentifierKind::Constant);
    assert_eq!(local.is_defined(), false);
}

#[test]
fn const_is_not_captured_by_default() {
    let local = Local::new("name", 0, IdentifierKind::Constant);
    assert_eq!(local.is_captured(), false);
}

#[test]
fn const_is_not_mutable() {
    let local = Local::new("name", 0, IdentifierKind::Constant);
    assert_eq!(local.is_mutable(), false);
}

#[test]
fn const_kind() {
    let local = Local::new("name", 0, IdentifierKind::Constant);
    assert_eq!(local.kind().as_str(), "Constant");
}


