// write simple unit test in Rust

use std::process;

fn main() {
    println!("{:?}", get_process_id());
}

fn get_process_id() -> u32 {
	process::id()
}

// add unit test codes
// note the assert macro is used to check whether the condition evaluates to true
// there are 2 other assert macros e.g. assert_eq! and assert_ne!
#[test]
fn test_if_process_id_is_returned() {
	// assert!(get_process_id() > 0);
	// another way to use assert macro
	// note assert_ne requires 2 arguments
	// this time with custom error message
	assert_ne!(get_process_id(), 0, "There is error in code");
}