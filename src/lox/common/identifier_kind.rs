

#[derive(Clone, Debug)]
pub enum IdentifierKind {
    Variable,
    Constant,
}


impl IdentifierKind {

    pub fn is_mutable(&self) -> bool {
        match self {
            IdentifierKind::Variable => true,
            IdentifierKind::Constant => false,
        }
    }


    pub fn as_str(&self) -> &str {
        match self {
            IdentifierKind::Variable => "Variable",
            IdentifierKind::Constant => "Constant",
        }
    }

}
