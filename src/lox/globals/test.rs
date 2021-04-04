

use super::Globals;
use crate::lox::value::Value;


#[test]
fn globals_new() {
    let _var = Globals::new();
}

#[test]
fn globals_declare() {
    let name = "test".to_string();
    
    let mut var = Globals::new();
    let id = var.declare(&name).unwrap();
    assert_eq!(id, 0);
}

#[test]
fn globals_id_by_name() {
    let name = "test".to_string();
    
    let mut var = Globals::new();
    let id1 = var.declare(&name).unwrap();
    let id2 = var.id_by_name(&name).unwrap();
    assert_eq!(id1, id2);
}

#[test]
#[should_panic]
fn globals_id_by_name_none() {
    let name = "test".to_string();
    
    let mut var = Globals::new();
    let id1 = var.declare(&name).unwrap();
    let id2 = var.id_by_name("unknown").unwrap();
    assert_ne!(id1, id2);
}

#[test]
#[should_panic]
fn globals_double_declare_none() {
    let name = "test".to_string();
    
    let mut var = Globals::new();
    let id = var.declare(&name).unwrap();
    assert_eq!(id, 0);
    
    let _id = var.declare(&name)
        .expect("declare() returned None");
}

#[test]
fn globals_set_by_id() {
    let name = "test".to_string();
    let value = Value::number(123.0);
    
    let mut var = Globals::new();
    let id = var.declare(&name).unwrap();
    assert_eq!(id, 0);
    
    var.define_by_id(id, value); 
}

#[test]
fn globals_value_by_id() {
    let name = "test".to_string();
    let value1 = Value::number(123.0);
    
    let mut var = Globals::new();
    let id = var.declare(&name).unwrap();
    assert_eq!(id, 0);
    
    var.define_by_id(id, value1.clone()); 
    let value2 = var.value_by_id(id).unwrap();
    assert_eq!(&value1, value2);
}

#[test]
#[should_panic]
fn globals_value_by_id_none() {
    let name = "test".to_string();
    
    let mut var = Globals::new();
    let id = var.declare(&name).unwrap();
    assert_eq!(id, 0);
    
    let _value = var.value_by_id(id).unwrap();
}

#[test]
fn globals_name_by_id() {
    let name = "test".to_string();
    
    let mut var = Globals::new();
    let id = var.declare(&name).unwrap();
    assert_eq!(id, 0);
    
    let name = var.name_by_id(id);
    assert_eq!(name, "test");
}

#[test]
#[should_panic]
fn globals_name_by_id_panic() {
    let name = "test".to_string();
    
    let mut var = Globals::new();
    let id = var.declare(&name).unwrap();
    assert_eq!(id, 0);
    
    let _name = var.name_by_id(id+1);
}

#[test]
fn globals_strings() {
    let name1 = "foo".to_string();
    let name2 = "bar".to_string();
    
    let mut var = Globals::new();
    let id1 = var.declare(&name1).unwrap();
    var.define_by_id(id1, Value::string("upper"));
    let id2 = var.declare(&name2).unwrap();
    var.define_by_id(id2, Value::string("lower"));
    
    let value3 = var.value_by_id(id1).unwrap();
    let value4 = var.value_by_id(id2).unwrap();
    assert_eq!(&Value::string("upper"), value3);
    assert_eq!(&Value::string("lower"), value4);
}



