extern crate rand;

use std::env;
use std::io;
use std::io::Write;

const HELP: &'static str = "Options:
[a]nalyze
[b]rute
[c]ompare
[l]ist";
const RUNS: u64 = 1_000_000;
const DICE: u8 = 4;
const SIDES: u8 = 6;
const SELECT: u8 = 3;

fn main() {
    match env::args().nth(1) {
        Some(s) => match s.chars().nth(0) {
            Some('a') => {analyze(); ()},
            Some('b') => {brute(); ()},
            Some('c') => compare(),
            Some('l') => {list_analyze(); ()},
            _ => println!("unknown option")
        },
        None => println!("{}", HELP)
    };
}

fn analyze() -> Vec<u32>{
    println!("analyzing");
    let vkw = (SIDES as u32).pow(DICE as u32);
    let mut prob = vec![0u32;((SIDES * SELECT) - SELECT + 1) as usize];
    for res in SELECT..SELECT*SIDES+1 {
        // println!("{0:>2} -> ???/{1} | {0:>2}! = 10^{2:.2}", res, vkw, (fact(res as u64) as f64).log10());
        let mut pos = Vec::<Vec<u8>>::new();
        // TODO generating all possible vecs for result res
        // They have to be unique
        {
            // this generates the 'basic' / simplest roll
            // the highest possible numbers, then only 1s
            let mut scount = DICE;
            let mut i = 0usize;
            let mut startvec = vec![1; DICE as usize];
            loop {
                if res >= scount + SIDES - 1{
                    startvec[i] = SIDES;
                    i += 1;
                    scount += SIDES - 1;
                } else {
                    // FIXME this brakes on SELECT >= DICE
                    startvec[i] = res + SELECT - scount - 1;
                    pos.push(startvec);
                    println!("   /-> {:?}", pos[pos.len()-1]);
                    break;
                }
            }
        }
        {
            let mut bases = Vec::<&[u8]>::new();
            bases.push(pos.get(0).unwrap() as &[u8]);
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
        for p in pos { // consumes
            variations += permutations(p);
        }
        println!("{:>3}: {:>5}/{} = {:>5.2}%", res, variations, vkw, variations as f64 / vkw as f64);
        prob[(res - SELECT) as usize] = variations;
    }
    prob
}

fn brute() -> Vec<u32> {
    println!("brute forcing it up ({})", RUNS);
    let mut rolls = Vec::<u16>::new();
    let perc = RUNS / 100;
    for i in 1..RUNS+1 {
        rolls.push(roll(DICE).iter().take(SELECT as usize).fold(0,|sum, x| sum + x) as u16);
        if i % perc == 0 {
            print!("{:>3}% ", i*100/RUNS);
            if i % (perc*10) == 0 {
                print!("\n");
            }
            io::stdout().flush().unwrap();
        }
    }
    let mut data = vec![0u32; ((SIDES * SELECT) - SELECT + 1) as usize];
    for n in rolls { //consumes
        data[(n as u8-SELECT) as usize] += 1;
    }
    println!("results");
    let mut rn = SELECT;
    for r in &data {
        println!("{:>2}: {:>10}/{} = {:>5.2}%", rn, r, RUNS, *r as f64 / RUNS as f64 * 100f64);
        rn += 1;
    }
    data
}

fn compare() {
    println!("comparing data");
}

fn append_possible(mut head: [u8; DICE as usize], index: usize, result: &mut [u32]) {
    if index < DICE as usize {
        for nd in 0..SIDES {
            head[index] = nd+1;
            append_possible(head, index+1, result);
        }
    } else {
        head.sort_by(|a, b| b.cmp(a));
        let r = head.iter().take(SELECT as usize).fold(0, |sum, x| sum + x) as usize;
        result[r-SELECT as usize] += 1;
    }
}

fn list_analyze() {
    println!("analyzing list of all possible rolls (cheat)");
    let vkw = (SIDES as u32).pow(DICE as u32) as f64;
    let mut result = [0 as u32; (SIDES*SELECT-SELECT+1) as usize];
    let head = [1u8; DICE as usize];
    append_possible(head, 0, &mut result);
    for x in 0..result.len() {
        println!("{:>3}: {:>5}/{:.0} = {:>5.2}%", x+SELECT as usize, result[x], vkw, result[x] as f64 / vkw * 100f64);
    }
}

fn roll(len: u8) -> Vec<u8> {
    let mut arr = Vec::<u8>::new();
    for _ in 0..len {
        arr.push(rand::random::<u8>()%SIDES+1);
    }
    arr.sort_by(|a, b| b.cmp(a));
    arr
}

fn permutations(arr: Vec<u8>) -> u32 {
    let fact = |x| {
        (1..x+1).fold(1, |a, b| a * b) as u32
    };
    let mut sames = [0; SIDES as usize];
    for val in arr { // consumes
        sames[val as usize -1] += 1;
    }
    let mut var = fact(DICE);
    for occ in &sames {
        var /= fact(*occ);
    }
    var
}
