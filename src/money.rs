#[derive(Debug)]
pub enum Money {
    Dollar(Dollar),
}

impl Money {
    fn times(&self, multiplier: i64) -> Self {
        match self {
            Money::Dollar(dollar) => dollar.times(multiplier),
        }
    }
}
impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Money::Dollar(left), Money::Dollar(right)) => left == right,
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
}

#[derive(Debug, PartialEq)]
pub struct Franc {
    amount: i64,
}

impl Franc {
    fn new(amount: i64) -> Franc {
        Franc { amount }
    }
    fn times(&self, multiplier: i64) -> Franc {
        Franc::new(self.amount * multiplier)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert_eq!(Dollar::new(10), five.times(2));
        assert_eq!(Dollar::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_eq!(Dollar::new(5) == Dollar::new(5), true);
        assert_eq!(Dollar::new(5) == Dollar::new(6), false);
        assert_eq!(Franc::new(5) == Franc::new(5), true);
        assert_eq!(Franc::new(5) == Franc::new(6), false);
        // mismatched types!!
        // assert_eq!(Dollar::new(5) == Franc::new(5), true);
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}
