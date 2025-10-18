use super::object::GF2NPolynomial;

pub fn validate(
    input_poly: &GF2NPolynomial,
    inverse_poly: &GF2NPolynomial,
    irr_poly: &GF2NPolynomial,
    n: u32,
) {
    println!(
        "\nValidating whether {} is the inverse of {} in GF(2^{}) with irreducible polynomial {}.",
        inverse_poly.algebraic_string(),
        input_poly.algebraic_string(),
        n,
        irr_poly.algebraic_string()
    );
    println!(
        "To check:\t({}) * ({}) ≡ 1 mod ({})",
        inverse_poly.algebraic_string(),
        input_poly.algebraic_string(),
        irr_poly.algebraic_string()
    );
    println!("Calculating LHS...");
    let product = inverse_poly.clone() * input_poly.clone();
    println!(
        "({}) * ({}) = {}",
        inverse_poly.algebraic_string(),
        input_poly.algebraic_string(),
        product.algebraic_string()
    );
    let (quotient, remainder) = product.clone() / irr_poly.clone();
    println!(
        "({}) mod({}): \n\tquotient = {}\n\tremainder = {}",
        product.algebraic_string(),
        irr_poly.algebraic_string(),
        quotient.algebraic_string(),
        remainder.algebraic_string()
    );

    assert!(
        remainder == GF2NPolynomial::one(),
        "Remainder is 1, therefore {} is the inverse of {}",
        inverse_poly.algebraic_string(),
        input_poly.algebraic_string()
    );
}
