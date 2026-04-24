/// Returns the build version information including git metadata
pub fn version() -> &'static str {
    env!("BUILD_VERSION")
}

/// Returns the Rust version used to build this binary
pub fn rust_version() -> &'static str {
    env!("BUILD_RUST_VERSION")
}

/// Returns the Cargo version used to build this binary
pub fn cargo_version() -> &'static str {
    env!("BUILD_CARGO_VERSION")
}

/// Returns the sum of two integers.
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

/// Returns the product of two integers.
pub fn multiply(left: i32, right: i32) -> i32 {
    left * right
}

/// Returns the difference between two integers.
pub fn subtract(left: i32, right: i32) -> i32 {
    left - right
}

/// Calculates and returns the sum of two numbers with formatted output
pub fn calculate_and_display(a: i32, b: i32) -> String {
    let sum = add(a, b);
    format!("{a} + {b} = {sum}")
}

// Unit tests live alongside the code they verify, grouped into one nested
// module per function. See the Rust Book, ch. 11.3 — "Test Organization":
// https://doc.rust-lang.org/book/ch11-03-test-organization.html
#[cfg(test)]
mod tests {
    mod version_info {
        use super::super::*;

        #[test]
        fn version_is_populated() {
            assert!(!version().is_empty());
        }

        #[test]
        fn rust_version_is_populated() {
            assert!(!rust_version().is_empty());
        }

        #[test]
        fn cargo_version_is_populated() {
            assert!(!cargo_version().is_empty());
        }
    }

    mod add {
        use super::super::*;

        #[test]
        fn two_positive_numbers() {
            assert_eq!(add(2, 3), 5);
        }

        #[test]
        fn two_negative_numbers() {
            assert_eq!(add(-2, -3), -5);
        }

        #[test]
        fn positive_and_negative() {
            assert_eq!(add(5, -3), 2);
            assert_eq!(add(-5, 3), -2);
        }

        #[test]
        fn with_zero() {
            assert_eq!(add(0, 5), 5);
            assert_eq!(add(5, 0), 5);
            assert_eq!(add(0, 0), 0);
        }

        #[test]
        fn large_numbers() {
            assert_eq!(add(1_000_000, 2_000_000), 3_000_000);
        }
    }

    mod multiply {
        use super::super::*;

        #[test]
        fn two_numbers() {
            assert_eq!(multiply(3, 4), 12);
        }

        #[test]
        fn with_zero() {
            assert_eq!(multiply(5, 0), 0);
            assert_eq!(multiply(0, 5), 0);
        }

        #[test]
        fn negative_numbers() {
            assert_eq!(multiply(-3, 4), -12);
            assert_eq!(multiply(3, -4), -12);
            assert_eq!(multiply(-3, -4), 12);
        }
    }

    mod subtract {
        use super::super::*;

        #[test]
        fn two_numbers() {
            assert_eq!(subtract(5, 3), 2);
        }

        #[test]
        fn with_zero() {
            assert_eq!(subtract(5, 0), 5);
            assert_eq!(subtract(0, 5), -5);
        }

        #[test]
        fn negative_numbers() {
            assert_eq!(subtract(-5, -3), -2);
            assert_eq!(subtract(5, -3), 8);
        }
    }

    mod calculate_and_display {
        use super::super::*;

        #[test]
        fn positive_numbers() {
            assert_eq!(calculate_and_display(2, 3), "2 + 3 = 5");
        }

        #[test]
        fn larger_numbers() {
            assert_eq!(calculate_and_display(10, 20), "10 + 20 = 30");
        }

        #[test]
        fn negative_numbers() {
            assert_eq!(calculate_and_display(-5, 3), "-5 + 3 = -2");
        }

        #[test]
        fn with_zero() {
            assert_eq!(calculate_and_display(0, 0), "0 + 0 = 0");
        }
    }
}
