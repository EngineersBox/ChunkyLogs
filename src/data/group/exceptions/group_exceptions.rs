use std::fmt;

pub struct GroupEntryAppendError {
    pub message: String
}

impl fmt::Display for GroupEntryAppendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not append log entry: {}", self.message)
    }
}