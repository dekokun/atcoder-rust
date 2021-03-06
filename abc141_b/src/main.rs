use std::io::*;
use std::str::FromStr;

#[allow(dead_code)]
fn read_option<T: FromStr>() -> Option<T> {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok()
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let s: String = read();
    for (i, c) in s.chars().enumerate() {
        if (i + 1) % 2 == 0 {
            if c == 'R' {
                println!("No");
                return;
            }
        } else {
            if c == 'L' {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
