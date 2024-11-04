fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_something() {
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn test_something_ignored() {
        assert_eq!(2, 2);
    }
}
