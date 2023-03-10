pub fn hello() -> String {
    "Hello, world!".to_owned()
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    #[ignore]
    fn test_dummy_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}