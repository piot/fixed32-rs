# Fixed32

Fixed32 is a Rust crate that provides a 32-bit fixed-point number type, designed for applications that require deterministic behavior and work with low precision and small ranges, such as games or embedded systems. This type is particularly useful in scenarios where floating-point arithmetic may introduce unwanted variability or overhead.

## Overview

- **Type:** `Fp`
- **Precision:** 16.16 fixed-point format
- **Integer Range:** -32768 to +32767
- **Decimal Precision:** The 16.16 fixed-point format provides approximately 4 decimal places of precision. If one unit represents a meter, this precision corresponds to about 0.1 millimeters.

## Features

- **Deterministic Arithmetic:** Ensures consistent results across different platforms and runs.
- **Low Overhead:** Efficient fixed-point arithmetic suitable for performance-critical applications.
- **Range Handling:** Suitable for scenarios with small numerical ranges.

## Installation

To use `fixed32` in your Rust project, add it to your `Cargo.toml`:

```toml
fixed32 = "0.0.14"
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
