use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::cmp::Ordering;

fn secret_entrance(start: i32, fname: &str) -> i32 {
    let f = File::open(fname).unwrap();
    let reader = BufReader::new(f);
    let mut position= start;
    let mut zeros = if position == 0 {
        1i32
    } else {
        0i32
    };

    for l in reader.lines() {
        if let Ok(lstr) = l {
            let lstr = lstr.to_uppercase();
            if lstr.as_str().starts_with('L') {
                let num = lstr[1..].parse().unwrap_or(0);
                position -= num;
                while position <= -100 {
                    position += 100;
                }
                match position.cmp(&0) {
                    Ordering::Less => position += 100,
                    Ordering::Equal => zeros += 1,
                    Ordering::Greater => (),
                }
            } else if lstr.as_str().starts_with('R') {
                let num = lstr[1..].parse().unwrap_or(0);
                position += num;
                while position >= 100 {
                    position -= 100;
                }
                match position.cmp(&0) {
                    Ordering::Less => (),
                    Ordering::Equal => zeros += 1,
                    Ordering::Greater => position -= 100,
                }
            }
        } else {
            break;
        }
    }
    zeros
}

fn secret_entrance2(start: i32, fname: &str) -> i32 {
    let f = File::open(fname).unwrap();
    let reader = BufReader::new(f);
    let mut position= start;
    let mut zeros = if position == 0 {
        1i32
    } else {
        0i32
    };

    for l in reader.lines() {
        if let Ok(lstr) = l {
            let lstr = lstr.to_uppercase();
            if lstr.as_str().starts_with('L') {
                let num = lstr[1..].parse().unwrap_or(0);
                let num = if num >= 100 {
                        let n : i32 = num / 100;
                        zeros += n;
                        num % 100
                    } else {
                        num
                    };
                println!("L num = {num}");
                position -= num;
                println!("position={position}");
                //while position <= -100 {
                //    position += 100;
                //    zeros += 1; // every time passing 0 we add 1 to the counter
                //    println!("position={position}");
                //}
                match position.cmp(&0) {
                    Ordering::Less => { position += 100 },
                    Ordering::Equal => zeros += 1,
                    Ordering::Greater => (),
                }
                println!("zeros = {zeros}");
            } else if lstr.as_str().starts_with('R') {
                let num = lstr[1..].parse().unwrap_or(0);
                println!("R num = {num}");
                position += num;
                println!("position={position}");
                //while position >= 100 {
                //    position -= 100;
                //    zeros += 1; // every time passing 0 we add 1 to the counter
                //    println!("position={position}");
                //}
                match position.cmp(&0) {
                    Ordering::Less => (),
                    Ordering::Equal => zeros += 1,
                    Ordering::Greater => { position -= 100; zeros += 1 },
                }
                println!("zeros = {zeros}");
            }
        } else {
            break;
        }
    }
    zeros
}
#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn day01_secret_entrance() {
    //    let res = secret_entrance(50, "data/day01b.txt");
    //    println!("result is {res}");   // run tests with --nocapture
    //    assert!(true);
    //}
    #[test]
    fn day01_secret_entrance2() {
        let res = secret_entrance2(50, "data/day01.txt");
        println!("result is {res}");   // run tests with --nocapture
        assert!(true);
    }
}
