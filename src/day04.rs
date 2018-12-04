static INPUT: &str = include_str!("day04.txt");

use regex;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

#[derive(Clone, Debug)]
struct Guard {
    id: u32,
    ckpt: Vec<(u32, bool)>,
}

fn parse(dat: Vec<&str>) -> Vec<Guard> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r#":(\d{2})"#).expect("compile");
        static ref WAKE_RE: Regex = Regex::new(r#"wake"#).unwrap();
        static ref SLEEP_RE: Regex = Regex::new(r#"falls"#).unwrap();
        static ref GUARD_RE: Regex = Regex::new(r#"Guard #([0-9]+)"#).unwrap();
    }
    let mut vec = Vec::with_capacity(10000);
    let mut guard = Guard {
        id: 0,
        ckpt: vec![],
    };
    for line in dat.iter().skip(1) {
        // println!("got {}", line);
        let cap = LINE_RE.captures(line).expect("line");
        let min = (&cap[1]).parse::<u32>().unwrap();
        match GUARD_RE.captures(line) {
            Some(cap) => {
                vec.push(guard);
                guard = Guard {
                    id: (&cap[1]).parse::<u32>().unwrap(),
                    ckpt: vec![],
                };
                continue;
            }
            _ => (),
        }
        if WAKE_RE.is_match(line) {
            guard.ckpt.push((min, false));
            continue;
        }
        if SLEEP_RE.is_match(line) {
            guard.ckpt.push((min, true));
            continue;
        }
    }
    vec
}

fn main() {
    let mut lines = INPUT.split("\n").collect::<Vec<_>>();
    lines.sort();
    let guards = parse(lines);
    // println!("{:#?}", guards);
    // PART ONE
    let mut minutes = [0; 10000];
    for guard in guards.iter() {
        let mut asleep = false;
        let mut fell_asleep_min = 0;
        for ck in guard.ckpt.iter() {
            if asleep && !ck.1 {
                minutes[guard.id as usize] += ck.0 - fell_asleep_min;
            } else if !asleep && ck.1 {
                fell_asleep_min = ck.0;
            }
            asleep = ck.1;
        }
    }
    let most_asleep_id = minutes
        .iter()
        .enumerate()
        .max_by_key(|&(_, item)| item)
        .unwrap()
        .0;
    println!("Most: {}", most_asleep_id);
    let mut occurences = [0; 60];
    guards
        .iter()
        .filter(|g| g.id == most_asleep_id as u32)
        .for_each(|g| {
            let mut asleep = false;
            let mut ck = 0;
            for i in 0..60 {
                if i >= g.ckpt.get(ck).unwrap_or(&(99, false)).0 {
                    asleep = g.ckpt[ck].1;
                    ck += 1;
                }
                if asleep {
                    occurences[i as usize] += 1;
                }
            }
        });
    let most_likely_minute = occurences
        .iter()
        .enumerate()
        .max_by_key(|&(_, item)| item)
        .unwrap()
        .0;
    println!("Occurence_max: {}", most_likely_minute);
    println!("Prod: {}", most_likely_minute * most_asleep_id);

    // PART TWO
    let mut map = std::collections::HashMap::new();

    guards.iter().for_each(|g| {
        let mut asleep = false;
        let mut ck = 0;
        for i in 0..60 {
            if i >= g.ckpt.get(ck).unwrap_or(&(99, false)).0 {
                asleep = g.ckpt[ck].1;
                ck += 1;
            }
            if asleep {
                *map.entry((g.id, i)).or_insert(0) += 1;
            }
        }
    });
    let max_min = map.iter().max_by_key(|&(_, val)| val).unwrap().0;
    println!(
        "{}, {}, * : {}",
        max_min.0,
        max_min.1,
        max_min.0 * max_min.1
    );
}
