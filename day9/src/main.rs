use std::fs;
use std::collections::HashSet;

fn update_position (prev: (i32, i32), next: (i32, i32)) -> (i32, i32) {

    let mut next = next;
    let prev = prev;
    match (prev.0 - next.0, prev.1 - next.1) {

        (0, 2) => { next.1 += 1; }
        (0, -2) => { next.1 -= 1; }
        (2, 0) => { next.0 += 1; }
        (-2, 0) => { next.0 -= 1; }
        (1, 2) => { 
            next.0 += 1;
            next.1 += 1;
        }
        (-1, 2) => {
            next.0 -= 1;
            next.1 += 1;
        }
        (1, -2) => {
            next.0 += 1;
            next.1 -= 1;
        }
        (-1, -2) => {
            next.0 -= 1;
            next.1 -= 1;
        }
        (2, 1) => {
            next.0 += 1;
            next.1 += 1;
        }
        (2, -1) => {
            next.0 += 1;
            next.1 -= 1;
        }
        (-2, 1) => {
            next.0 -= 1;
            next.1 += 1;
        }
        (-2, -1) => {
            next.0 -= 1;
            next.1 -= 1;
        }
        (2, 2) => {
            next.0 += 1;
            next.1 += 1;
        }
        (2, -2) => {
            next.0 += 1;
            next.1 -= 1;
        }
        (-2, 2) => {
            next.0 -= 1;
            next.1 += 1;
        }
        (-2, -2) => {
            next.0 -= 1;
            next.1 -= 1;
        }
        _ => {}
    }
    return next
}
fn task1(input: &str) -> i32 {


    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut set: HashSet<(i32, i32)> = HashSet::new();

    for l in input.lines() {
        let (cmd, val) = l.split_once(' ').unwrap();

        match cmd {
            "R" => { 
                let v = val.parse::<i32>().unwrap(); 
                for _ in 0..v {
                    head.0 += 1;
                    tail = update_position(head, tail);
                    set.insert(tail);
                }
            }
            "L" => {
                let v = val.parse::<i32>().unwrap(); 
                for _ in 0..v {
                    head.0 -= 1;
                    tail = update_position(head, tail);
                    set.insert(tail);
                }
            }
            "U" => {
                let v = val.parse::<i32>().unwrap(); 
                for _ in 0..v {
                    head.1 += 1;
                    tail =  update_position(head, tail);
                    set.insert(tail);
                }
            }
            "D" => {
                let v = val.parse::<i32>().unwrap(); 
                for _ in 0..v {
                    head.1 -= 1;
                    tail = update_position(head, tail);
                    set.insert(tail);
                }
            }

            _ => { return 1; }
        }

    }



    return set.len() as i32;
}

fn task2(input: &str) -> i32 {


    let mut head = (0, 0);
    let mut knot1 = (0, 0);
    let mut knot2 = (0, 0);
    let mut knot3 = (0, 0);
    let mut knot4 = (0, 0);
    let mut knot5 = (0, 0);
    let mut knot6 = (0, 0);
    let mut knot7 = (0, 0);
    let mut knot8 = (0, 0);
    let mut tail = (0, 0);

    let mut set: HashSet<(i32, i32)> = HashSet::new();

    for l in input.lines() {
        let (cmd, val) = l.split_once(' ').expect("unexpected error!");

        match cmd {
            "R" => { 
                let v = val.parse::<i32>().unwrap(); 
                for _ in 0..v {
                    head.0 += 1;
                    knot1 = update_position(head, knot1);
                    knot2 = update_position(knot1, knot2);
                    knot3 = update_position(knot2, knot3);
                    knot4 = update_position(knot3, knot4);
                    knot5 = update_position(knot4, knot5);
                    knot6 = update_position(knot5, knot6);
                    knot7 = update_position(knot6, knot7);
                    knot8 = update_position(knot7, knot8);
                    tail = update_position(knot8, tail);
                    set.insert(tail);
                }
            }
            "L" => {
                let v = val.parse::<i32>().unwrap(); 
                for _ in 0..v {
                    head.0 -= 1;
                    knot1 = update_position(head, knot1);
                    knot2 = update_position(knot1, knot2);
                    knot3 = update_position(knot2, knot3);
                    knot4 = update_position(knot3, knot4);
                    knot5 = update_position(knot4, knot5);
                    knot6 = update_position(knot5, knot6);
                    knot7 = update_position(knot6, knot7);
                    knot8 = update_position(knot7, knot8);
                    tail = update_position(knot8, tail);
                    set.insert(tail);
                }
            }
            "U" => {
                let v = val.parse::<i32>().unwrap(); 
                for _ in 0..v {
                    head.1 += 1;
                    knot1 = update_position(head, knot1);
                    knot2 = update_position(knot1, knot2);
                    knot3 = update_position(knot2, knot3);
                    knot4 = update_position(knot3, knot4);
                    knot5 = update_position(knot4, knot5);
                    knot6 = update_position(knot5, knot6);
                    knot7 = update_position(knot6, knot7);
                    knot8 = update_position(knot7, knot8);
                    tail = update_position(knot8, tail);
                    set.insert(tail);
                }
            }
            "D" => {
                let v = val.parse::<i32>().unwrap(); 
                for _ in 0..v {
                    head.1 -= 1;
                    knot1 = update_position(head, knot1);
                    knot2 = update_position(knot1, knot2);
                    knot3 = update_position(knot2, knot3);
                    knot4 = update_position(knot3, knot4);
                    knot5 = update_position(knot4, knot5);
                    knot6 = update_position(knot5, knot6);
                    knot7 = update_position(knot6, knot7);
                    knot8 = update_position(knot7, knot8);
                    tail = update_position(knot8, tail);
                    set.insert(tail);
                }
            }
            _ => {}
        }

    }



    return set.len() as i32;
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
        assert_eq!(task1(&test), 13);
    }
    #[test]
    fn test2() {
        let test2 = fs::read_to_string("test2").expect("Couldn't read file!");
        assert_eq!(task2(&test2), 36);
    }
}
