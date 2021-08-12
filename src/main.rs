// write simple unit test in Rust

use std::process;

fn main() {
    println!("{:?}", get_process_id());
}

fn get_process_id() -> u32 {
	process::id()
}

// add unit test codes
#[test]
fn test_if_process_id_is_returned() {
	assert!(get_process_id() > 0);
}