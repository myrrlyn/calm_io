/*! Calm, non-panicking I/O operations.

A fun fact about Rust is that `println!` and friends induce a panic when they
try to write to a standard stream that has closed.

A fun fact about UNIX pipelines is that, if the standard streams are a pipe, the
kernel fires a `SIGPIPE` at the process connected to the other end, before
returning `-EPIPE` from the system call. The C runtime `crt0` usually responds
to this signal by terminating. The Rust runtime masks that signal, and does not
terminate.

This means that Rust programs which use `println!` for emitting text, when
placed in a pipeline, can suddenly begin panicking as `stdout` is suddenly
finite.

This crate provides patches to these problems: fallible macros which write to
`stdout` and `stderr` just like `print` and `eprint` do, but return `io::Result`
instead of panicking.

In addition, this crate provides attributes you can place on functions which
return `io::Result` to suppress specific failures, such as pipes breaking. These
macros should only be used on terminal functions, such as `main`, as they
currently throw away the success value.

## Writing to Standand Streams

```sh
println!(...)` becomes `stdoutln!(...)?
print!(...)` becomes `stdout!(...)?
eprintln!(...)` becomes `stderrln!(...)?
eprint!(...)` becomes `stderr!(...)?
```

That’s it. Change your functions that write to those streams to return an error
variant compatible with `io::Result`, and add `?` to taste.

## Suppressing Pipe Failure

Take your function that returns `io::Result`:

```rust,ignore
fn main() -> io::Result<()> {
    stdoutln!("Hello!")?;
}
```

and add one attribute to it:

```rust,ignore
#[calm_io::pipefail] // <- this attribute
fn main() -> io::Result<()> {
    stdoutln!("Hello, but calmly!")?;
}
```

You can observe this by running these three shell commands:

```sh
yes | head
cargo run --example bad_yes | head
cargo run --example good_yes | head
```

and observing that `bad_yes` panics, `yes` exits quietly (unless you inspect
`PIPESTATUS` or use `set -o pipefail`), and `good_yes` always exits quietly.
!*/

pub use calmio_filters::*;

/// Like `print!`, except it returns a `Result` rather than `panic!`king.
#[macro_export]
macro_rules! stdout {
	( $( $t:tt )* ) => {{
		use std::io::Write;
		let stdout = std::io::stdout();
		let mut lock = stdout.lock();
		write!(lock, $( $t )*)
	}}
}

/// Like `println!`, except it returns a `Result` rather than `panic!`king.
#[macro_export]
macro_rules! stdoutln {
	( $( $t:tt )* ) => {{
		use std::io::Write;
		let stdout = std::io::stdout();
		let mut lock = stdout.lock();
		writeln!(lock, $( $t )*)
	}}
}

/// Like `eprint!`, except it returns a `Result` rather than `panic!`king.
#[macro_export]
macro_rules! stderr {
	( $( $t:tt )* ) => {{
		use std::io::Write;
		let stderr = std::io::stderr();
		let mut lock = stdout.lock();
		write!(lock, $( $t )*)
	}}
}

/// Like `eprintln!`, except it returns a `Result` rather than `panic!`king.
#[macro_export]
macro_rules! stderrln {
	( $( $t:tt )* ) => {{
		use std::io::Write;
		let stderr = std::io::stderr();
		let mut lock = stdout.lock();
		writeln!(lock, $( $t )*)
	}}
}
