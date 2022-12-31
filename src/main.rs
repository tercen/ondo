mod db;

pub fn hello() -> String {
    "Hello, world!".to_string()
}

fn main() {
    println!("{}", hello());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use mockall::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }

    use mockall::*;

    pub trait Trait1 {
        fn func1(&self);
    }

    pub trait Trait2: Trait1 {
        fn func2(&self);
    }

    mock! {
        Trait3 {}
        impl Trait1 for Trait3 {
            fn func1(&self);
        }
        impl Trait2 for Trait3 {
            fn func2(&self);
        }
    }

    mod subtests {
        use super::*;

        #[test]
        fn test_mock() {
            let mut mock = MockTrait3::new();
            mock.expect_func1().returning(|| ());
            mock.expect_func2().returning(|| ());
            mock.func1();
            mock.func2();
        }
    }
    fn test_mock() {
        let mut mock = MockTrait3::new();
        mock.expect_func1().returning(|| ());
        mock.expect_func2().returning(|| ());
        mock.func1();
        mock.func2();
    }
}
