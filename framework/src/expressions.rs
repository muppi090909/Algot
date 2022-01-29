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
    Mod,
    Pow
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
            _ => panic!("invalid operator")
        }
    }
}
#[derive(Clone)]
pub struct BiIterator<T> where T: Copy + Clone {
    values: Vec<T>,
    pos: usize
}
impl<T: Copy + Clone> Iterator for BiIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        if let None = self.values.get(self.pos-1) {
            return None;
        } else if let Some(val) = self.values.get(self.pos-1) {
            Some(val.clone())
        } else {
            unreachable!()
        }
    }
}
impl<T: Copy + Clone> BiIterator<T> {
    pub fn position(&mut self, new_pos: usize) {
        self.pos = new_pos;
    }
    pub fn prev(&mut self) {
        self.pos -= 1;
    }
}
impl<T: Clone + Copy> std::convert::From<Vec<T>> for BiIterator<T> {
    fn from(input: Vec<T>) -> Self {
        Self {
            values: input,
            pos: 0
        }
    }
}
/// A list of mathematical tokens 
pub enum Token {
    RParen,
    LParen,
    Constant(f64),
    Var(String),
    Operator(Op)
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
impl Iterator for Decimal {
    type Item=f64;
    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.0)
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

