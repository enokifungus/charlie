use std::fs::read_to_string;
use std::error::Error;

fn part2(sc: &Vec<i32>) -> i32 {
    let mut x = 50;
    let mut n = 0;
    for &k in sc {
        if k > 0 || x == 0 {
            n += (x + k.abs())/100;
        } else {
            n += (100 - x - k)/100;
        }
        x = (x + k).rem_euclid(100);
    }
    n
}

fn part1(sc: &Vec<i32>) -> i32 {
    let mut x = 50;
    let mut n = 0;
    for &k in sc {
        x = (x + k).rem_euclid(100);
        if x == 0 { n += 1; }
    }
    n
}

fn src_to_ints(sc: &str) -> Result<Vec<i32>,Box<dyn Error>> {
    sc.split_whitespace().map(|i| {
        let (g,j) = i.split_at(1);
        Ok((if g == "L" {-1} else {1}) * j.parse::<i32>()?)
    }).collect()
}

fn main() -> Result<(),Box<dyn Error>> {
    let sc = src_to_ints(&read_to_string("data")?)?;
    let n1 = part1(&sc);
    println!("Part 1: {}",n1);
    let n2 = part2(&sc);
    println!("Part 2: {}",n2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const IN1:&str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn _1() -> Result<(),Box<dyn Error>> {
        let sc = src_to_ints(IN1)?;
        let n = part1(&sc);
        assert_eq!(n,3);
        Ok(())
    }

    #[test]
    fn _2() -> Result<(),Box<dyn Error>> {
        let sc = src_to_ints(IN1)?;
        let n = part2(&sc);
        assert_eq!(n,6);
        Ok(())
    }
}
