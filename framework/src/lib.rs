//! A framework for number tokenizers mainly Algot
pub mod expressions {
    use std::{fmt, ops};
    
    /// A wrapper around f64 type
    /// eg
    /// ```
    /// use algot_framework::expressions::Decimal;
    ///
    /// let decimal = Decimal(1.0);
    /// assert_eq!(decimal, Decimal(1.0));
    /// assert_eq!(decimal.0, 1.0f64);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Decimal(pub f64);
    /// A wrapper around i64 type
    /// eg
    /// ```
    /// use algot_framework::expressions::Integer;
    ///
    /// let integer = Integer(1);
    /// assert_eq!(integer, Integer(1));
    /// assert_eq!(integer.0, 1);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Integer(pub i64);
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
        Mod
    }
    impl Op {
        pub fn new(token: char) -> Self {
            match token {
                '+' => Op::Add,
                '-' => Op::Sub,
                '*' => Op::Mul,
                '/' => Op::Div,
                '%' => Op::Mod,
                _ => panic!("invalid operator")
            }
        }
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
