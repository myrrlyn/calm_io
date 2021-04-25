//! Reimplementation of `yes(1)`, that panics on `SIGPIPE`.

fn main() {
	let mut text = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
	if text.trim().is_empty() {
		text = "y".to_owned();
	}
	loop {
		println!("{}", text);
	}
}
