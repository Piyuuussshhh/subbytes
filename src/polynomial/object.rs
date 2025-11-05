#![allow(non_snake_case)]
use comfy_table::Table;
use std::{
    collections::{BTreeSet, HashMap},
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Debug, PartialEq)]
pub struct GF2NPolynomial {
    pub degree: u32,
    // Contains the powers of all the terms of the GF2Npolynomial with coefficient 1.
    pub terms: Vec<u8>,
}

impl GF2NPolynomial {
    pub fn new(terms: Vec<u8>) -> Self {
        Self { degree: 0, terms }.fix_terms()
    }

    pub fn from_byte(byte: u8) -> Self {
        let terms = (0..8)
            .rev()
            .filter_map(|i| {
                if (byte >> i) & 1 == 1 {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        Self { degree: 0, terms }.fix_terms()
    }

    pub fn zero() -> Self {
        Self {
            degree: 0,
            terms: vec![],
        }
    }

    pub fn one() -> Self {
        Self {
            degree: 0,
            // Because x^0 = 1
            terms: vec![0],
        }
    }

    /// Calculated using the Extended Euclidean Algorithm.
    pub fn inverse(&self, irreducible_polynomial: &GF2NPolynomial) -> Self {
        let mut A = irreducible_polynomial.clone();
        let mut B = self.clone();
        let mut T1 = GF2NPolynomial::zero();
        let mut T2 = GF2NPolynomial::one();

        // let mut table = Table::new();
        // table.set_header(vec!["Q", "A", "B", "R", "T1", "T2", "S"]);
        while !B.terms.is_empty() {
            let (Q, R) = A.clone() / B.clone();
            let S = T1.clone() - Q.clone() * T2.clone();
            // table.add_row(vec![
            //     Q.algebraic_string(),
            //     A.algebraic_string(),
            //     B.algebraic_string(),
            //     R.algebraic_string(),
            //     T1.algebraic_string(),
            //     T2.algebraic_string(),
            //     S.algebraic_string(),
            // ]);
            // Preparing next round of EEA
            A = B;
            B = R;
            T1 = T2;
            T2 = S;
        }

        // println!("{table}");

        T1
    }

    pub fn algebraic_string(&self) -> String {
        if self.terms.is_empty() {
            return String::from("0");
        }

        self.terms
            .iter()
            .map(|&power| {
                if power == 0 {
                    "1".to_string()
                } else if power == 1 {
                    "x".to_string()
                } else {
                    format!("x^{}", power)
                }
            })
            .collect::<Vec<String>>()
            .join(" + ")
    }

    pub fn hex_string(&self) -> String {
        let mut byte = 0u8;
        for &power in &self.terms {
            byte |= 1 << power;
        }
        format!("{:02X}", byte)
    }

    fn fix_terms(mut self) -> Self {
        self.dedup();
        self.terms.sort_unstable();
        self.terms.reverse();

        if let Some(&max_power) = self.terms.first() {
            self.degree = max_power as u32;
        } else {
            self.degree = 0;
        }

        self
    }

    fn dedup(&mut self) {
        let mut counts = HashMap::new();
        for &number in self.terms.iter() {
            *counts.entry(number).or_insert(0) += 1;
        }
        self.terms.clear();

        for (number, count) in counts {
            if count % 2 != 0 {
                self.terms.push(number);
            }
        }
    }

    fn xor(&self, rhs: &Self) -> Self {
        let set_lhs: BTreeSet<_> = self.terms.iter().copied().collect();
        let set_rhs: BTreeSet<_> = rhs.terms.iter().copied().collect();

        let sum = set_lhs
            .symmetric_difference(&set_rhs)
            .copied()
            .collect::<Vec<u8>>();

        if sum.is_empty() {
            return GF2NPolynomial::zero();
        }

        GF2NPolynomial::new(sum)
    }
}

impl Into<u8> for GF2NPolynomial {
    fn into(self) -> u8 {
        let mut res: u8 = 0;
        for term in self.terms {
            res += 2u8.pow(term as u32);
        }

        res
    }
}

impl Add for GF2NPolynomial {
    type Output = GF2NPolynomial;

    fn add(self, rhs: Self) -> Self::Output {
        self.xor(&rhs)
    }
}

impl Sub for GF2NPolynomial {
    type Output = GF2NPolynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        self.xor(&rhs)
    }
}

impl Mul for GF2NPolynomial {
    type Output = GF2NPolynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.terms.is_empty() || rhs.terms.is_empty() {
            return GF2NPolynomial::zero();
        }
        let mut cross_product = Vec::new();
        for &p1 in self.terms.iter() {
            for &p2 in rhs.terms.iter() {
                cross_product.push(p1 + p2);
            }
        }

        GF2NPolynomial::new(cross_product)
    }
}

impl Div for GF2NPolynomial {
    type Output = (GF2NPolynomial, GF2NPolynomial);

    /// Returns a tuple of (quotient, remainder)
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.terms.is_empty() {
            panic!("Division by 0 polynomial");
        }
        // If dividing a polynomial by a polynomial of higher degree, return (q = 0, r = dividend)
        if rhs.degree > self.degree {
            return (GF2NPolynomial::zero(), self);
        }

        let mut quotient = GF2NPolynomial::zero();
        let mut remainder = self;

        while !remainder.terms.is_empty() && remainder.degree >= rhs.degree {
            let degree_diff = (remainder.degree - rhs.degree) as u8;
            let term = GF2NPolynomial::new(vec![degree_diff]);

            // Add the new term to the quotient
            quotient = quotient + term.clone();

            // Calculate the remainder
            let product = rhs.clone() * term.clone();
            remainder = remainder - product;
        }

        // Return the (quotient, remainder)
        (quotient.fix_terms(), remainder.fix_terms())
    }
}
