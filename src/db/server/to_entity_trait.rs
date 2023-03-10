pub trait ToEntity<T> {
    fn to_entity(&self) -> T;
}