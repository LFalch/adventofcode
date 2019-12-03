use std::io::{Result, BufRead, BufReader};
use std::fs::File;

type U = usize;

const ADD: U = 1;
const MUL: U = 2;
const HALT: U = 99;

fn read_file(p: &str) -> Result<Vec<U>> {
    let mut v = Vec::with_capacity(32);

    for bytes in BufReader::new(File::open(p)?).split(b',') {
        let n = String::from_utf8(bytes?).unwrap().parse().unwrap();

        v.push(n);
    }

    Ok(v)
}

fn work(vals: &mut [U]) {
    let mut pointer = 0;
    loop {
        match vals[pointer] {
            ADD => vals[vals[pointer+3]] = vals[vals[pointer+1]] + vals[vals[pointer+2]],
            MUL => vals[vals[pointer+3]] = vals[vals[pointer+1]] * vals[vals[pointer+2]],
            HALT => break,
            _ => unimplemented!()
        }
        pointer += 4;
    }
}

fn calc(noun: U, verb: U) -> U {
    let mut v = read_file("day2.input").unwrap();
    v[1] = noun;
    v[2] = verb;

    work(&mut v);
    v[0]
}

fn main() {
    let mut v = read_file("day2.input").unwrap();
    v[1] = 12;
    v[2] = 2;

    work(&mut v);

    println!("{}", v[0]);

    for noun in 1..=99 {
        for verb in 1..=99 {
            if calc(noun, verb) == 19690720 {
                eprintln!("noun={};verb={}", noun, verb);
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}
