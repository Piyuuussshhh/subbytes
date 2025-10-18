use super::object::GF2NPolynomial;

pub fn get_irreducible_polynomial(n: u32) -> GF2NPolynomial {
    match n {
        1 => {
            return GF2NPolynomial {
                degree: 1,
                // p(x) = x
                terms: vec![1],
            };
        }
        2 => {
            return GF2NPolynomial {
                degree: 2,
                // p(x) = x^2 + x + 1
                terms: vec![2, 1, 0],
            };
        }
        3 => {
            return GF2NPolynomial {
                degree: 3,
                // p(x) = x^3 + x + 1
                terms: vec![3, 1, 0],
            };
        }
        4 => {
            return GF2NPolynomial {
                degree: 4,
                // p(x) = x^4 + x + 1
                terms: vec![4, 1, 0],
            };
        }
        5 => {
            return GF2NPolynomial {
                degree: 5,
                // p(x) = x^5 + x^2 + 1
                terms: vec![5, 2, 0],
            };
        }
        6 => {
            return GF2NPolynomial {
                degree: 6,
                // p(x) = x^6 + x + 1
                terms: vec![6, 1, 0],
            };
        }
        7 => {
            return GF2NPolynomial {
                degree: 7,
                // p(x) = x^7 + x + 1
                terms: vec![7, 1, 0],
            };
        }
        8 => {
            return GF2NPolynomial {
                degree: 8,
                // p(x) = x^8 + x^4 + x^3 + x + 1
                terms: vec![8, 4, 3, 1, 0],
            };
        }
        9 => {
            return GF2NPolynomial {
                degree: 9,
                // p(x) = x^9 + x + 1
                terms: vec![9, 1, 0],
            };
        }
        10 => {
            return GF2NPolynomial {
                degree: 10,
                // p(x) = x^10 + x^3 + 1
                terms: vec![10, 3, 0],
            };
        }
        16 => {
            return GF2NPolynomial {
                degree: 16,
                // p(x) = x^16 + x^5 + x^3 + x^2 + 1
                terms: vec![16, 5, 3, 2, 0],
            };
        }
        32 => {
            return GF2NPolynomial {
                degree: 32,
                // p(x) = x^32 + x^22 + x^2 + x + 1
                terms: vec![32, 22, 2, 1, 0],
            };
        }
        64 => {
            return GF2NPolynomial {
                degree: 64,
                // p(x) = x^64 + x^4 + x^3 + x + 1
                terms: vec![64, 4, 3, 1, 0],
            };
        }
        _ => return GF2NPolynomial::zero(),
    }
}
