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
    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}