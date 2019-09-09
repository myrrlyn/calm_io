# Calm I/O

Writing to standard output and error streams is easy in Rust: use one of the
`print!`, `println!` (stdout), `eprint!`, or `eprintln!` (stderr) macros to
format a message and print it directly to the appropriate file descriptor.

But these macros return `()`, not an `io::Result`, even though they perform I/O.
That’s because if the write fails, they panic. Normally, this is fine: the
standard streams should basically always be there.

Here’s an example where they aren’t:

```sh
prints_more_than_ten_lines | head
```

The Unix `head` program reads ten lines (by default) from its standard input,
prints them to its standard output, and then exits. When it exits, it closes its
standard streams.

The `stdout` stream of `prints_more_than_ten_lines` is connected to a kernel
pipe, whose other end is the `stdin` stream of `head`. When `head` exits, it
closes the read end of the pipe. When the kernel processes `close()` calls for
the read ends of pipes, it sends `SIGPIPE` to the process holding the write side
of the pipe (which the C runtime `crt0` catches and terminates, but the Rust
runtime ignores), and then any future `write()` calls to the pipe are
immediately returned with `-EPIPE`.

Rust’s `std::io::Write` function correctly translates this into an `Err`, which
`println!` unwraps, beginning a panic.

The calm I/O crate does not panic in the face of closed streams: it propagates
the error, and allows the caller to gracefully unwind and exit.

This crate exposes four macros: `stdout!`, `stdoutln!`, `stderr!`, and
`stderrln!`. These behave exactly like the macros listed above, except that they
return the `io::Result` from `write!` and `writeln!` rather than unwrapping it
and potentially panicking.

In addition, this crate exports a function attribute, `pipefail`, which
suppresses only the `BrokenPipe` error. It can be attached to any function which
returns `io::Result` (but should only be attached to `main`). Functions
decorated with `#[pipefail]` have a `match` shim wrapped around their body,
which replaces both `Ok(_)` and `Err(io::ErrorKind::BrokenPipe)` with `Ok(())`,
and leaves all other errors as they were.

```rust
use calm_io::*;

#[pipefail]
fn main() -> std::io::Result<()> {
    stdoutln!("Hello stdout from Rust")?;
    stderrln!("Hello stderr from Rust")?;
}
```

As an example, consider this reimplementation of `yes | head`:

```rust
//  examples/yeah.rs
use calm_io::*;

#[pipefail]
fn main () -> std::io::Result<!> {
    let text = std::env::args().nth(1).unwrap_or("y".to_string());
    loop {
        stdoutln!("{}", text)?;
    }
}
```

Try running these commands in your shell!

```sh
$ cargo run --example yeah | head > /dev/null
$ echo "${PIPESTATUS[@]}"
# The name is `PIPESTATUS` in bash, but `pipestatus` (lowercase!) in zsh
0 0
# yeah exits successfully, head exits successfully
$ yes | head > /dev/null
$ echo "${PIPESTATUS[@]}"
141 0
# yes crashes due to SIGPIPE, head exits successfully
```

In the future, other suppression attributes may be added, or a general
suppression attribute may be created that takes a list of errors to suppress.
