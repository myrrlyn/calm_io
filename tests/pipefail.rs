extern crate calm_io;

use std::io;

#[calm_io::pipefail]
fn pipe_machine_broke() -> io::Result<i32> {
	Err(io::Error::new(
		io::ErrorKind::BrokenPipe,
		"understandable, have a nice day",
	))
}

#[calm_io::pipefail]
fn any_other_error() -> io::Result<i32> {
	Err(io::Error::new(
		io::ErrorKind::UnexpectedEof,
		"sudden truncation! oh no!",
	))
}

#[test]
fn broken_pipe_is_not_an_error() {
	assert!(pipe_machine_broke().is_ok());
}

#[test]
fn other_errors_are_still_errors() {
	assert!(any_other_error().is_err());
}
