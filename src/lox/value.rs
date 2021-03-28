
use std::rc::Rc;

use super::obj::Obj;


#[allow(dead_code)]
pub enum Value {
  NullValue,
  BoolValue(bool),
  NumValue(f64),
  ObjValue(Rc<Obj>),
}

