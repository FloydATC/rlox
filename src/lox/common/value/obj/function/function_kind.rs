

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum FunctionKind {
    Function,
    Initializer,
    Method,
    Script,
}


impl FunctionKind {

    pub fn has_receiver(&self) -> bool {
        return match self {
            FunctionKind::Initializer => true,
            FunctionKind::Method => true,
            _ => false,
        }
    }


    pub fn return_self(&self) -> bool {
        return match self {
            FunctionKind::Initializer => true,
            _ => false,
        }
    }


    pub fn is_toplevel(&self) -> bool {
        return match self {
            FunctionKind::Script => true,
            _ => false,
        }
    }

}

