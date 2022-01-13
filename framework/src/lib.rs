pub mod expressions {
    use std::{fmt, ops};

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Decimal(pub f64);
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Integer(pub i64);
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[allow(dead_code)]
    enum Op {
        Add,
        Sub,
        Mul,
        Dev,
    }
    impl fmt::Display for Decimal {
        fn fmt(self: &Decimal, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl fmt::Display for Integer {
        fn fmt(self: &Integer, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl ops::Add<Decimal> for Decimal {
        fn add(self, rhs: Decimal) -> Self::Output {
            Decimal(self.0 + rhs.0)
        }

        type Output = Decimal;
    }
    impl ops::Sub<Decimal> for Decimal {
        fn sub(self, rhs: Decimal) -> Self::Output {
            Decimal(self.0 - rhs.0)
        }

        type Output = Decimal;
    }
    impl ops::Mul<Decimal> for Decimal {
        type Output = Decimal;

        fn mul(self, rhs: Decimal) -> Self::Output {
            Decimal(self.0 * rhs.0)
        }
    }
    impl ops::Div<Decimal> for Decimal {
        type Output = Decimal;

        fn div(self, rhs: Decimal) -> Self::Output {
            Decimal(self.0 / rhs.0)
        }
    }
}
