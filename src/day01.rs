#![feature(type_ascription)]
static INPUT: &str = include_str!("day01.txt");

fn main() {
    println!(
        "{}",
        INPUT
            .to_owned()
            .split_whitespace()
            .map(|line| line.parse::<i32>().unwrap())
            .sum(): i32
    );

    const LEN: usize = 1000000;
    let mut seen = [false; LEN];
    for freq in INPUT
        .to_owned()
        .split_whitespace()
        .map(|line| line.parse::<i32>().unwrap())
        .cycle()
        .scan(0, |acc, int| {
            *acc += int;
            Some(*acc)
        })
    {
        const OFFSET: i32 = (LEN / 2) as i32;
        // print!("{}, ", freq);
        if seen[(freq + OFFSET) as usize] {
            println!("{}", freq);
            break;
        }
        seen[(freq + OFFSET) as usize] = true;
    }
    // let _ = seen.iter().map(|x| print!("{:?}, ", x)).collect::<Vec<_>>();
}
