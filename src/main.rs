extern crate rand;

use std::env;
use std::io;
use std::io::Write;

const HELP: &'static str = "Options:
[a]nalyze
[b]rute
[c]ompare";
const RUNS: u64 = 10_000_000;
const DICE: u8 = 4;

fn main() {
    match env::args().nth(1) {
        Some(s) => match s.chars().nth(0) {
            Some('a') => analyze(),
            Some('b') => {brute(); ()},
            Some('c') => compare(),
            _ => println!("unknown option")
        },
        None => println!("{}", HELP)
    };
}

fn analyze() {
    println!("analyzing");
}

fn brute() -> Vec<u32> {
    println!("brute forcing it up ({})", RUNS);
    let mut rolls = Vec::<u16>::new();
    let perc = RUNS / 100;
    for i in 1..RUNS+1 {
        rolls.push(roll(DICE).iter().take(3).fold(0,|sum, x| sum + x) as u16);
        if i % perc == 0 {
            print!("{:>3}% ", i*100/RUNS);
            if i % (perc*10) == 0 {
                print!("\n");
            }
            io::stdout().flush().unwrap();
        }
    }
    let mut data = vec![0u32;16];
    for n in rolls { //consumes
        data[(n-3) as usize] += 1;
    }
    println!("results");
    let mut rn = 3;
    for r in &data {
        println!("{:>2}: {:>10}/{} = {:>5.2}%", rn, r, RUNS, *r as f64 / RUNS as f64 * 100f64);
        rn += 1;
    }
    data
}

fn compare() {
    println!("comparing data");
}

fn roll(len: u8) -> Vec<u8> {
    let mut arr = Vec::<u8>::new();
    for _ in 0..len {
        arr.push(rand::random::<u8>()%6+1);
    }
    arr.sort_by(|a, b| b.cmp(a));
    arr
}
