pub trait Decoder<T> {
    fn decode(&self, raw: &Vec<u8>) -> T;
}

pub trait TransmuteDecoder<T,E> {
    fn decode(&self, from: &T) -> E;
}