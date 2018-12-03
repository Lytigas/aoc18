#![feature(type_ascription)]
static INPUT: &str = include_str!("day02.txt");

fn main() {
    let mut ct2 = 0;
    let mut ct3 = 0;
    for id in INPUT.to_owned().split_whitespace() {
        println!("{}", id);
        let mut instances = [0; 256];
        for c in id.chars() {
            instances[c as u8 as usize] += 1;
        }
        for i in 0..instances.len() {
            if instances[i] == 2 {
                ct2 += 1;
                break;
            }
        }
        for i in 0..instances.len() {
            if instances[i] == 3 {
                ct3 += 1;
                break;
            }
        }
    }
    println!("{}", ct2 * ct3);

    let inpstr = INPUT.to_string();
    let ids = inpstr.split_whitespace().collect::<Vec<_>>();

    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            let mut diffct = 0;
            let mut lastk = 0;
            for k in 0..usize::min(ids[i].len(), ids[j].len()) {
                // really should use one iterator rather than one every loop, but eh
                if ids[i].chars().nth(k).unwrap() != ids[j].chars().nth(k).unwrap() {
                    diffct += 1;
                    lastk = k;
                }
            }
            if diffct == 1 {
                let mut s = ids[i].to_string();
                s.remove(lastk);
                println!("{}", s);
                return;
            }
        }
    }
}
