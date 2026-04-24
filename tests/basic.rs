//! Integration tests for rust_template.
//!
//! These tests exercise the crate through its public API, from a consumer's
//! perspective. Unit tests of individual functions live alongside the source
//! in src/lib.rs.

#[test]
fn version_info_is_populated() {
    // Build metadata injected by build.rs must be non-empty at runtime.
    assert!(!rust_template::version().is_empty());
    assert!(!rust_template::rust_version().is_empty());
    assert!(!rust_template::cargo_version().is_empty());
}

#[test]
fn arithmetic_functions_compose() {
    let a = 10;
    let b = 5;

    let sum = rust_template::add(a, b);
    let product = rust_template::multiply(a, b);
    let difference = rust_template::subtract(a, b);

    assert_eq!(sum, 15);
    assert_eq!(product, 50);
    assert_eq!(difference, 5);

    // calculate_and_display should agree with add() over the same inputs.
    assert_eq!(
        rust_template::calculate_and_display(a, b),
        format!("{a} + {b} = {sum}")
    );
}

#[test]
fn display_stays_consistent_with_add() {
    // Cross-function invariant: display output must always match add().
    for a in -3..=3 {
        for b in -3..=3 {
            let sum = rust_template::add(a, b);
            assert_eq!(
                rust_template::calculate_and_display(a, b),
                format!("{a} + {b} = {sum}")
            );
        }
    }
}
