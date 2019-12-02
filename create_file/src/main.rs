use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;

// `% echo s > path`の簡単な実装
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = try!(File::create(path));

    f.write_all(s.as_bytes())
}


fn main() {
    println!("`echo hello > a/b.txt`");
    // 上のmatchは`unwrap_or_else`をメソッドを用いて簡略化できる。
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
