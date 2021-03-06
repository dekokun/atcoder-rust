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
    let _N: usize = read();
    let S: String = read();
    let whiteCount = S.chars().filter(|&c| c == '.').count();
    let (ans, _blackCount, _whiteCount) = S.chars().fold(
        (std::usize::MAX, 0, whiteCount),
        |(ans, leftBlackCount, rightWhiteCount), c| {
            let (leftBlackCount, rightWhiteCount) = if c == '.' {
                (leftBlackCount, rightWhiteCount - 1)
            } else {
                (leftBlackCount, rightWhiteCount)
            };
            let (leftBlackCount, rightWhiteCount) = if c == '#' {
                (leftBlackCount + 1, rightWhiteCount)
            } else {
                (leftBlackCount, rightWhiteCount)
            };
            let ans = std::cmp::min(ans, leftBlackCount + rightWhiteCount);
            (ans, leftBlackCount, rightWhiteCount)
        },
    );
    // 先頭の判定をしていないのでここでする
    let ans = std::cmp::min(ans, whiteCount);
    println!("{}", ans);
}
