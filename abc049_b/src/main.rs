#![allow(non_snake_case, unused_macros)]
// https://maguro.dev/debug-macro/ $B$+$i(B
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

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
    let h: usize = read();
    let _m: usize = read();
    println!(
        "{}",
        solve((0..h).map(|_| read::<String>().chars().collect()).collect())
            .into_iter()
            .map(|s| s.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(vec: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ans = vec![];
    for v in vec {
        ans.push(v.clone());
        ans.push(v);
    }
    ans
}
