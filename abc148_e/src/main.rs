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
    let N: u64 = read();
    let mut ans: u64 = 0;
    if N % 2 == 1 {
        println!("0");
        return;
    }
    let mut n_tmp = 2;
    loop {
        n_tmp *= 5;
        if n_tmp > N {
            break;
        }
        ans += N / n_tmp;
    }
    println!("{}", ans);
}
