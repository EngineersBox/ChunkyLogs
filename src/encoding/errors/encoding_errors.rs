use std::fmt;
use std::marker::PhantomData;

pub struct EncoderError<T> {
    pub message: String,
    phantom: PhantomData<T>,
}

impl<T> EncoderError<T> {
    fn new(message: &str) -> EncoderError<T> {
        EncoderError{
            message: message.to_string(),
            phantom: PhantomData,
        }
    }
}

impl<T> fmt::Display for EncoderError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to encode {} as {}: {}",
            std::any::type_name::<Self>(),
            std::any::type_name::<T>(),
            self.message
        )
    }
}

pub struct DecoderError<T> {
    pub message: String,
    phantom: PhantomData<T>,
}

impl<T> DecoderError<T> {
    fn new(message: &str) -> DecoderError<T> {
        DecoderError{
            message: message.to_string(),
            phantom: PhantomData,
        }
    }
}

impl<T> fmt::Display for DecoderError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to decode {} into {}: {}",
            std::any::type_name::<T>(),
            std::any::type_name::<Self>(),
            self.message
        )
    }
}

pub struct TranscoderError<T> {
    pub message: String,
    phantom: PhantomData<T>,
}

impl<T> TranscoderError<T> {
    fn new(message: &str) -> TranscoderError<T> {
        TranscoderError{
            message: message.to_string(),
            phantom: PhantomData,
        }
    }
}

impl<T> fmt::Display for TranscoderError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to transcode {} to {}: {}",
            std::any::type_name::<Self>(),
            std::any::type_name::<T>(),
            self.message
        )
    }
}