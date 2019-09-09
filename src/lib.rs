extern crate pipefail;

pub use pipefail::pipefail;

#[macro_export]
macro_rules! stdout {
    ( $( $t:tt )* ) => {{
        use std::io::Write;
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        write!(lock, $( $t )*)
    }}
}

#[macro_export]
macro_rules! stdoutln {
    ( $( $t:tt )* ) => {{
        use std::io::Write;
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        writeln!(lock, $( $t )*)
    }}
}

#[macro_export]
macro_rules! stderr {
    ( $( $t:tt )* ) => {{
        use std::io::Write;
        let stderr = std::io::stderr();
        let mut lock = stdout.lock();
        write!(lock, $( $t )*)
    }}
}

#[macro_export]
macro_rules! stderrln {
    ( $( $t:tt )* ) => {{
        use std::io::Write;
        let stderr = std::io::stderr();
        let mut lock = stdout.lock();
        writeln!(lock, $( $t )*)
    }}
}
