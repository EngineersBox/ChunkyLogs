pub trait Encoder<T> {
    fn encode(&self, raw: &T) -> Vec<u8>;
}

pub trait TransmuteEncoder<T,E> {
    fn encode(&self, from: &T) -> E;
}