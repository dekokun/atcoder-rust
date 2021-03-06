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
    let n: i32 = read();
    let mut vec: Vec<i32> = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    let mut sunuke_balls = vec![0; vec.len()];
    let mut m = 0;
    for (i_1, v) in vec.iter().enumerate().rev() {
        let i = i_1 + 1;
        let mut sum = 0;
        for j in 1..((vec.len() / i) + 1) {
            sum += sunuke_balls[(i * j) - 1];
        }
        if sum % 2 == *v {
            sunuke_balls[i_1] = 0;
        } else {
            sunuke_balls[i_1] = 1;
            m += 1;
        }
    }
    println!("{}", m);
    println!(
        "{}",
        sunuke_balls.iter().enumerate()
            .filter(|&(_, i)| *i == 1)
            .map(|(i, _)| (i + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
