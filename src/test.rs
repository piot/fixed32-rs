/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-rs
 * Licensed under the MIT License. See LICENSE file for details.
 */
#[cfg(test)]
use crate::{Fp, SCALE};
#[test]
fn add() {
    let result = Fp::from(2) + Fp::from(2);
    assert_eq!(result.0, 4 * SCALE);
}

#[test]
fn mul() {
    let result = Fp::from(3) * Fp::from(2);
    assert_eq!(result.0, 6 * SCALE);
}

#[test]
fn div() {
    let result = Fp::from(99) / Fp::from(3);
    assert_eq!(result.0, 33 * SCALE);
}

#[test]
fn div_bigger_number() {
    let result = Fp::from(30000) / Fp::from(12);
    assert_eq!(result.0, 2500 * SCALE);
}

#[test]
fn sub() {
    let result = Fp::from(-42) + Fp::from(43);
    assert_eq!(result.0, 1 * SCALE);
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
            fp_raw_radian as f64 / SCALE as f64,
            expected_raw_radian,
            expected_raw_radian as f64 / SCALE as f64,
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
