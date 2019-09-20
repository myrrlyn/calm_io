/*! Reimplementation of `yes(1)`, that does not die from SIGPIPE`.

A common idiom in UNIX computing is to prepend `yes |` to a pipeline in order to
get interactive scripts to act without user input. The coreutils implementation
of `yes(1)` crashes from SIGPIPE when the pipeline ends.

This program does not.
!*/

use calm_io::*;

#[pipefail]
fn main() -> std::io::Result<()> {
	let mut text = std::env::args()
		.skip(1)
		.collect::<Vec<_>>()
		.join(" ");
	if text.trim().is_empty() {
		text = "y".to_owned();
	}
	loop {
		stdoutln!("{}", text)?;
	}
}
