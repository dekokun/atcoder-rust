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
    let N: usize = read();
    let M: usize = read();
    if M % N == 0 {
        println!("{}", M / N);
        return;
    }
    let mut factors = vec![];
    for i in 1..((M as f64).sqrt().ceil() as usize) {
        if M % i == 0 {
            factors.push(i);
            factors.push(M / i);
        }
    }
    let mut ans = 0;
    for v in factors {
        if v <= M / N {
            ans = std::cmp::max(ans, v);
        }
    }
    println!("{}", ans)
}
