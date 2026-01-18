use std::fmt;

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GFElement {
    // Define a Galois (Finite) Field element
    pub value: i32,
    pub modulus: i32,
}

impl GFElement {
    pub fn new(v: i32, m: i32) -> Self {
        // Create a new GFElement
        // v: value
        // m: modulus >= 0
        assert!(m >= 0);
        GFElement {
            value: v.rem_euclid(m),
            modulus: m,
        }
    }
}

impl fmt::Display for GFElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for GFElement {
    type Output = GFElement;

    fn add(self, other: GFElement) -> Self {
        assert!(self.modulus == other.modulus);
        GFElement::new(
            (self.value + other.value).rem_euclid(self.modulus),
            self.modulus,
        )
    }
}

impl Sub for GFElement {
    type Output = GFElement;

    fn sub(self, other: GFElement) -> Self {
        assert!(self.modulus == other.modulus);
        GFElement::new(
            (self.value - other.value).rem_euclid(self.modulus),
            self.modulus,
        )
    }
}

impl Mul for GFElement {
    type Output = GFElement;

    fn mul(self, other: GFElement) -> Self {
        assert_eq!(self.modulus, other.modulus);
        GFElement::new(
            (self.value * other.value).rem_euclid(self.modulus),
            self.modulus,
        )
    }
}

impl Div for GFElement {
    type Output = GFElement;

    fn div(self, other: GFElement) -> Self {
        assert_eq!(self.modulus, other.modulus);
        if other.value == 0 {
            panic!("Division by 0 attempted");
        }

        GFElement {
            value: (self.value * other.mult_inverse().value).rem_euclid(self.modulus),
            modulus: self.modulus,
        }
    }
}

// Find the multiplicative inverse, needed for Division
impl GFElement {
    fn mult_inverse(&self) -> GFElement {
        for i in 0..self.modulus {
            if (self.value * i).rem_euclid(self.modulus) == 1 {
                return GFElement::new(i, self.modulus);
            }
        }
        let msg = format!(
            "Multiplicative inverse for value {} not found for base {}",
            self.value, self.modulus
        );
        panic!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::GFElement;
    use rstest::rstest;

    #[rstest]
    #[case::zero_plus_zero(0, 0, 0)]
    #[case::two_plus_one(2, 1, 0)]
    #[case::one_plus_one(1, 1, 2)]
    #[case::zero_plus_zero(0, 0, 0)]
    #[case::modulus_works(3, 4, 1)]
    #[case::negative_cycles(3, -5, 1)]
    fn test_add(#[case] a_val: i32, #[case] b_val: i32, #[case] expected: i32) {
        let a = GFElement::new(a_val, 3);
        let b = GFElement::new(b_val, 3);
        let c = GFElement::new(expected, 3);
        assert_eq!(a + b, c);
    }

    #[rstest]
    #[case::three_minus_one(3, 1, 2)]
    #[case::negative_cycles(1, 2, 2)]
    fn test_sub(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        let a = GFElement::new(a, 3);
        let b = GFElement::new(b, 3);
        let c = GFElement::new(expected, 3);
        assert_eq!(a - b, c);
    }

    #[rstest]
    #[case(1, 1, 1)]
    #[case(2, 0, 0)]
    #[case(2, 2, 1)]
    #[case(2, 3, 0)]
    fn test_mul(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        let a = GFElement::new(a, 3);
        let b = GFElement::new(b, 3);
        let expected = GFElement::new(expected, 3);

        assert_eq!(a * b, expected);
    }

    #[rstest]
    #[case(1, 1, 3, 1)]
    #[case(2, 2, 3, 1)]
    #[case(1, 2, 3, 2)]
    #[case(2, 3, 7, 3)]
    #[case(5, 9, 13, 2)]
    fn test_div(
        #[case] dividend: i32,
        #[case] divisor: i32,
        #[case] modulus: i32,
        #[case] expected: i32,
    ) {
        let dividend = GFElement::new(dividend, modulus);
        let divisor = GFElement::new(divisor, modulus);
        let expected = GFElement::new(expected, modulus);

        assert_eq!(dividend / divisor, expected);
    }

    #[test]
    fn test_display() {
        let a = GFElement::new(2, 3);
        let expected = "2";
        assert_eq!(format!("{a}"), expected);
    }
}
