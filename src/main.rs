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
