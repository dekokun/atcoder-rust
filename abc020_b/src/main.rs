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

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, true);
    }
}

fn main() {
    println!("{}", solve(read(), read()));
}

fn solve(a: usize, b: usize) -> usize {
    let mut tmp = 10;
    while b / tmp != 0 {
        tmp *= 10;
    }
    (a * tmp + b) * 2
}
