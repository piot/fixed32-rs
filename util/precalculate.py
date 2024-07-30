import math

# Fixed-point scale factor
SCALE = 1 << 16  # 2^16

def to_fixed_point(value):
    return int(round(value * SCALE))

def compute_lookup_tables(num_entries=256):
    angles = range(num_entries)

    step = 360.0 / num_entries

    sin_values = [math.sin(math.radians(angle * step)) for angle in angles]
    sin_table = [to_fixed_point(value) for value in sin_values]

    cos_values = [math.cos(math.radians(angle * step)) for angle in angles]
    cos_table = [to_fixed_point(value) for value in cos_values]

    normalize_values = [2 * (i / (num_entries - 1)) - 1 for i in range(num_entries)]

    asin_values = [math.asin(normalize_value) for normalize_value in normalize_values]
    asin_table = [to_fixed_point(value) for value in asin_values]

    acos_values = [math.acos(normalize_value) for normalize_value in normalize_values]
    acos_table = [to_fixed_point(value) for value in acos_values]

    return {
        "sin_table": sin_table,
        "cos_table": cos_table,
        "asin_table": asin_table,
        "acos_table": acos_table
    }

def print_lookup_table(name, table):
    print(f"pub const {name}: [Fp; 256] = [")
    for value in table:
        print(f"    Fp({value}),")
    print("];")

tables = compute_lookup_tables()

print("use crate::Fp;")

print_lookup_table("SIN_TABLE", tables["sin_table"])
print_lookup_table("COS_TABLE", tables["cos_table"])
print_lookup_table("ASIN_TABLE", tables["asin_table"])
print_lookup_table("ACOS_TABLE", tables["acos_table"])
