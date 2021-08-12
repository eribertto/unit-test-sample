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
//#[test]
//fn test_if_process_id_is_returned() {
//	// assert!(get_process_id() > 0);
//	// another way to use assert macro
//	// note assert_ne requires 2 arguments
//	// this time with custom error message
//	assert_ne!(get_process_id(), 0, "There is error in code");
//}

// version 2 of test function, this time with provision for scale, modularity
// this is Rust idiomatic way to group test functions in a test module
// test function is moved inside the tests module
// cfg attribute is added which tells the compiler to compile test code only
// if we are trying to run tests e.g. only for cargo test, not for cargo build command
// note the use keyword inside the module, this is to bring the function get_process_id into the scope
// of the tests module. also note that tests is an inner module so we use super:: prefix to bring
// the function that is being tested into the scope fo the tests module.
#[cfg(test)]
mod tests {
	use super::get_process_id;
	#[test]
	fn test_if_process_id_is_returned() {
		assert_ne!(get_process_id(), 0, "There is error in code");
	}
}