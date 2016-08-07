extern crate rand;
extern crate num;

use std::env;
use std::io;
use num::{Num, NumCast};
use std::io::Write;

const HELP: &'static str = r"Options:
[a]nalyze
[b]rute
[c]ompare
[l]ist";
const RUNS: usize = 1_000_000;
const DICE: usize = 4;
const SIDES: usize = 6;
const SELECT: usize = 3;
const RESULTS_LEN: usize = SIDES * SELECT - SELECT + 1;

fn main() {
    debug_assert!(SELECT < DICE, "YOU BROKE da rulz! DON'T DO THAT!");
    if let Some(s) = env::args().nth(1) {
        match s.chars().next() {
            Some('a') => {
                analyze();
            }
            Some('b') => {
                brute();
            }
            Some('c') => compare(),
            Some('l') => {
                list_analyze();
            }
            _ => println!("unknown option"),
        }
    } else {
        println!("{}", HELP);
    }
}

fn analyze() -> [usize; RESULTS_LEN] {
    println!("analyzing");
    let vkw = SIDES.pow(DICE as u32);
    let mut prob = [0; RESULTS_LEN];
    for res in SELECT..SELECT * SIDES + 1 {
        // let mut pos = Vec::new();
        // TODO generating all possible vecs for result res
        // They have to be unique
        let pos = {
            let mut pos = Vec::new();
            // this generates the 'basic' / simplest roll
            // the highest possible numbers, then only 1s
            let mut scount = DICE;
            let mut i = 0;
            let mut startvec = [1; DICE];
            loop {
                if res >= scount + SIDES - 1 {
                    startvec[i] = SIDES;
                    i += 1;
                    scount += SIDES - 1;
                } else {
                    // FIXME this brakes on SELECT >= DICE
                    startvec[i] = res + SELECT - scount - 1;
                    pos.push(startvec);
                    println!("   /-> {:?}", pos[pos.len() - 1]);
                    break;
                }
            }
            pos
        };
        {
            let mut bases = Vec::<&[usize]>::new();
            bases.push(&pos[0]);
            let mut active = 0;
            let mut permute = move |base, index| {
                println!("{:?} {:?}", base, index);
                if active >= DICE {
                    false
                } else {
                    active += 1;
                    true
                }
            };
            while permute(bases[0], active) {

            }
        }
        // ---
        let mut variations = 0;
        for p in pos {
            // consumes
            variations += permutations(p);
        }
        println!("{:>3}: {:>5}/{} = {:>5.2}%",
                 res,
                 variations,
                 vkw,
                 variations as f64 / vkw as f64);
        prob[res - SELECT] = variations;
    }
    prob
}

fn brute() -> [usize; RESULTS_LEN] {
    println!("brute forcing it up ({})", RUNS);
    let mut rolls = Box::new([0; RUNS]);
    const PERC: usize = RUNS / 100;
    for (i, a_roll) in (1..).zip(&mut rolls[..]) {
        *a_roll = roll::<usize>()[..SELECT].into_iter().fold(0, |sum, x| sum + x);
        if i % PERC == 0 {
            print!("{:>3}% ", i * 100 / RUNS);
            if i % (PERC * 10) == 0 {
                print!("\n");
            }
            io::stdout().flush().expect("couldn't toilet");
        }
    }
    let mut data = [0; RESULTS_LEN];
    for &n in &rolls[..] {
        // consumes
        data[n] += 1;
    }
    println!("results");
    let mut rn = SELECT;
    for &r in &data {
        println!("{:>2}: {:>10}/{} = {:>5.2}%",
                 rn,
                 r,
                 RUNS,
                 r as f64 / RUNS as f64 * 100_f64);
        rn += 1;
    }
    data
}

fn compare() {
    println!("comparing data");
}

fn append_possible(mut head: [usize; DICE], index: usize, result: &mut [usize]) {
    if index < DICE {
        for nd in 0..SIDES {
            head[index] = nd + 1;
            append_possible(head, index + 1, result);
        }
    } else {
        head.sort_by(|a, b| b.cmp(a));
        let r = head.iter().take(SELECT).fold(0, |sum, x| sum + x);
        result[r - SELECT] += 1;
    }
}

fn list_analyze() {
    println!("analyzing list of all possible rolls (cheat)");
    let vkw = SIDES.pow(DICE as u32) as f64;
    let mut result = [0usize; RESULTS_LEN];
    let head = [1usize; DICE];
    append_possible(head, 0, &mut result);
    for (x, &r) in result.iter().enumerate() {
        println!("{:>3}: {:>5}/{:.0} = {:>5.2}%",
                 x + SELECT,
                 r,
                 vkw,
                 r as f64 / vkw * 100f64);
    }
}

fn roll<T: rand::Rand + Num + NumCast + Ord + Copy + Default>() -> [T; DICE] {
    let mut arr: [T; DICE] = [Default::default(); DICE];
    let upper = NumCast::from(SIDES).unwrap();
    for a in &mut arr[..] {
        *a = rand::random::<T>() % upper;
    }
    arr.sort_by(|a, b| b.cmp(a));
    arr
}

fn permutations<T: Copy + Ord + Num + NumCast>(arr: [T; DICE]) -> usize {
    let fact = |x| (1..x + 1).fold(1, |a, b| a * b);
    let mut sames = [0; SIDES];
    for val in arr.into_iter() {
        // consumes
        let i: usize = NumCast::from(*val).expect("couldn't Carsten");
        sames[i - 1] += 1;
    }
    let mut var = fact(DICE);
    for occ in &sames {
        var /= fact(*occ);
    }
    var
}
