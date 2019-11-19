#[derive(Debug)]
pub enum Money {
    Dollar(Dollar),
    Franc(Franc),
}

impl Money {
    fn dollar(amount: i64) -> Money {
        Dollar::new(amount)
    }
    fn franc(amount: i64) -> Money {
        Franc::new(amount)
    }
    fn times(&self, multiplier: i64) -> Self {
        match self {
            Money::Dollar(dollar) => dollar.times(multiplier),
            Money::Franc(franc) => franc.times(multiplier),
        }
    }
    fn currency(&self) -> &str {
        match self {
            Money::Dollar(dollar) => dollar.currency(),
            Money::Franc(franc) => franc.currency(),
        }
    }
}
impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Money::Dollar(left), Money::Dollar(right)) => left == right,
            (Money::Franc(left), Money::Franc(right)) => left == right,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Dollar {
    amount: i64,
}

impl Dollar {
    fn new(amount: i64) -> Money {
        Money::Dollar(Dollar { amount })
    }
    fn times(&self, multiplier: i64) -> Money {
        Dollar::new(self.amount * multiplier)
    }
    fn currency(&self) -> &str {
        "USD"
    }
}

#[derive(Debug, PartialEq)]
pub struct Franc {
    amount: i64,
    currency: String,
}

impl Franc {
    fn new(amount: i64) -> Money {
        Money::Franc(Franc {
            amount,
            currency: "CHF".to_string(),
        })
    }
    fn times(&self, multiplier: i64) -> Money {
        Franc::new(self.amount * multiplier)
    }
    fn currency(&self) -> &str {
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
