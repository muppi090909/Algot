// One line to give the program's name and a brief description.
// Copyright © 2022 Ishaan Subramanya and Gaurav Chandrashekar

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program; if not, see <http://www.gnu.org/licenses/>.

use std::{fmt, ops};

/// A wrapper around f64 type
/// eg
/// ```
/// use algot_framework::Decimal;
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
/// use algot_framework::Integer;
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
/// # use algot_framework::Op;
/// #
/// assert_eq!(Op::new('+'), Op::Add);
/// ```
/// panic on invalid token
/// ```should_panic
/// # use algot_framework::Op;
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
    Pow,
}
impl Op {
    pub fn new(token: char) -> Self {
        match token {
            '+' => Op::Add,
            '-' => Op::Sub,
            '*' => Op::Mul,
            '/' => Op::Div,
            '%' => Op::Mod,
            '^' => Op::Pow,
            _ => panic!("invalid operator"),
        }
    }
}
/// A bi-directional iteraotor
/// eg
/// create Iter from [Vec]
/// ```
/// # use algot_framework::BiIterator;
/// #
/// let mut iter = BiIterator::from(vec![1, 2, 3]);
/// ```
#[derive(Clone)]
pub struct BiIterator<T>
where
    T: Clone,
{
    values: Vec<T>,
    pos: usize,
}
impl<T: Clone> Iterator for BiIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        if let None = self.values.get(self.pos - 1) {
            return None;
        } else if let Some(val) = self.values.get(self.pos - 1) {
            Some(val.clone())
        } else {
            unreachable!()
        }
    }
}
impl<T: Clone> BiIterator<T> {
    /// Set the position of the iterator to a certain point in the values
    /// ```
    /// # use algot_framework::BiIterator;
    /// #
    /// let values = vec![1, 2, 3, 4];
    /// let mut iter = BiIterator::from(values);
    /// assert_eq!(iter.next(), Some(1));
    /// iter.position(2);
    /// assert_eq!(Some(3), iter.next())
    /// ```
    pub fn position(&mut self, new_pos: usize) {
        self.pos = new_pos;
    }
    /// go to previous element in iterator
    /// eg
    /// ```
    /// # use algot_framework::BiIterator;
    /// #
    /// let mut iter = BiIterator::from(vec![1, 2, 3]);
    /// assert_eq!(1, iter.next().unwrap());
    /// assert_eq!(2, iter.next().unwrap());
    /// assert_eq!(1, iter.prev().unwrap())
    /// ```
    pub fn prev(&mut self) -> Option<T> {
        self.pos -= 1;
        match self.values.get(self.pos - 1) {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }
    pub fn get_pos(&self) -> usize {
        self.pos
    }
    pub fn recall(&self) -> Option<&T> {
        match self.values.get(self.pos - 1) {
            Some(val) => Some(&val),
            None => None,
        }
    }
    pub fn peek(&self) -> Option<&T> {
        match self.values.get(self.pos + 1) {
            Some(val) => Some(&val),
            None => None
        }
    }
}
impl<T: Clone> std::convert::From<Vec<T>> for BiIterator<T> {
    fn from(input: Vec<T>) -> Self {
        Self {
            values: input,
            pos: 0,
        }
    }
}
/// A list of mathematical tokens
pub enum Token {
    RParen,
    LParen,
    Constant(f64),
    Var(String),
    Operator(Op),
}
impl Token {
    pub fn new(token: String) -> Self {
        match token.as_str() {
            "(" => Self::LParen,
            ")" => Self::RParen,
            "+" => Self::Operator(Op::new('+')),
            "-" => Self::Operator(Op::new('-')),
            "*" => Self::Operator(Op::new('*')),
            "/" => Self::Operator(Op::new('/')),
            "%" => Self::Operator(Op::new('%')),
            "^" => Self::Operator(Op::new('^')),
            _ if token.contains(".") => Self::Constant(token.parse::<f64>().unwrap()),
            _ => Self::Var(token),
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
