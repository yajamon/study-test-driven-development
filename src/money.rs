pub trait Expression {}

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
    pub fn plus<'a>(&'a self, addend: &'a Money) -> Sum<'a> {
        Sum::new(self, addend)
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
}

impl Expression for Money {}

pub struct Bank {}

impl Bank {
    fn new() -> Bank {
        Bank {}
    }
    fn reduce(&self, source: &Sum, to: &str) -> Money {
        let amount = source.augend.amount + source.addend.amount;
        Money::new(amount, to.to_string())
    }
}

pub struct Sum<'a> {
    augend: &'a Money,
    addend: &'a Money,
}
impl<'a> Sum<'a> {
    pub fn new(augend: &'a Money, addend: &'a Money) -> Sum<'a> {
        Sum { augend, addend }
    }
}
impl<'a> Expression for Sum<'a> {}

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
        let five = Money::dollar(5);
        let sum = five.plus(&five);
        let bank = Bank::new();
        let reduced = bank.reduce(&sum, "USD");
        assert_eq!(Money::dollar(10), reduced);
    }

    #[test]
    fn test_plus_return_sum() {
        let five = Money::dollar(5);
        let sum: Sum = five.plus(&five);
        assert_eq!(&five, sum.augend);
        assert_eq!(&five, sum.addend);
    }

    #[test]
    fn test_reduce_sum() {
        let three = Money::dollar(3);
        let four = Money::dollar(4);
        let sum = Sum::new(&three, &four);
        let bank = Bank::new();
        let result = bank.reduce(&sum, "USD");
        assert_eq!(Money::dollar(7), result);
    }
}
