#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

fn main() {
    println!("Start calculation");
    let time1 = Instant::now();
    let sequence = calc(1_000_000_000);
    let time2 = Instant::now();
    println!("\tfinished in {:?}s", time2.duration_since(time1.clone()).as_secs());
    println!("end: {:?}", sequence.last().unwrap());
    return;
    print!("Start writing to file");
    let mut file = File::create("C:/Users/info/Desktop/result.csv").unwrap();
    let time3 = Instant::now();
    for number in sequence {
        file.write_all((number.to_string() + "\n").as_bytes()).unwrap();
    }
    let time4 = Instant::now();
    println!("\tfinished in {:?}s", time4.duration_since(time3).as_secs());


    println!("Finished calculation in {:?}", time4.duration_since(time1).as_secs())
}


fn fibonacci(rounds: usize) -> Vec<u128> {
    if rounds >= 185 {
        panic!("Calculating {:?} rounds of fibonacci will lead to a overflow! Max is 184.")
    }
    let mut fib: Vec<u128> = vec![1; 2+rounds];
    for i in 2..(rounds+2) {
       fib[i] = fib[i-2] + fib[i-1];
    }

    fib
}

fn calc(rounds: usize) -> Vec<u128> {
    let mut seq: Vec<u128> = vec![1; 2+rounds];
    for i in 2..(rounds+2) {
        seq[i] = seq[(i as u128-seq[i-1]) as usize] + seq[(i as u128-seq[i-2]) as usize];
    }

    seq
}