use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let p: u16 = read();
    let q: u16 = read();
    let r: u16 = read();
    let mut vec: Vec<u16> = vec![];
    vec.push(p);
    vec.push(q);
    vec.push(r);
    vec.sort();
    println!("{}", vec[0] + vec[1]);
}
