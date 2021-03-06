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
    let (N, M): (usize, usize) = (read(), read());
    let (X, Y): (usize, usize) = (read(), read());
    let a_deps: Vec<usize> = (0..N).map(|_| read()).collect();
    let b_deps: Vec<usize> = (0..M).map(|_| read()).collect();
    let mut now = 0;
    let mut ans = 0;
    let depses = vec![(a_deps, X), (b_deps, Y)];
    'outer: loop {
        for (deps, time) in &depses {
            match deps.binary_search(&now) {
                Ok(a) => {
                    now = deps[a];
                }
                Err(a) if a < deps.len() => {
                    now = deps[a];
                }
                Err(_) => {
                    break 'outer;
                }
            }
            now += time;
        }
        ans += 1;
    }
    println!("{}", ans);
}
