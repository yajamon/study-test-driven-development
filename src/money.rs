#[derive(Debug)]
pub enum Money {
    // currency, inner
    Dollar(String, Dollar),
    Franc(String, Franc),
}

impl Money {
    fn dollar(amount: i64) -> Money {
        Dollar::new(amount, "USD".to_string())
    }
    fn franc(amount: i64) -> Money {
        Franc::new(amount, "CHF".to_string())
    }
    fn times(&self, multiplier: i64) -> Self {
        match self {
            Money::Dollar(_, dollar) => dollar.times(dollar.amount, multiplier),
            Money::Franc(_, franc) => franc.times(multiplier),
        }
    }
    fn currency(&self) -> &str {
        match self {
            Money::Dollar(currency, _) => currency,
            Money::Franc(currency, _) => currency,
        }
    }
}
impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Money::Dollar(_, left), Money::Dollar(_, right)) => left == right,
            (Money::Franc(_, left), Money::Franc(_, right)) => left == right,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Dollar {
    amount: i64,
}

impl Dollar {
    fn new(amount: i64, currency: String) -> Money {
        Money::Dollar(currency, Dollar { amount })
    }
    fn times(&self, amount: i64, multiplier: i64) -> Money {
        Money::dollar(amount * multiplier)
    }
}

#[derive(Debug, PartialEq)]
pub struct Franc {
    amount: i64,
}

impl Franc {
    fn new(amount: i64, currency: String) -> Money {
        Money::Franc(currency, Franc { amount })
    }
    fn times(&self, multiplier: i64) -> Money {
        Money::franc(self.amount * multiplier)
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
        assert_eq!(Money::franc(5) == Money::franc(5), true);
        assert_eq!(Money::franc(5) == Money::franc(6), false);
        assert_eq!(Money::dollar(5) == Money::franc(5), false);
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Money::franc(5);
        assert_eq!(Money::franc(10), five.times(2));
        assert_eq!(Money::franc(15), five.times(3));
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }
}
