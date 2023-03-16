pub trait ToReference<T> {
    fn to_reference(&self) -> T;
}

pub trait FromReference<T> {
    fn from_reference(reference: T) -> Self;
}