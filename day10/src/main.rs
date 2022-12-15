use std::{fs, str::FromStr};
fn task1(input: &str) -> i32 {

    let mut sum = 0;
    let mut cycle = 0;
    let mut reg = 1;
    for l in input.lines() {
        if l == "noop" { 
            cycle += 1; 
            if cycle % 40 == 20 {
                sum += cycle * reg;
            }
        }
        else {
            let val: i32 = l.split_once(' ').unwrap().1.parse().unwrap();
            cycle += 1;
            if cycle % 40 == 20 {
                sum += cycle * reg;
            }
            cycle += 1;
            if cycle % 40 == 20 {
                sum += cycle * reg;
            }
            reg += val;

        }
    }
    return sum;
}

fn task2(input: &str) -> String {

    let mut crt = ['.'; 240];
    let mut cycle: usize = 0;
    let mut reg: i32 = 1;
    for l in input.lines() {
        if l == "noop" { 
            if (cycle % 40) as i32 == reg -1 || (cycle % 40) as i32 == reg || (cycle % 40) as i32 == reg +1 {
                crt[cycle] = '#';
            }
            cycle += 1; 
        }
        else {
            let val: i32 = l.split_once(' ').unwrap().1.parse().unwrap();
            if (cycle % 40) as i32 == reg -1 || (cycle % 40) as i32 == reg || (cycle % 40) as i32 == reg +1 {
                crt[cycle] = '#';
            }
            cycle += 1;
            if (cycle % 40) as i32 == reg -1 || (cycle % 40) as i32 == reg || (cycle % 40) as i32 == reg +1 {
                crt[cycle] = '#';
            }
            cycle += 1;
            reg += val;

        }
    }
    let mut s = String::from_str("").unwrap();
    for i in 0..240 {
        if i != 0 && i % 40 == 0 {
            s.push('\n');
        }
        s.push(crt[i]);
    }
    s.push('\n');
    return s;
}

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file!");

    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

#[cfg(test)]
mod test {
    use super::{task1, task2};
    use std::fs;

    #[test]
    fn test1() {
        let test = fs::read_to_string("test").expect("Couldn't read file!");
        assert_eq!(task1(&test), 13140);
    }
    #[test]
    fn test2() {
        let test = fs::read_to_string("test").expect("Couldn't read file!");
        let test_result = fs::read_to_string("test_result").expect("Couldn't read file!");
        assert_eq!(task2(&test), test_result);
    }
}
