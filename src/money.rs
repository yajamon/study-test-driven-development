#[derive(Debug)]
pub enum Money {
    // amount, currency
    Money(i64, String),
    Franc(i64, String),
}

impl Money {
    fn dollar(amount: i64) -> Money {
        Money::Money(amount, "USD".to_string())
    }
    fn franc(amount: i64) -> Money {
        Money::Money(amount, "CHF".to_string())
    }
    fn times(&self, multiplier: i64) -> Self {
        match self {
            Money::Money(amount, currency) => {
                Money::Money(amount * multiplier, currency.to_string())
            }
            Money::Franc(amount, currency) => {
                Money::Money(amount * multiplier, currency.to_string())
            }
        }
    }
    fn currency(&self) -> &str {
        match self {
            Money::Money(_, currency) => currency,
            Money::Franc(_, currency) => currency,
        }
    }
}
impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        let left = match self {
            Money::Franc(amount, currency) => (amount, currency),
            Money::Money(amount, currency) => (amount, currency),
        };
        let right = match other {
            Money::Franc(amount, currency) => (amount, currency),
            Money::Money(amount, currency) => (amount, currency),
        };
        left == right
    }
}

#[derive(Debug, PartialEq)]
pub struct Dollar {}

impl Dollar {}

#[derive(Debug, PartialEq)]
pub struct Franc {}

impl Franc {}

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
