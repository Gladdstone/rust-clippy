#![warn(clippy::doc_link_reference)]

/// Example Rust file to test the malformed reference lint
///
/// [useful text][reference]
///
/// [reference](https://example.com)
fn main() {
    println!("hello, world");
}

