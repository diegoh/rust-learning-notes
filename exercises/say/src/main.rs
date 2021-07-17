use ferris_says::say;
use std::io::{stdout, BufWriter, Stdout, StdoutLock};

fn main() {
    let stdout: Stdout = stdout();
    let message: String = String::from("Hello, world!");
    let width: usize = message.chars().count();
    let mut writer: BufWriter<StdoutLock> = BufWriter::new(stdout.lock());

    say(message.as_bytes(), width, &mut writer).unwrap();
}
