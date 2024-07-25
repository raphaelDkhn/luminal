use cairo_vm::Felt252;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use std::cmp::{max, min};

// Constants for Fixed Point 32x32
const ONE: i128 = 4294967296; // 2^32
const _HALF: i128 = 2147483648; // 2^31
const MAX: i128 = 9223372036854775807; // 2^62
const MIN: i128 = -9223372036854775808; // -2^63

pub trait CairoFloat: std::fmt::Debug + Copy + Default + 'static {
    fn to_felt252(self) -> Felt252;
    fn from_felt252(a: Felt252) -> Self;
    fn type_name() -> &'static str;
}

impl CairoFloat for f32 {
    fn to_felt252(self) -> Felt252 {
        match self {
            x if x.is_infinite() || x.is_nan() => Felt252::ZERO,
            _ => {
                let scaled_value = (self as f64 * ONE as f64) as i128;
                Felt252::from(scaled_value.clamp(MIN, MAX))
            }
        }
    }

    fn from_felt252(a: Felt252) -> Self {
        let big_int = a.to_bigint();
        let prime = BigInt::from(Felt252::prime());
        let half_prime = &prime / 2u32;

        if big_int == BigInt::from(MIN) {
            return f32::MIN;
        }

        let int_value = if big_int >= half_prime {
            max(MIN + 1, -(&prime - &big_int).to_i128().unwrap_or(MAX))
        } else {
            min(MAX, big_int.to_i128().unwrap_or(MAX))
        };

        if int_value == MAX {
            f32::MAX
        } else {
            (int_value as f64 / ONE as f64) as f32
        }
    }

    fn type_name() -> &'static str {
        "f32"
    }
}

#[cfg(test)]
mod cairo_float_tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn assert_f32_approx_eq(a: f32, b: f32) {
        assert!(
            (a as f64 - b as f64).abs() < EPSILON,
            "Expected {} to be approximately equal to {}",
            a,
            b
        );
    }

    fn assert_felt_approx_eq(a: Felt252, b: Felt252) {
        let diff = if a >= b { a - b } else { b - a };
        assert!(
            diff < Felt252::from((EPSILON * ONE as f64) as u128),
            "Expected {} to be approximately equal to {}",
            a,
            b
        );
    }

    #[test]
    fn test_f32_conversions() {
        let test_cases = [
            (0.0, Felt252::ZERO),
            (1.0, Felt252::from(ONE)),
            (-1.0, Felt252::from(-ONE)),
            (0.5, Felt252::from(_HALF)),
            (1000000.0, Felt252::from(4294967296000000_i128)),
        ];

        for (f, expected_felt) in test_cases.iter() {
            let result_felt = f32::to_felt252(*f);
            assert_felt_approx_eq(result_felt, *expected_felt);

            let result_f32 = f32::from_felt252(*expected_felt);
            assert_f32_approx_eq(result_f32, *f);
        }
    }

    #[test]
    fn test_f32_roundtrips() {
        let test_cases = [0.123456, 123456.789, -9876.54321, -0.123456, -123456.789];

        for &original in test_cases.iter() {
            let roundtrip = f32::from_felt252(f32::to_felt252(original));
            assert_f32_approx_eq(original, roundtrip);
        }
    }

    #[test]
    fn test_f32_special_cases() {
        assert_eq!(f32::to_felt252(f32::INFINITY), Felt252::ZERO);
        assert_eq!(f32::to_felt252(f32::NEG_INFINITY), Felt252::ZERO);
        assert_eq!(f32::to_felt252(f32::NAN), Felt252::ZERO);
    }

    #[test]
    fn test_f32_type_name() {
        assert_eq!(f32::type_name(), "f32");
    }
}
