pub struct Dollar {
    amount: i64,
}

impl Dollar {
    fn new(amount: i64) -> Dollar {
        Dollar { amount }
    }
    fn times(&self, multiplier: i64) -> Dollar {
        Dollar::new(self.amount * multiplier)
    }
    fn equals(&self, other: Dollar) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        let mut product = five.times(2);
        assert_eq!(10, product.amount);
        product = five.times(3);
        assert_eq!(15, product.amount);
    }

    #[test]
    fn test_equality() {
        assert_eq!(Dollar::new(5).equals(Dollar::new(5)), true);
        assert_eq!(Dollar::new(5).equals(Dollar::new(6)), false);
    }
}
