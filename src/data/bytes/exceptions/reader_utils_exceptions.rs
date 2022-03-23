use std::fmt;

pub struct HandledBufferReadError {
    pub message: String
}

impl fmt::Display for HandledBufferReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not append log entry: {}", self.message)
    }
}