#![feature(type_ascription)]
static INPUT: &str = include_str!("day03.txt");

use regex;

#[derive(Copy, Clone, Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}
#[macro_use]
extern crate lazy_static;

use regex::Regex;

fn parse(text: &str) -> Vec<Claim> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
    }
    RE.captures_iter(text)
        .map(|cap| Claim {
            id: cap[1].parse::<u32>().unwrap(),
            x: cap[2].parse::<u32>().unwrap(),
            y: cap[3].parse::<u32>().unwrap(),
            w: cap[4].parse::<u32>().unwrap(),
            h: cap[5].parse::<u32>().unwrap(),
        })
        .collect()
}
fn main() {
    let claims = parse(INPUT);
    // println!("{:?}", claims);
    let mut ct = [[0; 1000]; 1000];
    claims.iter().for_each(|c| {
        for i in c.x..c.x + c.w {
            for j in c.y..c.y + c.h {
                ct[i as usize][j as usize] += 1;
            }
        }
    });

    let mut ct2 = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if ct[i][j] >= 2 {
                ct2 += 1;
            }
        }
    }
    println!("{}", ct2);

    let mut result_id = 0;
    let mut result = || {
        claims.iter().for_each(|c| {
            for i in c.x..c.x + c.w {
                for j in c.y..c.y + c.h {
                    if ct[i as usize][j as usize] >= 2 {
                        return;
                    }
                }
            }
            result_id = c.id;
        })
    };
    result();
    println!("{}", result_id);
}
