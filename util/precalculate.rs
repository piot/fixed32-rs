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

struct LookupTables {
    sin_table: Vec<i32>,
    cos_table: Vec<i32>,
    asin_table: Vec<i32>,
    acos_table: Vec<i32>,
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

    LookupTables {
        sin_table,
        cos_table,
        asin_table,
        acos_table,
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
        print!("{}Fp({}),", if last_in_line { "    " } else { " " }, value);
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
}
