#![allow(non_snake_case)]

use std::io::{stdin, Read};
use std::str::FromStr;
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
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let k: usize = read();
    let s: usize = read();
    let mut ans = 0;
    for i in 0..=k {
        for j in 0..=k {
            if i + j > s{
                break;
            }
            if s - (i + j) > k {
                continue;
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}
