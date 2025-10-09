/// Main function that demonstrates the add function
pub fn main() {
    // Display version information
    println!("rust_template v{}", rust_template::version());
    println!(
        "Built with Rust {} and Cargo {}",
        rust_template::rust_version(),
        rust_template::cargo_version()
    );
    println!();

    let a = 2;
    let b = 3;
    let result = rust_template::calculate_and_display(a, b);
    println!("{result}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_main_functionality() {
        // Test that main function logic works correctly
        let a = 2;
        let b = 3;
        let result = rust_template::calculate_and_display(a, b);
        assert_eq!(result, "2 + 3 = 5");
    }

    #[test]
    fn test_main_with_different_values() {
        let a = 10;
        let b = 20;
        let result = rust_template::calculate_and_display(a, b);
        assert_eq!(result, "10 + 20 = 30");
    }

    #[test]
    fn test_main_with_negative_values() {
        let a = -5;
        let b = 3;
        let result = rust_template::calculate_and_display(a, b);
        assert_eq!(result, "-5 + 3 = -2");
    }
}
