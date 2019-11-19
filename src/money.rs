#[derive(Debug, PartialEq)]
pub struct Money {
    amount: i64,
    currency: String,
}

impl Money {
    fn new(amount: i64, currency: String) -> Money {
        Money { amount, currency }
    }
    pub fn dollar(amount: i64) -> Money {
        Money::new(amount, "USD".to_string())
    }
    pub fn franc(amount: i64) -> Money {
        Money::new(amount, "CHF".to_string())
    }
    pub fn times(&self, multiplier: i64) -> Self {
        Money::new(self.amount * multiplier, self.currency.to_string())
    }
    pub fn plus(&self, addend: &Money) -> Self {
        Money::new(self.amount + addend.amount, self.currency.to_string())
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_eq!(Money::dollar(5) == Money::dollar(5), true);
        assert_eq!(Money::dollar(5) == Money::dollar(6), false);
        assert_eq!(Money::dollar(5) == Money::franc(5), false);
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }

    #[test]
    fn test_simple_addition() {
        let sum = Money::dollar(5).plus(&Money::dollar(5));
        assert_eq!(Money::dollar(10), sum);
    }
}
