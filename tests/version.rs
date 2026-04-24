//! Integration tests for build-time version metadata.
//!
//! Complements the unit tests in `src/lib.rs` by verifying, from an external
//! crate's perspective, that the values wired up by `build.rs` are reachable
//! through the public API and non-empty at runtime.

#[test]
fn version_info_is_populated() {
    assert!(!rust_template::version().is_empty());
    assert!(!rust_template::rust_version().is_empty());
    assert!(!rust_template::cargo_version().is_empty());
}
