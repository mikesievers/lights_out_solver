fn something() -> usize {
    42
}
mod tests {
    use super::something;
    #[test]
    fn test_something() {
        assert_eq!(something(), 42);
    }
}
