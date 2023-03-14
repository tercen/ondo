pub trait ToEntity<T> {
    fn to_entity(&self) -> T;
}

pub trait FromEntity<T> {
    fn from_entity(entity: T) -> Self;
}