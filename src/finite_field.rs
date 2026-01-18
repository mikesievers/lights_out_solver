use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq)]
pub struct GFElement {
    // Define a Galois (Finite) Field element
    value: i32,
    modulus: i32,
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

impl Add for GFElement {
    type Output = GFElement;

    fn add(self, other: GFElement) -> Self {
        assert!(self.modulus == other.modulus);
        GFElement::new((self.value + other.value) % self.modulus, self.modulus)
    }
}

mod tests {
    use super::GFElement;
    use rstest::rstest;

    #[rstest]
    #[case::two_plus_one(2, 1, 0)]
    #[case::one_plus_one(1, 1, 2)]
    #[case::zero_plus_zero(0, 0, 0)]
    #[case::modulus_works(3, 4, 1)]
    fn test_add(#[case] a_val: i32, #[case] b_val: i32, #[case] expected: i32) {
        let a = GFElement::new(a_val, 3);
        let b = GFElement::new(b_val, 3);
        let c = GFElement::new(expected, 3);
        assert_eq!(a + b, c);
    }
}
