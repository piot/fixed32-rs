/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use std::f64::consts::TAU;

// Fixed-point scale factor
const SCALE: i32 = 1 << 16; // 2^16
const NUM_ENTRIES: usize = 256;

// Converts a floating-point value to a fixed-point representation.
fn to_fixed_point(value: f64) -> i32 {
    (value * SCALE as f64).round() as i32
}

// Function to convert fixed-point to floating-point (for verification)
fn from_fixed_point(x: i32) -> f64 {
    x as f64 / SCALE as f64
}

struct LookupTables {
    sin_table: Vec<i32>,
    cos_table: Vec<i32>,
    asin_table: Vec<i32>,
    acos_table: Vec<i32>,
    atan2_table: Vec<i32>,
}

fn compute_lookup_tables() -> LookupTables {
    let mut sin_table = Vec::with_capacity(NUM_ENTRIES);
    let mut cos_table = Vec::with_capacity(NUM_ENTRIES);
    let mut asin_table = Vec::with_capacity(NUM_ENTRIES);
    let mut acos_table = Vec::with_capacity(NUM_ENTRIES);

    let step = TAU / NUM_ENTRIES as f64;

    for i in 0..NUM_ENTRIES {
        let radians = i as f64 * step;
        let normalized_value = 2.0 * (i as f64 / (NUM_ENTRIES - 1) as f64) - 1.0;

        let sin_value = radians.sin();
        sin_table.push(to_fixed_point(sin_value));

        let cos_value = radians.cos();
        cos_table.push(to_fixed_point(cos_value));

        let asin_value = normalized_value.asin();
        asin_table.push(to_fixed_point(asin_value));

        let acos_value = normalized_value.acos();
        acos_table.push(to_fixed_point(acos_value));
    }

    let mut atan2_table = Vec::with_capacity(NUM_ENTRIES);

    const NUM_OCTANTS: usize = 8;
    const RATIOS_PER_OCTANT: usize = 32;
    const NUM_ENTRIES: usize = NUM_OCTANTS * RATIOS_PER_OCTANT;

    for octant in 0..NUM_OCTANTS {
        for ratio in 0..RATIOS_PER_OCTANT {
            // Calculate the ratio as fixed-point
            let ratio_fp = (ratio as i32 * SCALE) >> 5; // ratio / 32 * SCALE

            // Determine (x, y) based on the octant
            let (x, y) = match octant {
                0 => (SCALE, ratio_fp),
                1 => (ratio_fp, SCALE),
                2 => (-ratio_fp, SCALE),
                3 => (-SCALE, ratio_fp),
                4 => (-SCALE, -ratio_fp),
                5 => (-ratio_fp, -SCALE),
                6 => (ratio_fp, -SCALE),
                7 => (SCALE, -ratio_fp),
                _ => unreachable!(),
            };

            // Convert fixed-point to floating-point for atan2 calculation
            let x_f = from_fixed_point(x);
            let y_f = from_fixed_point(y);

            // Compute the angle in radians
            let angle = y_f.atan2(x_f); // Range: -PI to PI

            // Convert angle to fixed-point and store in the table
            atan2_table.push(to_fixed_point(angle));
        }
    }

    LookupTables {
        sin_table,
        cos_table,
        asin_table,
        acos_table,
        atan2_table,
    }
}

fn format_with_underscores(n: i32) -> String {
    let mut s = n.to_string();
    let is_negative = s.starts_with('-');
    if is_negative {
        s = s.trim_start_matches('-').to_string();
    }

    let mut chars: Vec<char> = s.chars().rev().collect();
    for i in (3..chars.len()).step_by(4) {
        chars.insert(i, '_');
    }

    let formatted: String = chars.iter().rev().collect();
    if is_negative {
        format!("-{}", formatted)
    } else {
        formatted
    }
}

// Print the lookup tables with 10 values per line
fn print_lookup_table(name: &str, table: &[i32]) {
    println!("pub const {}: [Fp; {}] = [", name, table.len());
    for (i, value) in table.iter().enumerate() {
        let last_in_line = i % 10 == 0;
        if last_in_line && i > 0 {
            println!();
        }
        print!(
            "{}Fp({}),",
            if last_in_line { "    " } else { " " },
            format_with_underscores(*value)
        );
    }
    println!("\n];");
}

fn main() {
    let tables = compute_lookup_tables();

    println!("use crate::Fp;");

    print_lookup_table("SIN_TABLE", &tables.sin_table);
    print_lookup_table("COS_TABLE", &tables.cos_table);
    print_lookup_table("ASIN_TABLE", &tables.asin_table);
    print_lookup_table("ACOS_TABLE", &tables.acos_table);
    print_lookup_table("ATAN2_TABLE", &tables.atan2_table);
}
