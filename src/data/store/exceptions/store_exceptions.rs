use std::fmt;

pub struct StoreImportError {
    pub message: String
}

impl fmt::Display for StoreImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not import store: {}", self.message)
    }
}

pub struct StoreConvertError {
    pub message: String
}

impl fmt::Display for StoreConvertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not convert bytes to chunks: {}", self.message)
    }
}