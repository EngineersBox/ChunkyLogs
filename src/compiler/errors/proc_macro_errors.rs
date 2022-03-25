use std::fmt;

#[derive(Debug)]
pub struct StructFieldNotFoundError {
    pub struct_name: String,
    pub field_name: String
}

impl fmt::Display for StructFieldNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not find field {} in struct {}",
            self.field_name,
            self.struct_name
        )
    }
}

#[derive(Debug)]
pub struct TypedAttributeRetrievalError {
    pub message: String,
}

impl fmt::Display for TypedAttributeRetrievalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not parse attribute into concrete type: {}",
            self.message,
        )
    }
}