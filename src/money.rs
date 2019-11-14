pub struct Dollar {}

impl Dollar {
    fn new(amount: i64) -> Dollar {
        Dollar {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiplication() {
        let mut five = Dollar::new(5);
        five.times(2);
        assert_eq!(10, five.amount)
    }
}
