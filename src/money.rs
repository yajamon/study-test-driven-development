#[cfg(test)]
mod test {
    #[test]
    fn test_multiplication() {
        let mut five = Doller { amount: 5 };
        five.times(2);
        assert_eq!(10, five.amount)
    }
}
