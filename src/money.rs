pub struct Dollar {
    amount: i64,
}

impl Dollar {
    fn new(amount: i64) -> Dollar {
        Dollar { amount }
    }
    fn times(&mut self, multiplier: i64) {
        self.amount *= multiplier;
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
        product.times(3);
        assert_eq!(15, product.amount);
    }
}
