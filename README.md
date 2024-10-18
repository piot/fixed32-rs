# Fixed32

Fixed32 is a Rust crate providing a 32-bit fixed-point number type. It is designed for applications
requiring deterministic behavior with low precision and small numerical ranges, such as games
or embedded systems. This type is particularly useful in situations where floating-point
arithmetic might introduce variability or unnecessary overhead.

## Overview

- **Type:** `Fp`
- **Precision:** 16.16 fixed-point format (16 bits for integer, 16 bits for fractional)
- **Integer Range:** -32768 to +32767
- **Decimal Precision:** The 16.16 fixed-point format provides approximately 4 decimal places
  of precision (e.g., if 1 unit equals 1 meter, the precision is about 0.1 millimeters)

## Features

- **Deterministic Arithmetic:** Ensures consistent results across different platforms and runs.
- **Low Overhead:** Efficient fixed-point arithmetic suitable for performance-critical applications.
- **Range Handling:** Suitable for scenarios with small numerical ranges.

## Installation

To use `fixed32` in your Rust project, add it to your `Cargo.toml`:

```toml
fixed32 = "0.0.16"
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
