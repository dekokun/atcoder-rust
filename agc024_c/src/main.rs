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
    let vec = (0..N).map(|_| read::<usize>()).collect::<Vec<usize>>();
    if vec[0] != 0 {
        println!("-1");
        return;
    }
    for v in vec.windows(2) {
        let before = v[0];
        let now = v[1];
        if now > before + 1 {
            println!("-1");
            return;
        }
    }
    let mut ans = 0;
    let mut must = vec![];
    for &v in vec.iter().rev() {
        if v == 0 {
            continue;
        }
        let mut next_must = vec![];
        use std::collections::HashSet;
        let mut set = HashSet::new();
        must.push(v);
        for &m in &must {
            if m == 0 {
                continue;
            }
            if set.get(&(m - 1)).is_none() {
                next_must.push(m - 1);
                set.insert(m - 1);
                ans += 1;
            }
        }
        must = next_must;
    }
    println!("{}", ans);
}
