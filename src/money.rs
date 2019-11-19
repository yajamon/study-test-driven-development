#[derive(Debug)]
pub enum Money {
    // amount, currency
    Money(i64, String),
    Dollar(i64, String),
    Franc(i64, String),
}

impl Money {
    fn dollar(amount: i64) -> Money {
        Money::Dollar(amount, "USD".to_string())
    }
    fn franc(amount: i64) -> Money {
        Money::Franc(amount, "CHF".to_string())
    }
    fn times(&self, multiplier: i64) -> Self {
        match self {
            Money::Money(_, _) => Money::Money(0, String::new()),
            Money::Dollar(amount, currency) => Dollar::times(amount, multiplier, &currency),
            Money::Franc(amount, currency) => Franc::times(amount, multiplier, &currency),
        }
    }
    fn currency(&self) -> &str {
        match self {
            Money::Money(_, _) => "",
            Money::Dollar(_, currency) => currency,
            Money::Franc(_, currency) => currency,
        }
    }
}
impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        let left = match self {
            Money::Dollar(amount, currency) => (amount, currency),
            Money::Franc(amount, currency) => (amount, currency),
            Money::Money(amount, currency) => (amount, currency),
        };
        let right = match other {
            Money::Dollar(amount, currency) => (amount, currency),
            Money::Franc(amount, currency) => (amount, currency),
            Money::Money(amount, currency) => (amount, currency),
        };
        left == right
    }
}

#[derive(Debug, PartialEq)]
pub struct Dollar {}

impl Dollar {
    fn times(amount: &i64, multiplier: i64, currency: &str) -> Money {
        Money::Money(amount * multiplier, currency.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct Franc {}

impl Franc {
    fn times(amount: &i64, multiplier: i64, currency: &str) -> Money {
        Money::Money(amount * multiplier, currency.to_string())
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
    fn test_different_type_equality() {
        assert_eq!(
            Money::Money(10, "CHF".to_string()) == Money::Franc(10, "CHF".to_string()),
            true
        );
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
