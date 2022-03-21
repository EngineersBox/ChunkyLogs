use std::fmt;

pub enum GroupErrors {
    GroupEntryAppendError(GroupEntryAppendError),
    GroupChunkProcessingError(GroupChunkProcessingError),
}

pub struct GroupEntryAppendError {
    pub message: String
}

impl fmt::Display for GroupEntryAppendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not append log entry: {}", self.message)
    }
}

pub struct GroupChunkProcessingError {
    pub message: String
}

impl fmt::Display for GroupChunkProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could process chunk into log group: {}", self.message)
    }
}