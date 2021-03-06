extern crate core;
use core::fmt;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::*;
use std::str::FromStr;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

// binary heap for PartialOrd
struct MyBinaryHeap<T> {
    data: Vec<T>,
}
impl<T: Clone> Clone for MyBinaryHeap<T> {
    fn clone(&self) -> Self {
        MyBinaryHeap {
            data: self.data.clone(),
        }
    }
}
impl<T: fmt::Debug> core::fmt::Debug for MyBinaryHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}
impl<T: PartialOrd> MyBinaryHeap<T> {
    pub fn new() -> MyBinaryHeap<T> {
        MyBinaryHeap { data: vec![] }
    }
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        let mut now_index = self.data.len() - 1;
        while now_index > 0 {
            // 0 start
            let parent_index = (now_index - 1) / 2;
            let cmp_result = self.data[now_index].partial_cmp(&self.data[parent_index]);
            if cmp_result.is_none() {
                panic!("compare non comparable values");
            }
            if cmp_result.unwrap() == Less || cmp_result.unwrap() == Equal {
                break;
            }
            self.data.swap(now_index, parent_index);
            now_index = parent_index;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let ret = self.data.swap_remove(0);
        let mut now_index = 0;
        if self.data.is_empty() {
            return Some(ret);
        }
        loop {
            let child_index1 = (now_index + 1) * 2 - 1;
            let child_index2 = (now_index + 1) * 2;
            if child_index1 > self.data.len() - 1 {
                break;
            }
            let change_index = if child_index2 > self.data.len() - 1
                || self.data[child_index1] > self.data[child_index2]
            {
                child_index1
            } else {
                child_index2
            };
            let cmp_result = self.data[now_index].partial_cmp(&self.data[change_index]);
            if cmp_result.is_none() {
                panic!("compare non comparable");
            }
            if cmp_result.unwrap() == Greater || cmp_result.unwrap() == Equal {
                break;
            }
            self.data.swap(now_index, change_index);
            now_index = change_index;
        }
        Some(ret)
    }
}
// slow because when call next, popped every time.
impl<T: PartialOrd> Iterator for MyBinaryHeap<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

#[test]
fn it_works() {
    let mut a = MyBinaryHeap::new();
    assert_eq!(a.pop(), None);
    a.push(1);
    a.push(5);
    a.push(0);
    a.push(100);
    assert_eq!(a.pop(), Some(100));
    assert_eq!(a.pop(), Some(5));
    assert_eq!(a.pop(), Some(1));
    assert_eq!(a.pop(), Some(0));
    assert_eq!(a.pop(), None);
    assert_eq!(a.pop(), None);
}

#[test]
fn iter_works() {
    let mut a = MyBinaryHeap::new();
    assert_eq!(a.pop(), None);
    a.push(1);
    a.push(5);
    a.push(0);
    a.push(100);
    let expected = vec![100, 5, 1, 0];
    for (i, v) in a.enumerate() {
        assert_eq!(expected[i], v);
    }
}
#[test]
#[should_panic]
fn nan_panic() {
    let mut a = MyBinaryHeap::new();
    a.push(1.5);
    a.push(std::f64::NAN);
}
fn main() {
    let mut queue = MyBinaryHeap::new();
    let n: i32 = read();
    let m: i64 = read();
    for _ in 0..n {
        let price: i64 = read();
        queue.push(price);
    }
    for _ in 0..m {
        let price = queue.pop().unwrap();
        // round
        queue.push(price / 2);
    }
    let mut sum = 0;
    for price in queue {
        sum += price;
    }
    println!("{}", sum);
}
