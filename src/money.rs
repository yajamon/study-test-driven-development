use std::collections::HashMap;

pub trait Expression {
    fn reduce(&self, bank: &Bank, to: &str) -> Money;
}

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

impl Expression for Money {
    fn reduce(&self, bank: &Bank, to: &str) -> Money {
        let rate = bank.rate(&self.currency, to);
        Money::new(self.amount / rate, to.to_string())
    }
}

pub struct Bank {
    rates: HashMap<Pair, i64>,
}

impl Bank {
    fn new() -> Bank {
        Bank {
            rates: HashMap::new(),
        }
    }
    fn reduce(&self, source: &impl Expression, to: &str) -> Money {
        source.reduce(self, to)
    }
    fn add_rate(&mut self, from: &str, to: &str, rate: i64) {
        self.rates
            .insert(Pair::new(from.to_string(), to.to_string()), rate);
    }

    fn rate(&self, from: &str, to: &str) -> i64 {
        let pair = Pair::new(from.to_string(), to.to_string());
        self.rates.get(&pair).unwrap_or(&0).clone()
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
impl<'a> Expression for Sum<'a> {
    fn reduce(&self, bank: &Bank, to: &str) -> Money {
        let amount = self.augend.amount + self.addend.amount;
        Money::new(amount, to.to_string())
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Pair {
    from: String,
    to: String,
}

impl Pair {
    fn new(from: String, to: String) -> Pair {
        Pair { from, to }
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

    #[test]
    fn test_reduce_money() {
        let bank = Bank::new();
        let result = bank.reduce(&Money::dollar(1), "USD");
        assert_eq!(Money::dollar(1), result);
    }

    #[test]
    fn test_reduce_money_different_currency() {
        let bank = &mut Bank::new();
        bank.add_rate("CHF", "USD", 2);
        let result = bank.reduce(&Money::franc(2), "USD");
        assert_eq!(Money::dollar(1), result);
    }

    #[test]
    fn test_identity_rate() {
        let bank = Bank::new();
        assert_eq!(1, bank.rate("USD", "USD"));
    }
}
