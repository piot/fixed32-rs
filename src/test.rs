/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-rs
 * Licensed under the MIT License. See LICENSE file for details.
 */
#[cfg(test)]
use crate::Fp;
#[test]
fn add() {
    let result = Fp::from(2) + Fp::from(2);
    assert_eq!(result.0, 4 * Fp::SCALE);
}

#[test]
fn mul() {
    let result = Fp::from(3) * Fp::from(2);
    assert_eq!(result.0, 6 * Fp::SCALE);
}

#[test]
fn div() {
    let result = Fp::from(99) / Fp::from(3);
    assert_eq!(result.0, 33 * Fp::SCALE);
}

#[test]
fn div_bigger_number() {
    let result = Fp::from(30000) / Fp::from(12);
    assert_eq!(result.0, 2500 * Fp::SCALE);
}

#[test]
fn sub() {
    let result = Fp::from(-42) + Fp::from(43);
    assert_eq!(result.0, 1 * Fp::SCALE);
}

#[test]
fn div_int() {
    let result = 400 / Fp::from(10);
    let i: i32 = result.into();
    assert_eq!(i, 40);
}

#[test]
fn mul_int() {
    let result = 99 * Fp::from(10);
    let i: i16 = result.into();
    assert_eq!(i, 990);
}

#[test]
fn from_float() {
    let result = 99 * Fp::from(10.0);
    let i: i16 = result.into();
    assert_eq!(i, 990);
}

#[test]
fn acos_test() {
    // Expected floating-point outputs for acos
    let test_values = [
        (core::f32::consts::PI, -1.0),       // π
        (core::f32::consts::FRAC_PI_2, 0.0), // π/2
        (0.0, 1.0),                          // acos(1.0)
        (core::f32::consts::FRAC_PI_3, 0.5), // π/3
        (2.0944, -0.5),                      // 2π/3
        (
            core::f32::consts::FRAC_PI_4,
            core::f32::consts::FRAC_1_SQRT_2,
        ), // π/4
        (2.3562, -core::f32::consts::FRAC_1_SQRT_2), // 3π/4
        (core::f32::consts::FRAC_PI_6, 0.866), // π/6
        (2.61799, -0.866),                   // 5π/6
    ];

    println!("Expected fixed-point values (16.16 format):");
    for &(expected_radian, unit_interval) in test_values.iter() {
        let fp_unit_interval = Fp::from(unit_interval);
        let result_radian = fp_unit_interval.acos();
        let fp_raw_radian = result_radian.0;
        let expected_raw_radian = Fp::from(expected_radian).0;
        let diff = (fp_raw_radian - expected_raw_radian).abs();

        println!(
            "Input: {:.4}, Calculated: {:.4} ({:.4}), Expected: {} ({:.4}) diff: {diff}",
            unit_interval,
            fp_raw_radian,
            fp_raw_radian as f64 / Fp::SCALE as f64,
            expected_raw_radian,
            expected_raw_radian as f64 / Fp::SCALE as f64,
        );

        if diff > 500 {
            panic!("too far off. got {result_radian}, but expected {expected_raw_radian}");
        }
    }
}

#[test]
fn test_custom_sin() {
    let test_values = [
        (0.0, 0.0),
        (core::f64::consts::PI / 2.0, 1.0),
        (5.0 * core::f64::consts::PI / 2.0, 1.0),
        (core::f64::consts::PI, 0.0),
        (3.0 * core::f64::consts::PI / 2.0, -1.0),
        (2.0 * core::f64::consts::PI, 0.0),
        (core::f64::consts::PI / 6.0, 0.5),
        (1.01598529, 0.85),
    ];

    for (input, expected) in test_values {
        let fp = Fp::from(input as f32);
        let fp_result = fp.sin();
        let result: f32 = fp_result.into();
        println!("result: {result} expected: {expected}");
        assert!(
            (result - expected).abs() < 0.03,
            "Expected sin({}) ≈ {}, got {}",
            input,
            expected,
            result
        );
    }
}

#[test]
fn test_sqrt_zero() {
    let value = Fp::from(0);
    let result = value.sqrt();
    assert_eq!(result, Fp::zero()); // sqrt(0) is 0
}

#[test]
fn test_sqrt_one() {
    let value = Fp::one();
    let result = value.sqrt();
    assert_eq!(result, Fp::one()); // sqrt(1) is 1
}

#[test]
fn test_sqrt_four() {
    let value = Fp::from(4);
    let result = value.sqrt();
    assert_eq!(result, Fp::from(2)); // sqrt(4) is 2
}

#[test]
fn test_sqrt_fraction() {
    let value = Fp::from(4225);
    let result = value.sqrt();
    assert_eq!(result, Fp::from(65)); // sqrt(4225) is 65
}

#[test]
#[should_panic]
fn test_sqrt_negative() {
    let value = Fp::from(-3923);
    value.sqrt();
}

#[test]
fn test_ceil_positive() {
    let value = Fp::from(5);
    let result = value.ceil();
    assert_eq!(result, value);
}

#[test]
fn test_ceil_negative() {
    let value = Fp::from(-5) - Fp(1);
    let result = value.ceil();
    assert_eq!(result, Fp::from(-5)); // Ceil of -5.000015 should be -5
}

#[test]
fn test_round_positive() {
    let value = Fp::from(5) + Fp(Fp::SCALE / 2); // 5.5
    let result = value.round();
    assert_eq!(result, Fp::from(6)); // Round(5.5) should be 6
}

#[test]
fn test_round_negative() {
    let value = Fp::from(-5) - Fp(Fp::SCALE / 2); // -5.5
    let result = value.round();
    assert_eq!(result, Fp::from(-5)); // Round(-5.5) should be -5
}

#[test]
fn test_clamp_within_range() {
    let value = Fp::from(5);
    let min = Fp::from(3);
    let max = Fp::from(7);
    let result = value.clamp(min, max);
    assert_eq!(result, value); // 5 is within the range [3, 7]
}

#[test]
fn test_clamp_below_range() {
    let value = Fp::from(-4);
    let min = Fp::from(-3);
    let max = Fp::from(7);
    let result = value.clamp(min, max);
    assert_eq!(result, min);
}

#[test]
fn test_clamp_above_range() {
    let value = Fp::from(8);
    let min = Fp::from(-3);
    let max = Fp::from(7);
    let result = value.clamp(min, max);
    assert_eq!(result, max); // 8 is above the range [-3, 7], so it should be clamped to 7
}

#[test]
fn test_abs_positive() {
    let value = Fp::from(5);
    let result = value.abs();
    assert_eq!(result, value); // Absolute value of 5 should be 5
}

#[test]
fn test_abs_negative() {
    let value = Fp::from(-5);
    let result = value.abs();
    assert_eq!(result, Fp::from(5)); // Absolute value of -5 should be 5
}

#[test]
fn test_int16_less_than() {
    let value = Fp::from(-5);
    let result = value < -4;
    assert!(result);
}

#[test]
fn test_int16_greater_than() {
    let value = Fp::from(-5);
    let result = value > -6;
    assert!(result);
}

#[test]
fn test_int16_equal() {
    let value = Fp::from(-42);
    let result = value == -42;
    assert!(result);
}

#[test]
fn test_int16_not_equal() {
    let value = Fp::from(-42);
    let result = value != 90;
    assert!(result);
}
