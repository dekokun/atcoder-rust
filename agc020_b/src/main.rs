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
    let K: usize = read();
    let mut A: Vec<usize> = (0..K).map(|_| read()).collect();
    A.reverse();
    if *A.first().unwrap() > 2 {
        println!("{}", -1);
        return;
    }
    let mut min = 2;
    let mut max = if A[0] == 1 { 2 } else { 3 };
    for &a in &A[1..] {
        if (min - 1)/ a == max / a {
            println!("-1");
            return
        }
        max += a - (max % a) - 1;
        if min % a != 0 {
            min += a - (min % a);
        }
    }
    println!("{} {}", min, max)
}
