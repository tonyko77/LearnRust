//=================================
//   Some basic Rust stuff :))
//=================================

// Since this is just a test/learning file, disable some "dead code" warnings
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// Packages, crates ... etc
// * Package:
//   - contains one or more Crates (at most one library crate, 0 or more binary crates)
//   - contains the Cargo.toml file
// * Crate:
//   - the smallest compilation unit
//   - starts from one source file (the crate root), then includes all its modules, submodules etc.
// * Module:
//   - contains one or more items (types, functions, etc)
//   - must be declared using `mod ...` in the crate root
//   - may contain other submodules -> sub-submodules -> ...
// => Package -> Crate(s) -> Module(s) -> Sub-module(s) ...


// Modules
mod common_rust;
mod ownership;
mod first_word;
mod collections;
mod error_handling;
mod playground;
mod lifetimes;

// Public modules
pub mod enums;
pub mod structs;


// Some constants
const _INT_CONSTANT: isize = 123_456_789;
const _STRING_CONSTANT: &str = "abc";


// MAIN entry point
fn main() {
    // common_rust::test_basic_rust();
    // ownership::test_ownership_and_borrowing();
    // first_word::test_first_word_corner_cases();
    // structs::test_structs();
    // enums::test_enums();
    // collections::test_common_collections();
    // error_handling::test_error_handling();
    playground::playground();
    lifetimes::lifetimes();
}
