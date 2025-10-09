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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adds_two_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn adds_positive_and_negative_numbers() {
        assert_eq!(add(5, -3), 2);
        assert_eq!(add(-5, 3), -2);
    }

    #[test]
    fn adds_with_zero() {
        assert_eq!(add(0, 5), 5);
        assert_eq!(add(5, 0), 5);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn adds_large_numbers() {
        assert_eq!(add(1000000, 2000000), 3000000);
    }

    #[test]
    fn multiplies_two_numbers() {
        assert_eq!(multiply(3, 4), 12);
    }

    #[test]
    fn multiplies_with_zero() {
        assert_eq!(multiply(5, 0), 0);
        assert_eq!(multiply(0, 5), 0);
    }

    #[test]
    fn multiplies_negative_numbers() {
        assert_eq!(multiply(-3, 4), -12);
        assert_eq!(multiply(3, -4), -12);
        assert_eq!(multiply(-3, -4), 12);
    }

    #[test]
    fn subtracts_two_numbers() {
        assert_eq!(subtract(5, 3), 2);
    }

    #[test]
    fn subtracts_with_zero() {
        assert_eq!(subtract(5, 0), 5);
        assert_eq!(subtract(0, 5), -5);
    }

    #[test]
    fn subtracts_negative_numbers() {
        assert_eq!(subtract(-5, -3), -2);
        assert_eq!(subtract(5, -3), 8);
    }
}
