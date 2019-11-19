#[derive(Debug)]
pub enum Money {
    // amount, currency
    Dollar(i64, String),
    Franc(i64, String),
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
            Money::Dollar(amount, _) => Dollar::times(amount, multiplier),
            Money::Franc(amount, _) => Franc::times(amount, multiplier),
        }
    }
    fn currency(&self) -> &str {
        match self {
            Money::Dollar(_, currency) => currency,
            Money::Franc(_, currency) => currency,
        }
    }
}
impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Money::Dollar(lamount, _), Money::Dollar(ramount, _)) => lamount == ramount,
            (Money::Franc(lamount, _), Money::Franc(ramount, _)) => lamount == ramount,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Dollar {}

impl Dollar {
    fn new(amount: i64, currency: String) -> Money {
        Money::Dollar(amount, currency)
    }
    fn times(amount: &i64, multiplier: i64) -> Money {
        Money::dollar(amount * multiplier)
    }
}

#[derive(Debug, PartialEq)]
pub struct Franc {}

impl Franc {
    fn new(amount: i64, currency: String) -> Money {
        Money::Franc(amount, currency)
    }
    fn times(amount: &i64, multiplier: i64) -> Money {
        Money::franc(amount * multiplier)
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
