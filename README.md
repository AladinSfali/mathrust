# MathRust

MathRust is a collection of command-line calculators built in Rust. Each calculator is a separate binary that can be run directly with Cargo.

## Calculators

Here is a list of the available calculators:

| Calculator                  | Description                                                                                             |
| --------------------------- | ------------------------------------------------------------------------------------------------------- |
| `bodmas_calculator`         | Evaluates mathematical expressions following the BODMAS/PEMDAS order of operations.                     |
| `decimal_places`            | Rounds a number to a specified number of decimal places.                                                |
| `estimation`                | Estimates the result of an expression by rounding each number to one significant figure.                |
| `estimation_square_root`    | Estimates the square root of a number.                                                                  |
| `hcf`                       | Calculates the Highest Common Factor (HCF) of two numbers.                                              |
| `lcm`                       | Calculates the Lowest Common Multiple (LCM) of two numbers.                                             |
| `multiples`                 | Lists the multiples of a number up to a certain limit.                                                  |
| `prime_numbers`             | Lists all prime numbers up to a given number.                                                           |
| `prod_prime_factor`         | Calculates the product of the prime factors of a number.                                                |
| `rounding`                  | Rounds a number to the nearest 10, 100, 1000, etc.                                                      |
| `significant_figures`       | Rounds a number to a specified number of significant figures.                                           |
| `upperlower_bounds`         | Calculates the upper and lower bounds of a number that has been rounded to a certain degree of accuracy. |

## GUI Application

In addition to the command-line tools, MathRust also provides a graphical user interface (GUI) that gives you access to all the calculators in a single window.

### Platform-specific Dependencies

To compile and run the GUI application, you may need to install some system dependencies.

#### Linux

On Linux, you need to have development libraries for either X11 or Wayland.

For **Debian-based distributions (like Ubuntu)**, you can install the required packages with:

```bash
sudo apt install libx11-xcb-dev xinput libxcursor-dev libxkbcommon-x11-dev libx11-dev
```

For **Fedora/RHEL-based distributions**, you can use `dnf` to install similar packages (e.g., `libX11-devel`, `libxkbcommon-x11-devel`, etc.).

#### Windows

No additional system dependencies are required on Windows. You will only need the standard Rust toolchain for MSVC.

#### macOS

No additional system dependencies are required on macOS. Make sure you have the Xcode Command Line Tools installed.

## How to Run

### Command-line Calculators

To run any of the command-line calculators, use the `cargo run --bin` command, followed by the name of the calculator. For example, to run the BODMAS calculator, you would use the following command:

```bash
cargo run --bin bodmas_calculator
```

You will then be prompted to enter your input.

### GUI Application

To run the GUI application, use the following command:

```bash
cargo run --bin gui
```
