#[test]
fn add_works() {
    assert_eq!(rust_template::add(10, 5), 15);
}

#[test]
fn multiply_works() {
    assert_eq!(rust_template::multiply(4, 5), 20);
}

#[test]
fn subtract_works() {
    assert_eq!(rust_template::subtract(10, 3), 7);
}

#[test]
fn calculate_and_display_works() {
    let result = rust_template::calculate_and_display(7, 8);
    assert_eq!(result, "7 + 8 = 15");
}

#[test]
fn integration_test_complex_calculation() {
    // Test a more complex scenario using multiple functions
    let a = 10;
    let b = 5;
    let sum = rust_template::add(a, b);
    let product = rust_template::multiply(a, b);
    let difference = rust_template::subtract(a, b);

    assert_eq!(sum, 15);
    assert_eq!(product, 50);
    assert_eq!(difference, 5);

    // Test the display function
    let display = rust_template::calculate_and_display(a, b);
    assert_eq!(display, "10 + 5 = 15");
}

#[test]
fn edge_cases_test() {
    // Test edge cases
    assert_eq!(rust_template::add(0, 0), 0);
    assert_eq!(rust_template::multiply(0, 100), 0);
    assert_eq!(rust_template::subtract(0, 0), 0);

    // Test negative numbers
    assert_eq!(rust_template::add(-5, -3), -8);
    assert_eq!(rust_template::multiply(-2, 3), -6);
    assert_eq!(rust_template::subtract(-5, -3), -2);
}
