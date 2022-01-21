//! A framework for number tokenizers mainly Algot
pub mod expressions {
    use std::{fmt, ops};

    /// A wrapper around i64 type
    /// eg
    /// ```
    /// # use algot_framework::expressions::Integer;
    /// #
    /// let val = Integer(5);
    /// assert_eq!(val.0, 5)
    /// ```
    #[derive(PartialEq, Clone, Copy)]
    pub struct Integer(pub i64);
    impl ops::Add<Self> for Integer {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }
    impl ops::Sub<Self> for Integer {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self(self.0 - rhs.0)
        }
    }
    impl ops::Mul<Self> for Integer {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }
    impl ops::Div<Self> for Integer {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            Self(self.0 / rhs.0)
        }
    }

    /// A wrapper around f64 type
    /// eg
    /// ```
    /// # use algot_framework::expressions::Decimal;
    /// #
    /// let val = Decimal(5.0f64);
    /// assert_eq!(val.0, 5.0f64)
    /// ```
    #[derive(PartialEq, Clone, Copy)]
    pub struct Decimal(pub f64);

    impl ops::Add<Self> for Decimal {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }
    impl ops::Sub<Self> for Decimal {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self(self.0 - rhs.0)
        }
    }
    impl ops::Mul<Self> for Decimal {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }
    impl ops::Div<Self> for Decimal {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            Self(self.0 / rhs.0)
        }
    }
    impl fmt::Display for Integer {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl fmt::Display for Decimal {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    /// A series of possible mathematical operator tokenizers
    /// eg
    /// ```
    /// # use algot_framework::expressions::Op;
    /// #
    /// assert_eq!(Op::new('+'), Op::Add);
    /// ```
    /// panic on invalid token
    /// ```should_panic
    /// # use algot_framework::expressions::Op;
    /// #
    /// Op::new('_');
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[allow(dead_code)]
    pub enum Op {
        Add,
        Sub,
        Mul,
        Div,
        Mod,
    }
    impl Op {
        pub fn new(token: char) -> Self {
            match token {
                '+' => Op::Add,
                '-' => Op::Sub,
                '*' => Op::Mul,
                '/' => Op::Div,
                '%' => Op::Mod,
                _ => panic!("invalid operator"),
            }
        }
    }
}
