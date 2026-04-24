//! Integration tests for the arithmetic surface of `rust_template`.
//!
//! Each file under `tests/` is compiled as its own crate and may only call
//! the public API, so these tests exercise `rust_template` exactly the way a
//! downstream consumer would. Per-function correctness tests live as unit
//! tests next to the code in `src/lib.rs`; this file focuses on
//! cross-function composition and invariants.

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
